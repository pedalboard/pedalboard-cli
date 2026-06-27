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
    /// Upload a setlist configuration file (legacy OpenDeck SysEx)
    Upload { file: PathBuf },
    /// Factory reset the device
    Reset,
    /// Upload config via MIDI-CI Property Exchange (direct model)
    PeUpload { file: PathBuf },
    /// PoC: send a raw string via PE
    PeTest { label: String },
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
    program_change: Option<u8>,
    #[serde(default)]
    value: Option<u8>,
    #[serde(default)]
    toggle: Option<bool>,
    #[serde(default)]
    level: Option<bool>,
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    channel: Option<u8>,
}

#[derive(Deserialize)]
struct EncoderConfig {
    label: String,
    #[serde(default)]
    cc: Option<u16>,
    #[serde(default)]
    channel: Option<u8>,
}

const BUTTON_KEYS: &[&str] = &["A", "B", "C", "D", "E", "F"];
const BUTTON_HW_OFFSET: u8 = 2; // A=2, B=3, ..., F=7 (0-1 are encoder buttons)
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
        Commands::PeUpload { file } => {
            pe_upload(&cli.address, &file).await?;
        }
        Commands::PeTest { label } => {
            pe_test(&cli.address, &label).await?;
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

    // Handshake — must wait for ACK before sending config
    ws.send(Message::Binary(vec![0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0xF7].into())).await?;
    match tokio::time::timeout(std::time::Duration::from_secs(5), ws.next()).await {
        Ok(Some(Ok(_))) => println!("Connected."),
        _ => {
            eprintln!("Handshake failed — no response from device");
            return Ok(());
        }
    }

    for (preset_idx, preset) in setlist.presets.iter().enumerate() {
        println!("  Preset {}: \"{}\"", preset_idx + 1, preset.name);

        // Set preset name label
        for msg in label_set_messages::preset_name(preset_idx as u8, &preset.name) {
            send_sysex(&mut ws, &msg).await?;
        }

        // Set button labels and MIDI config
        for (key_idx, key) in BUTTON_KEYS.iter().enumerate() {
            if let Some(btn) = preset.buttons.get(*key) {
                let hw_idx = key_idx as u8 + BUTTON_HW_OFFSET;
                // Label
                for msg in label_set_messages::button(preset_idx as u8, key_idx as u8, &btn.label) {
                    send_sysex(&mut ws, &msg).await?;
                }
                // MIDI config
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

async fn pe_test(address: &str, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws, _) = connect_async(address).await?;

    let msg = pedalboard_protocol::property_exchange::build_set_inquiry(
        [0x10, 0x20, 0x30, 0x40], // CLI MUID
        [0x01, 0x02, 0x03, 0x04], // device MUID
        0x01,                      // request_id
        0x00,                      // resource (preset 0)
        label.as_bytes(),
    );

    println!("Sending PE Set Property: {:02X?}", msg.as_slice());
    ws.send(Message::Binary(msg.to_vec().into())).await?;

    match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
        Ok(Some(Ok(resp))) => println!("Reply: {:02X?}", resp.into_data()),
        Ok(Some(Err(e))) => eprintln!("Error: {}", e),
        _ => eprintln!("No reply (timeout)"),
    }

    Ok(())
}

fn yaml_to_presets(setlist: &Setlist) -> Vec<pedalboard_protocol::config::Preset> {
    use pedalboard_protocol::config as pc;

    setlist
        .presets
        .iter()
        .map(|p| {
            let mut buttons = heapless::Vec::new();
            for key in BUTTON_KEYS {
                let btn_cfg = if let Some(btn) = p.buttons.get(*key) {
                    let mut on_press: heapless::Vec<pc::Action, { pc::MAX_ACTIONS }> =
                        heapless::Vec::new();

                    if let Some(prog) = btn.program_change {
                        let _ = on_press.push(pc::Action::ProgramChange {
                            program: prog,
                            channel: btn.channel.unwrap_or(1),
                        });
                    } else if let Some(cc) = btn.cc {
                        if btn.toggle == Some(true) {
                            let _ = on_press.push(pc::Action::CcToggle {
                                cc: cc as u8,
                                value_a: btn.value.unwrap_or(127),
                                value_b: 0,
                                channel: btn.channel.unwrap_or(1),
                            });
                        } else {
                            let _ = on_press.push(pc::Action::Cc {
                                cc: cc as u8,
                                value: btn.value.unwrap_or(127),
                                channel: btn.channel.unwrap_or(1),
                            });
                        }
                    } else if let Some(note) = btn.note {
                        let _ = on_press.push(pc::Action::NoteOn {
                            note,
                            channel: btn.channel.unwrap_or(1),
                        });
                    }

                    let color = match btn.color.as_deref() {
                        Some("red") => pc::Color::Red,
                        Some("green") => pc::Color::Green,
                        Some("blue") => pc::Color::Blue,
                        Some("yellow") => pc::Color::Yellow,
                        Some("cyan") => pc::Color::Cyan,
                        Some("magenta") => pc::Color::Magenta,
                        Some("white") => pc::Color::White,
                        Some("orange") => pc::Color::Orange,
                        Some("purple") => pc::Color::Purple,
                        _ => pc::Color::Off,
                    };

                    let mode = if btn.toggle == Some(true) {
                        pc::ButtonMode::Toggle
                    } else {
                        pc::ButtonMode::Momentary
                    };

                    pc::ButtonConfig {
                        label: pc::Label::try_from(btn.label.as_str()).unwrap_or_default(),
                        color: pc::LedConfig {
                            on: color,
                            off: pc::Color::Off,
                        },
                        mode,
                        on_press,
                        on_release: heapless::Vec::new(),
                        on_long_press: heapless::Vec::new(),
                    }
                } else {
                    pc::ButtonConfig {
                        label: pc::Label::new(),
                        color: pc::LedConfig::default(),
                        mode: pc::ButtonMode::default(),
                        on_press: heapless::Vec::new(),
                        on_release: heapless::Vec::new(),
                        on_long_press: heapless::Vec::new(),
                    }
                };
                let _ = buttons.push(btn_cfg);
            }

            let mut encoders = heapless::Vec::new();
            for key in ENCODER_KEYS {
                let enc_cfg = if let Some(enc) = p.encoders.get(*key) {
                    pc::EncoderConfig {
                        label: pc::Label::try_from(enc.label.as_str()).unwrap_or_default(),
                        action: pc::EncoderAction::Cc {
                            cc: enc.cc.unwrap_or(0),
                            channel: enc.channel.unwrap_or(1),
                            min: 0,
                            max: 127,
                        },
                    }
                } else {
                    pc::EncoderConfig {
                        label: pc::Label::new(),
                        action: pc::EncoderAction::Cc {
                            cc: 0,
                            channel: 1,
                            min: 0,
                            max: 127,
                        },
                    }
                };
                let _ = encoders.push(enc_cfg);
            }

            pc::Preset {
                name: pc::Label::try_from(p.name.as_str()).unwrap_or_default(),
                buttons,
                encoders,
                analog: heapless::Vec::new(),
            }
        })
        .collect()
}

async fn pe_upload(address: &str, file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file)?;
    let setlist: Setlist = serde_yaml::from_str(&content)?;
    let presets = yaml_to_presets(&setlist);

    println!("Uploading {} presets via Property Exchange...", presets.len());

    let (mut ws, _) = connect_async(address).await?;

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
        ws.send(Message::Binary(msg.to_vec().into())).await?;

        match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
            Ok(Some(Ok(_))) => println!("    ACK ✓"),
            _ => eprintln!("    No reply (timeout)"),
        }
    }

    println!("Upload complete.");
    Ok(())
}
