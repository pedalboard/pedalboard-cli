use clap::{Parser, Subcommand};
use futures_util::{SinkExt, StreamExt};
use std::path::PathBuf;
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod protocol;
use pedalboard_cli::config::{yaml_to_presets, Setlist, BUTTON_KEYS, ENCODER_KEYS};
use protocol::opendeck_set_messages;

#[derive(Parser)]
#[command(name = "pedalboard-cli", about = "Pedalboard configuration tool", version = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH")))]
struct Cli {
    /// WebSocket address of the bridge
    #[arg(short, long, default_value = "ws://cm5-dev.home/config")]
    address: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Upload a setlist configuration file (legacy OpenDeck SysEx)
    Upload { file: PathBuf },
    /// Factory reset the device
    Reset,
    /// Reboot the device (no data loss)
    Reboot,
    /// Enter UF2 bootloader (for firmware flashing)
    Bootloader,
    /// Upload config via MIDI-CI Property Exchange (direct model)
    PeUpload { file: PathBuf },
    /// Read back a preset from the device via PE
    PeRead { index: u8 },
    /// Monitor MIDI output from the device in real-time
    Monitor,
    /// Flash a UF2 firmware file to the device (enters bootloader, uploads via bridge)
    Flash { file: PathBuf },
}

const BUTTON_HW_OFFSET: u8 = 2; // A=2, B=3, ..., F=7 (0-1 are encoder buttons)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upload { file } => {
            let content = std::fs::read_to_string(&file)?;
            let setlist: Setlist = serde_yaml::from_str(&content)?;
            upload(&cli.address, &setlist).await?;
        }
        Commands::Reset => {
            reset(&cli.address).await?;
        }
        Commands::Reboot => {
            reboot(&cli.address).await?;
        }
        Commands::Bootloader => {
            bootloader(&cli.address).await?;
        }
        Commands::PeUpload { file } => {
            pe_upload(&cli.address, &file).await?;
        }
        Commands::PeRead { index } => {
            pe_read(&cli.address, index).await?;
        }
        Commands::Monitor => {
            monitor(&cli.address).await?;
        }
        Commands::Flash { file } => {
            flash(&cli.address, &file).await?;
        }
    }

    Ok(())
}

async fn send_sysex(
    ws: &mut (impl SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
              + StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
              + Unpin),
    msg: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    ws.send(Message::Binary(msg.to_vec())).await?;
    // Wait for ACK with timeout
    let _ = tokio::time::timeout(std::time::Duration::from_millis(100), ws.next()).await;
    Ok(())
}

async fn upload(address: &str, setlist: &Setlist) -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws, _) = connect_async(address).await?;

    // Handshake — must wait for ACK before sending config
    ws.send(Message::Binary(vec![
        0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0xF7,
    ]))
    .await?;
    match tokio::time::timeout(std::time::Duration::from_secs(5), ws.next()).await {
        Ok(Some(Ok(_))) => println!("Connected."),
        _ => {
            eprintln!("Handshake failed — no response from device");
            return Ok(());
        }
    }

    for (preset_idx, preset) in setlist.presets.iter().enumerate() {
        if preset_idx > 0 {
            println!(
                "  Preset {}: \"{}\" (skipped — OpenDeck only supports slot 0, use PE for multi-preset)",
                preset_idx + 1,
                preset.name
            );
            continue;
        }
        println!("  Preset {}: \"{}\"", preset_idx + 1, preset.name);

        // Set button MIDI config
        for (key_idx, key) in BUTTON_KEYS.iter().enumerate() {
            if let Some(btn) = preset.buttons.get(*key) {
                let hw_idx = key_idx as u8 + BUTTON_HW_OFFSET;
                for msg in opendeck_set_messages::button(preset_idx as u8, hw_idx, btn) {
                    send_sysex(&mut ws, &msg).await?;
                }
                print!("    {}: {} ", key, btn.label);
                if let Some(n) = btn.note {
                    print!("(Note {})", n);
                }
                if let Some(c) = btn.cc {
                    print!("(CC {})", c);
                }
                println!();
            }
        }

        // Set encoder MIDI config
        for (key_idx, key) in ENCODER_KEYS.iter().enumerate() {
            if let Some(enc) = preset.encoders.get(*key) {
                for msg in opendeck_set_messages::encoder(preset_idx as u8, key_idx as u8, enc) {
                    send_sysex(&mut ws, &msg).await?;
                }
                println!("    {}: {} (CC {:?})", key, enc.label, enc.cc);
            }
        }
    }

    println!("Upload complete.");

    // Close SysEx session so device returns to normal MIDI operation
    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x00, 0xF7]).await?;

    Ok(())
}

async fn reset(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws, _) = connect_async(address).await?;

    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0xF7]).await?;
    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x44, 0xF7]).await?;
    println!("Factory reset sent. Device will reboot.");
    Ok(())
}

async fn reboot(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws, _) = connect_async(address).await?;

    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0xF7]).await?;
    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x7F, 0xF7]).await?;
    println!("Reboot sent.");
    Ok(())
}

async fn bootloader(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws, _) = connect_async(address).await?;

    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0xF7]).await?;
    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x55, 0xF7]).await?;
    println!("Bootloader entry sent.");
    Ok(())
}

async fn pe_read(address: &str, index: u8) -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws, _) = connect_async(address).await?;

    let msg = pedalboard_protocol::property_exchange::build_get_inquiry(
        [0x10, 0x20, 0x30, 0x40], // CLI MUID
        [0x01, 0x02, 0x03, 0x04], // device MUID
        0x01,                     // request_id
        index,
    );
    ws.send(Message::Binary(msg.to_vec())).await?;

    match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
        Ok(Some(Ok(Message::Binary(data)))) => {
            // Extract body from PE Get Reply (same layout as Set, different sub-ID2)
            if let Some(body) = pedalboard_protocol::property_exchange::extract_get_body(&data) {
                let mut decoded_buf = [0u8; 256];
                let dec_len =
                    pedalboard_protocol::property_exchange::decode_mcoded7(body, &mut decoded_buf);
                let body = &decoded_buf[..dec_len];
                if body.is_empty() {
                    println!("Preset {}: not found", index);
                } else {
                    match postcard::from_bytes::<pedalboard_protocol::config::Preset>(body) {
                        Ok(preset) => {
                            println!("Preset {}: \"{}\"", index, preset.name);
                            for (i, btn) in preset.buttons.iter().enumerate() {
                                println!("  Button {}: \"{}\"", i, btn.label);
                            }
                            for (i, enc) in preset.encoders.iter().enumerate() {
                                println!("  Encoder {}: \"{}\"", i, enc.label);
                            }
                        }
                        Err(e) => println!("Preset {}: deserialize error: {}", index, e),
                    }
                }
            } else {
                println!("Preset {}: invalid reply", index);
            }
        }
        Ok(Some(Ok(_))) => println!("Preset {}: unexpected reply type", index),
        _ => println!("Preset {}: no reply (timeout)", index),
    }

    Ok(())
}

async fn pe_upload(address: &str, file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file)?;
    let setlist: Setlist = serde_yaml::from_str(&content)?;
    let presets = yaml_to_presets(&setlist);

    println!(
        "Uploading {} presets via Property Exchange...",
        presets.len()
    );

    let (mut ws, _) = connect_async(address).await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Upload global config if present
    if let Some(ref global_yaml) = setlist.global {
        use pedalboard_cli::config::yaml_global_to_protocol;
        let gc = yaml_global_to_protocol(global_yaml);
        let serialized = postcard::to_allocvec(&gc)?;
        let msg = pedalboard_protocol::property_exchange::build_set_inquiry(
            [0x10, 0x20, 0x30, 0x40],
            [0x01, 0x02, 0x03, 0x04],
            0x7F, // request_id (must be 7-bit safe)
            pedalboard_protocol::config::GLOBAL_CONFIG_RESOURCE,
            &serialized,
        );
        println!("  Global config ({} bytes)", serialized.len());
        ws.send(Message::Binary(msg.to_vec())).await?;
        match tokio::time::timeout(std::time::Duration::from_secs(5), ws.next()).await {
            Ok(Some(Ok(_))) => println!("    ACK ✓"),
            _ => eprintln!("    No reply (timeout)"),
        }
    }

    for (idx, preset) in presets.iter().enumerate() {
        let serialized = postcard::to_allocvec(preset)?;
        let msg = pedalboard_protocol::property_exchange::build_set_inquiry(
            [0x10, 0x20, 0x30, 0x40], // CLI MUID
            [0x01, 0x02, 0x03, 0x04], // device MUID
            idx as u8 + 1,            // request_id
            idx as u8,                // resource = preset index
            &serialized,
        );

        println!(
            "  Preset {}: \"{}\" ({} bytes)",
            idx,
            preset.name,
            serialized.len()
        );
        ws.send(Message::Binary(msg.to_vec())).await?;

        match tokio::time::timeout(std::time::Duration::from_secs(5), ws.next()).await {
            Ok(Some(Ok(_))) => println!("    ACK ✓"),
            _ => eprintln!("    No reply (timeout)"),
        }

        // Brief delay between presets to avoid overflowing firmware persist channel
        if presets.len() > 4 {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        }
    }

    println!("Upload complete.");
    Ok(())
}

async fn flash(address: &str, file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read(file)?;
    println!("Firmware: {} ({} bytes)", file.display(), data.len());

    // 1. Enter bootloader
    println!("Entering bootloader...");
    let (mut ws, _) = connect_async(address).await?;
    ws.send(Message::Binary(vec![
        0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0xF7,
    ]))
    .await?;
    let _ = tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await;
    ws.send(Message::Binary(vec![
        0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x55, 0xF7,
    ]))
    .await?;
    drop(ws);

    // 2. Upload UF2 via HTTP POST to bridge /flash (bridge waits for drive internally)
    println!("Uploading firmware to bridge...");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let base_url = address
        .replace("ws://", "http://")
        .replace("/config", "/flash");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let form = reqwest::multipart::Form::new().part(
        "firmware",
        reqwest::multipart::Part::bytes(data).file_name("firmware.uf2"),
    );
    let resp = client.post(&base_url).multipart(form).send().await?;
    if resp.status().is_success() {
        let body = resp.text().await?;
        println!("{}", body.trim());
        println!("Device will reboot. Bridge will auto-reconnect.");
    } else {
        let status = resp.status();
        let body = resp.text().await?;
        eprintln!("Flash failed ({}): {}", status, body.trim());
    }
    Ok(())
}

async fn monitor(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Replace /config with /monitor in the address
    let monitor_addr = address.replace("/config", "/monitor");
    let (mut ws, _) = connect_async(&monitor_addr).await?;
    println!("Monitoring MIDI output (Ctrl+C to stop)...");

    while let Some(msg) = ws.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                parse_and_print(&data);
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                eprintln!("Connection error: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_and_print(data: &[u8]) {
    let mut i = 0;
    while i < data.len() {
        let status = data[i];
        // SysEx: consume until 0xF7
        if status == 0xF0 {
            let end = data[i..]
                .iter()
                .position(|&b| b == 0xF7)
                .unwrap_or(data.len() - i);
            print_midi(&data[i..i + end + 1]);
            i += end + 1;
            continue;
        }
        // System real-time (1 byte)
        if status >= 0xF8 {
            print_midi(&data[i..i + 1]);
            i += 1;
            continue;
        }
        // Channel messages
        let len = match status & 0xF0 {
            0xC0 | 0xD0 => 2, // PC, Channel Pressure
            _ => 3,           // Note, CC, Bend, etc.
        };
        let end = (i + len).min(data.len());
        print_midi(&data[i..end]);
        i = end;
    }
}

fn print_midi(data: &[u8]) {
    if data.is_empty() {
        return;
    }
    let status = data[0];

    // SysEx
    if status == 0xF0 {
        println!("  SysEx     {} bytes", data.len());
        return;
    }

    // System real-time
    match status {
        0xF8 => {
            println!("  Clock");
            return;
        }
        0xFA => {
            println!("  Start");
            return;
        }
        0xFB => {
            println!("  Continue");
            return;
        }
        0xFC => {
            println!("  Stop");
            return;
        }
        0xFE => {
            return;
        } // Active Sensing — suppress
        0xFF => {
            println!("  Reset");
            return;
        }
        _ => {}
    }

    let channel = (status & 0x0F) + 1;
    let msg_type = status & 0xF0;

    match msg_type {
        0x90 if data.len() >= 3 => {
            if data[2] == 0 {
                println!("  Note Off ch={} note={} vel=0", channel, data[1]);
            } else {
                println!("  Note On  ch={} note={} vel={}", channel, data[1], data[2]);
            }
        }
        0x80 if data.len() >= 3 => {
            println!("  Note Off ch={} note={} vel={}", channel, data[1], data[2]);
        }
        0xB0 if data.len() >= 3 => {
            println!("  CC       ch={} cc={} val={}", channel, data[1], data[2]);
        }
        0xC0 if data.len() >= 2 => {
            println!("  PC       ch={} prog={}", channel, data[1]);
        }
        0xD0 if data.len() >= 2 => {
            println!("  ChPress  ch={} val={}", channel, data[1]);
        }
        0xE0 if data.len() >= 3 => {
            let bend = ((data[2] as u16) << 7) | data[1] as u16;
            println!("  Bend     ch={} val={}", channel, bend as i16 - 8192);
        }
        _ => {
            println!(
                "  Raw      {}",
                data.iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    }
}
