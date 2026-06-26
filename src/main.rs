use clap::{Parser, Subcommand};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::path::PathBuf;
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod protocol;
use protocol::{label_set_messages, opendeck_set_messages};

#[derive(Parser)]
#[command(name = "pedalboard-cli", about = "Pedalboard configuration tool")]
struct Cli {
    /// WebSocket address of the bridge
    #[arg(short, long, default_value = "ws://cm5-dev.home/config")]
    address: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Upload a setlist configuration file
    Upload { file: PathBuf },
    /// Factory reset the device
    Reset,
}

#[derive(Deserialize)]
struct Setlist {
    presets: Vec<PresetConfig>,
}

#[derive(Deserialize)]
struct PresetConfig {
    name: String,
    #[serde(default)]
    buttons: std::collections::HashMap<String, ButtonConfig>,
    #[serde(default)]
    encoders: std::collections::HashMap<String, EncoderConfig>,
}

#[derive(Deserialize)]
struct ButtonConfig {
    label: String,
    #[serde(default)]
    note: Option<u8>,
    #[serde(default)]
    cc: Option<u8>,
    #[serde(default)]
    toggle: Option<bool>,
    #[serde(default)]
    level: Option<bool>,
    #[serde(default)]
    color: Option<String>,
}

#[derive(Deserialize)]
struct EncoderConfig {
    label: String,
    #[serde(default)]
    cc: Option<u16>,
}

const BUTTON_KEYS: &[&str] = &["A", "B", "C", "D", "E", "F"];
const ENCODER_KEYS: &[&str] = &["Vol", "Gain"];

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
    }

    Ok(())
}

async fn send_sysex(
    ws: &mut (impl SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
          + StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
          + Unpin),
    msg: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    ws.send(Message::Binary(msg.to_vec().into())).await?;
    // Wait for ACK with timeout
    let _ = tokio::time::timeout(std::time::Duration::from_millis(100), ws.next()).await;
    Ok(())
}

async fn upload(address: &str, setlist: &Setlist) -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws, _) = connect_async(address).await?;

    // Handshake
    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0xF7]).await?;
    println!("Connected.");

    for (preset_idx, preset) in setlist.presets.iter().enumerate() {
        println!("  Preset {}: \"{}\"", preset_idx + 1, preset.name);

        // Set preset name label
        for msg in label_set_messages::preset_name(preset_idx as u8, &preset.name) {
            send_sysex(&mut ws, &msg).await?;
        }

        // Set button labels and MIDI config
        for (key_idx, key) in BUTTON_KEYS.iter().enumerate() {
            if let Some(btn) = preset.buttons.get(*key) {
                // Label
                for msg in label_set_messages::button(preset_idx as u8, key_idx as u8, &btn.label) {
                    send_sysex(&mut ws, &msg).await?;
                }
                // MIDI config
                for msg in opendeck_set_messages::button(preset_idx as u8, key_idx as u8, btn) {
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

        // Set encoder labels and MIDI config
        for (key_idx, key) in ENCODER_KEYS.iter().enumerate() {
            if let Some(enc) = preset.encoders.get(*key) {
                for msg in label_set_messages::encoder(preset_idx as u8, key_idx as u8, &enc.label)
                {
                    send_sysex(&mut ws, &msg).await?;
                }
                for msg in opendeck_set_messages::encoder(preset_idx as u8, key_idx as u8, enc) {
                    send_sysex(&mut ws, &msg).await?;
                }
                println!("    {}: {} (CC {:?})", key, enc.label, enc.cc);
            }
        }
    }

    println!("Upload complete.");
    Ok(())
}

async fn reset(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws, _) = connect_async(address).await?;

    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0xF7]).await?;
    send_sysex(&mut ws, &[0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x44, 0xF7]).await?;
    println!("Factory reset sent. Device will reboot.");
    Ok(())
}
