use clap::{Parser, Subcommand};
use futures_util::{SinkExt, StreamExt};
use pedalboard_cli::config::{yaml_to_presets, Setlist};
use std::path::PathBuf;
use tokio_tungstenite::{connect_async, tungstenite::Message};

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
    /// Factory reset the device
    Reset,
    /// Reboot the device (no data loss)
    Reboot,
    /// Enter UF2 bootloader (for firmware flashing)
    Bootloader,
    /// Upload config via MIDI-CI Property Exchange
    Upload { file: PathBuf },
    /// Read back a preset from the device
    Read { index: u8 },
    /// Monitor MIDI output from the device in real-time
    Monitor,
    /// Flash a UF2 firmware file to the device (enters bootloader, uploads via bridge)
    Flash { file: PathBuf },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Reset => {
            reset(&cli.address).await?;
        }
        Commands::Reboot => {
            reboot(&cli.address).await?;
        }
        Commands::Bootloader => {
            bootloader(&cli.address).await?;
        }
        Commands::Upload { file } => {
            pe_upload(&cli.address, &file).await?;
        }
        Commands::Read { index } => {
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

async fn reset(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_system_command(
        address,
        pedalboard_protocol::config::SystemCommand::FactoryReset,
    )
    .await?;
    println!("Factory reset sent. Device will reboot.");
    Ok(())
}

async fn reboot(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_system_command(address, pedalboard_protocol::config::SystemCommand::Reboot).await?;
    println!("Reboot sent.");
    Ok(())
}

async fn bootloader(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_system_command(
        address,
        pedalboard_protocol::config::SystemCommand::Bootloader,
    )
    .await?;
    println!("Bootloader entry sent.");
    Ok(())
}

async fn send_system_command(
    address: &str,
    cmd: pedalboard_protocol::config::SystemCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw_address = address.replace("/config", "/raw");
    let (mut ws, _) = connect_async(&raw_address).await?;
    let msg = pedalboard_protocol::property_exchange::build_set_inquiry(
        [0x10, 0x20, 0x30, 0x40],
        [0x01, 0x02, 0x03, 0x04],
        0x70, // request_id for system commands
        pedalboard_protocol::config::SYSTEM_COMMAND_RESOURCE,
        &[cmd as u8],
    );
    ws.send(Message::Binary(msg.to_vec())).await?;
    // Don't wait for ACK — device may reboot before replying
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
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

    // 1. Enter bootloader via PE system command
    println!("Entering bootloader...");
    let raw_address = address.replace("/config", "/raw");
    let (mut ws, _) = connect_async(&raw_address).await?;
    let msg = pedalboard_protocol::property_exchange::build_set_inquiry(
        [0x10, 0x20, 0x30, 0x40],
        [0x01, 0x02, 0x03, 0x04],
        0x70,
        pedalboard_protocol::config::SYSTEM_COMMAND_RESOURCE,
        &[pedalboard_protocol::config::SystemCommand::Bootloader as u8],
    );
    ws.send(Message::Binary(msg.to_vec())).await?;
    drop(ws);

    // 2. Upload UF2 via HTTP POST to bridge /flash (bridge waits for drive internally)
    println!("Uploading firmware to bridge...");
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

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
