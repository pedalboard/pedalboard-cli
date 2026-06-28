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
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

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
