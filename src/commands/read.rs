use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn pe_read(address: &str, index: u8) -> Result<(), Box<dyn std::error::Error>> {
    let raw_address = format!("{}/raw", address.trim_end_matches('/'));
    let (mut ws, _) = connect_async(&raw_address).await?;

    let msg = midi_controller::property_exchange::build_get_inquiry(
        [0x10, 0x20, 0x30, 0x40], // CLI MUID
        [0x01, 0x02, 0x03, 0x04], // device MUID
        0x01,                     // request_id
        index,
    );
    ws.send(Message::Binary(msg.to_vec())).await?;

    match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
        Ok(Some(Ok(Message::Binary(data)))) => {
            // Check reply status
            let status = midi_controller::property_exchange::extract_reply_status(&data);
            if let Some(s) = status {
                if !s.is_ok() {
                    use midi_controller::property_exchange::PeStatus;
                    match s {
                        PeStatus::NotFound => println!("Preset {}: not found", index),
                        PeStatus::FormatError => {
                            println!("Preset {}: format error (re-upload setlist)", index)
                        }
                        PeStatus::VersionMismatch => println!(
                            "Preset {}: version mismatch (re-upload setlist after firmware update)",
                            index
                        ),
                        _ => println!("Preset {}: error ({:?})", index, s),
                    }
                    return Ok(());
                }
            }
            // Extract body from PE Get Reply
            if let Some(body) = midi_controller::property_exchange::extract_get_body(&data) {
                let mut decoded_buf = [0u8; 256];
                let dec_len =
                    midi_controller::property_exchange::decode_mcoded7(body, &mut decoded_buf);
                let body = &decoded_buf[..dec_len];
                if body.is_empty() {
                    println!("Preset {}: not found", index);
                } else {
                    match postcard::from_bytes::<midi_controller::config::Preset>(body) {
                        Ok(preset) => {
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
