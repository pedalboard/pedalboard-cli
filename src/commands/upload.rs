use std::path::PathBuf;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::config::{yaml_global_to_protocol, yaml_to_presets, Setlist, SCHEMA_VERSION};

pub async fn pe_upload(address: &str, file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let raw_address = format!("{}/raw", address.trim_end_matches('/'));
    let content = std::fs::read_to_string(file)?;
    let setlist: Setlist = serde_yaml::from_str(&content)?;

    if setlist.version > SCHEMA_VERSION {
        eprintln!(
            "Error: setlist requires schema version {}, but this CLI supports up to version {}.",
            setlist.version, SCHEMA_VERSION
        );
        eprintln!("Upgrade pedalboard-cli to use this setlist file.");
        std::process::exit(1);
    }

    let presets = yaml_to_presets(&setlist);

    println!(
        "Uploading {} presets via Property Exchange...",
        presets.len()
    );

    let (mut ws, _) = connect_async(&raw_address).await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Upload global config if present
    if let Some(ref global_yaml) = setlist.global {
        let gc = yaml_global_to_protocol(global_yaml);
        let serialized = postcard::to_allocvec(&gc)?;
        let msg = midi_controller::property_exchange::build_set_inquiry(
            [0x10, 0x20, 0x30, 0x40],
            [0x01, 0x02, 0x03, 0x04],
            0x7F, // request_id (must be 7-bit safe)
            midi_controller::config::GLOBAL_CONFIG_RESOURCE,
            &serialized,
        );
        println!("  Global config ({} bytes)", serialized.len());
        ws.send(Message::Binary(msg.to_vec())).await?;
        match tokio::time::timeout(std::time::Duration::from_secs(5), ws.next()).await {
            Ok(Some(Ok(msg))) => {
                let data = msg.into_data();
                match midi_controller::property_exchange::extract_reply_status(&data) {
                    Some(midi_controller::property_exchange::PeStatus::Ok) => {
                        println!("    ACK ✓")
                    }
                    Some(status) => eprintln!("    Error: {:?}", status),
                    None => println!("    ACK ✓"),
                }
            }
            _ => eprintln!("    No reply (timeout)"),
        }
    }

    for (idx, preset) in presets.iter().enumerate() {
        let serialized = postcard::to_allocvec(preset)?;
        let msg = midi_controller::property_exchange::build_set_inquiry(
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
            Ok(Some(Ok(msg))) => {
                let data = msg.into_data();
                match midi_controller::property_exchange::extract_reply_status(&data) {
                    Some(midi_controller::property_exchange::PeStatus::Ok) => {
                        println!("    ACK ✓")
                    }
                    Some(status) => eprintln!("    Error: {:?}", status),
                    None => println!("    ACK ✓"),
                }
            }
            _ => eprintln!("    No reply (timeout)"),
        }

        // Brief delay between presets to avoid overflowing firmware persist channel
        if presets.len() > 4 {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        }
    }

    // Clear any stale presets beyond the uploaded count (send empty body = delete)
    let max_presets: u8 = 32;
    let uploaded_count = presets.len() as u8;
    if uploaded_count < max_presets {
        let mut cleared = 0u8;
        for idx in uploaded_count..max_presets {
            let msg = midi_controller::property_exchange::build_set_inquiry(
                [0x10, 0x20, 0x30, 0x40],
                [0x01, 0x02, 0x03, 0x04],
                idx + 1,
                idx,
                &[], // empty body = delete
            );
            ws.send(Message::Binary(msg.to_vec())).await?;
            cleared += 1;
        }
        // Drain all ACKs with a single bulk timeout (not per-message)
        for _ in 0..cleared {
            if tokio::time::timeout(std::time::Duration::from_millis(100), ws.next())
                .await
                .is_err()
            {
                break; // stop draining if no more replies
            }
        }
        if cleared > 0 {
            println!("  Cleared {} stale preset slot(s).", cleared);
            // Allow firmware persist queue to drain
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    println!("Upload complete.");
    Ok(())
}
