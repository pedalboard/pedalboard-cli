use clap::{Parser, Subcommand};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::path::PathBuf;
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod protocol;
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
    /// Upload config via MIDI-CI Property Exchange (direct model)
    PeUpload { file: PathBuf },
    /// Read back a preset from the device via PE
    PeRead { index: u8 },
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
    #[serde(default)]
    analog: std::collections::HashMap<String, AnalogYamlConfig>,
}

#[derive(Deserialize)]
struct AnalogYamlConfig {
    label: String,
    cc: u8,
    #[serde(default)]
    channel: Option<u8>,
    #[serde(default)]
    min: Option<u8>,
    #[serde(default)]
    max: Option<u8>,
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
    radio_group: Option<u8>,
    #[serde(default)]
    level: Option<bool>,
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    channel: Option<u8>,
    #[serde(default)]
    on_long_press: Option<String>,
    #[serde(default)]
    values: Option<Vec<u8>>,
    #[serde(default)]
    reverse: Option<bool>,
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
                        if btn.values.is_some() {
                            let _ = on_press.push(pc::Action::CcCycle {
                                cc,
                                channel: btn.channel.unwrap_or(1),
                                reverse: btn.reverse.unwrap_or(false),
                            });
                        } else if btn.toggle == Some(true) {
                            let _ = on_press.push(pc::Action::CcToggle {
                                cc,
                                value_a: btn.value.unwrap_or(127),
                                value_b: 0,
                                channel: btn.channel.unwrap_or(1),
                            });
                        } else {
                            let _ = on_press.push(pc::Action::Cc {
                                cc,
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
                        Some("off") => pc::Color::Off,
                        Some(hex) if hex.starts_with('#') && hex.len() == 7 => {
                            let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
                            let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
                            let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
                            pc::Color::Custom(r, g, b)
                        }
                        _ => pc::Color::Off,
                    };

                    let mode = if btn.toggle == Some(true) {
                        pc::ButtonMode::Toggle
                    } else if let Some(group) = btn.radio_group {
                        pc::ButtonMode::RadioGroup(group)
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
                        on_long_press: {
                            let mut lp = heapless::Vec::new();
                            match btn.on_long_press.as_deref() {
                                Some("next_preset") => {
                                    lp.push(pc::Action::PresetNext).ok();
                                }
                                Some("prev_preset") => {
                                    lp.push(pc::Action::PresetPrev).ok();
                                }
                                _ => {}
                            }
                            lp
                        },
                        cycle_values: {
                            let mut cv = heapless::Vec::new();
                            if let Some(vals) = &btn.values {
                                for &v in vals.iter().take(pc::MAX_CYCLE_VALUES) {
                                    cv.push(v).ok();
                                }
                            }
                            cv
                        },
                    }
                } else {
                    pc::ButtonConfig {
                        label: pc::Label::new(),
                        color: pc::LedConfig::default(),
                        mode: pc::ButtonMode::default(),
                        on_press: heapless::Vec::new(),
                        on_release: heapless::Vec::new(),
                        on_long_press: heapless::Vec::new(),
                        cycle_values: heapless::Vec::new(),
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

            const ANALOG_KEYS: &[&str] = &["Exp2", "Exp1"];
            let mut analog = heapless::Vec::new();
            for key in ANALOG_KEYS {
                if let Some(a) = p.analog.get(*key) {
                    let _ = analog.push(pc::AnalogConfig {
                        label: pc::Label::try_from(a.label.as_str()).unwrap_or_default(),
                        cc: a.cc,
                        channel: a.channel.unwrap_or(1),
                        min: a.min.unwrap_or(0),
                        max: a.max.unwrap_or(127),
                    });
                } else if !analog.is_empty() || p.analog.values().any(|_| true) {
                    // Push placeholder to preserve index alignment
                    let _ = analog.push(pc::AnalogConfig {
                        label: pc::Label::new(),
                        cc: 0,
                        channel: 0, // channel 0 = disabled (won't send MIDI)
                        min: 0,
                        max: 0,
                    });
                }
            }

            pc::Preset {
                name: pc::Label::try_from(p.name.as_str()).unwrap_or_default(),
                buttons,
                encoders,
                analog,
            }
        })
        .collect()
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
    }

    println!("Upload complete.");
    Ok(())
}
