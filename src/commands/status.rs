use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::send_system_command;

pub async fn device_status(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let raw_address = format!("{}/raw", address.trim_end_matches('/'));
    let (mut ws, _) = connect_async(&raw_address).await?;

    let msg = midi_controller::property_exchange::build_get_inquiry(
        [0x10, 0x20, 0x30, 0x40],
        [0x01, 0x02, 0x03, 0x04],
        0x01,
        midi_controller::config::DEVICE_INFO_RESOURCE,
    );
    ws.send(Message::Binary(msg.to_vec())).await?;

    match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
        Ok(Some(Ok(Message::Binary(data)))) => {
            let status = midi_controller::property_exchange::extract_reply_status(&data);
            if let Some(s) = status {
                if !s.is_ok() {
                    eprintln!("Device returned error: {:?}", s);
                    return Ok(());
                }
            }
            if let Some(body) = midi_controller::property_exchange::extract_get_body(&data) {
                let mut decoded_buf = [0u8; 64];
                let dec_len =
                    midi_controller::property_exchange::decode_mcoded7(body, &mut decoded_buf);
                let body = &decoded_buf[..dec_len];
                match postcard::from_bytes::<midi_controller::config::DeviceInfo>(body) {
                    Ok(info) => {
                        println!("Device Status:");
                        println!("  Firmware version:     {}", info.version);
                        println!("  Flash format version: {}", info.flash_format);
                        println!("  Presets loaded:       {}", info.presets_loaded);
                        if info.presets_skipped > 0 {
                            println!(
                                "  Presets skipped:      {} (version mismatch — re-upload setlist)",
                                info.presets_skipped
                            );
                        } else {
                            println!("  Presets skipped:      0");
                        }
                    }
                    Err(e) => eprintln!("Failed to decode device info: {}", e),
                }
            } else {
                eprintln!("No body in device info reply");
            }
        }
        Ok(Some(Ok(_))) => eprintln!("Unexpected reply type"),
        _ => eprintln!("No reply (device may not support status query)"),
    }

    Ok(())
}

pub async fn reset(address: &str, wait: bool) -> Result<(), Box<dyn std::error::Error>> {
    send_system_command(
        address,
        midi_controller::config::SystemCommand::FactoryReset,
    )
    .await?;
    println!("Factory reset sent. Device will reboot.");
    if wait {
        println!("Waiting for device...");
        super::wait_for_device(address, 15).await?;
        println!("Device ready.");
    }
    Ok(())
}

pub async fn reboot(address: &str, wait: bool) -> Result<(), Box<dyn std::error::Error>> {
    send_system_command(address, midi_controller::config::SystemCommand::Reboot).await?;
    println!("Reboot sent.");
    if wait {
        println!("Waiting for device...");
        super::wait_for_device(address, 15).await?;
        println!("Device ready.");
    }
    Ok(())
}

pub async fn bootloader(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_system_command(address, midi_controller::config::SystemCommand::Bootloader).await?;
    println!("Bootloader entry sent.");
    Ok(())
}
