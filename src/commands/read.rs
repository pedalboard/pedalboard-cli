use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// Read a single preset from the device via PE Get. Returns the deserialized preset or None.
async fn read_preset(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    index: u8,
) -> Result<Option<midi_controller::config::Preset>, Box<dyn std::error::Error>> {
    let msg = midi_controller::property_exchange::build_get_inquiry(
        [0x10, 0x20, 0x30, 0x40],
        [0x01, 0x02, 0x03, 0x04],
        0x01,
        index,
    );
    ws.send(Message::Binary(msg.to_vec())).await?;

    match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
        Ok(Some(Ok(Message::Binary(data)))) => {
            let status = midi_controller::property_exchange::extract_reply_status(&data);
            if let Some(s) = status {
                if !s.is_ok() {
                    use midi_controller::property_exchange::PeStatus;
                    match s {
                        PeStatus::NotFound => {}
                        PeStatus::FormatError => {
                            eprintln!("Preset {}: format error (re-upload setlist)", index);
                        }
                        PeStatus::VersionMismatch => {
                            eprintln!(
                                "Preset {}: version mismatch (re-upload after firmware update)",
                                index
                            );
                        }
                        _ => eprintln!("Preset {}: error ({:?})", index, s),
                    }
                    return Ok(None);
                }
            }
            if let Some(body) = midi_controller::property_exchange::extract_get_body(&data) {
                let mut decoded_buf = [0u8; 256];
                let dec_len =
                    midi_controller::property_exchange::decode_mcoded7(body, &mut decoded_buf);
                let body = &decoded_buf[..dec_len];
                if body.is_empty() {
                    return Ok(None);
                }
                match postcard::from_bytes::<midi_controller::config::Preset>(body) {
                    Ok(preset) => Ok(Some(preset)),
                    Err(e) => {
                        eprintln!("Preset {}: deserialize error: {}", index, e);
                        Ok(None)
                    }
                }
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

/// Read global config from the device via PE Get.
async fn read_global(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Result<Option<midi_controller::config::GlobalConfig>, Box<dyn std::error::Error>> {
    let msg = midi_controller::property_exchange::build_get_inquiry(
        [0x10, 0x20, 0x30, 0x40],
        [0x01, 0x02, 0x03, 0x04],
        0x01,
        midi_controller::config::GLOBAL_CONFIG_RESOURCE,
    );
    ws.send(Message::Binary(msg.to_vec())).await?;

    match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
        Ok(Some(Ok(Message::Binary(data)))) => {
            let status = midi_controller::property_exchange::extract_reply_status(&data);
            if let Some(s) = status {
                if !s.is_ok() {
                    return Ok(None);
                }
            }
            if let Some(body) = midi_controller::property_exchange::extract_get_body(&data) {
                let mut decoded_buf = [0u8; 256];
                let dec_len =
                    midi_controller::property_exchange::decode_mcoded7(body, &mut decoded_buf);
                let body = &decoded_buf[..dec_len];
                if body.is_empty() {
                    return Ok(None);
                }
                match postcard::from_bytes::<midi_controller::config::GlobalConfig>(body) {
                    Ok(gc) => Ok(Some(gc)),
                    Err(_) => Ok(None),
                }
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

/// Print a human-readable summary of a preset.
fn print_preset_summary(index: u8, preset: &midi_controller::config::Preset) {
    println!("Preset {}: \"{}\"", index, preset.name);
    for (i, btn) in preset.buttons.iter().enumerate() {
        println!("  Button {}: \"{}\"", i, btn.label);
    }
    for (i, enc) in preset.encoders.iter().enumerate() {
        println!("  Encoder {}: \"{}\"", i, enc.label);
    }
    for (i, analog) in preset.analog.iter().enumerate() {
        println!(
            "  Analog {}: \"{}\" (CC {}, ch {})",
            i, analog.label, analog.cc, analog.channel
        );
    }
}

pub async fn pe_read(
    address: &str,
    index: Option<u8>,
    yaml: bool,
    all: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw_address = format!("{}/raw", address.trim_end_matches('/'));
    let (mut ws, _) = connect_async(&raw_address).await?;

    if all || (yaml && index.is_none()) {
        // Read all presets + global config
        let mut presets = Vec::new();
        for i in 0..32u8 {
            match read_preset(&mut ws, i).await? {
                Some(preset) if !preset.name.is_empty() => presets.push(preset),
                Some(_) => break, // empty name = end of presets
                None => break,
            }
        }

        if yaml {
            let global = read_global(&mut ws).await?;
            let yaml_str = pedalboard_config::presets_to_yaml(&presets, global.as_ref())
                .map_err(|e| format!("YAML serialization error: {}", e))?;
            print!("{}", yaml_str);
        } else {
            for (i, preset) in presets.iter().enumerate() {
                print_preset_summary(i as u8, preset);
            }
        }
    } else {
        let idx = index.unwrap_or(0);
        match read_preset(&mut ws, idx).await? {
            Some(preset) => {
                if yaml {
                    let yaml_str = pedalboard_config::presets_to_yaml(&[preset], None)
                        .map_err(|e| format!("YAML serialization error: {}", e))?;
                    print!("{}", yaml_str);
                } else {
                    print_preset_summary(idx, &preset);
                }
            }
            None => println!("Preset {}: not found", idx),
        }
    }

    Ok(())
}
