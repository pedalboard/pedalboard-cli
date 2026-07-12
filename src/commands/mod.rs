pub mod flash;
pub mod mode;
pub mod monitor;
pub mod read;
pub mod status;
pub mod upload;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

async fn send_system_command(
    address: &str,
    cmd: midi_controller::config::SystemCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw_address = format!("{}/raw", address.trim_end_matches('/'));
    let (mut ws, _) = connect_async(&raw_address).await?;
    let msg = midi_controller::property_exchange::build_set_inquiry(
        [0x10, 0x20, 0x30, 0x40],
        [0x01, 0x02, 0x03, 0x04],
        0x70, // request_id for system commands
        midi_controller::config::SYSTEM_COMMAND_RESOURCE,
        &[cmd as u8],
    );
    ws.send(Message::Binary(msg.to_vec())).await?;
    // Wait for ACK (firmware delays 1s before reboot to allow USB flush)
    let _ = tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await;
    Ok(())
}

/// Poll device until it responds to a status query, or timeout.
/// First waits for the device to go offline (disconnect), then waits for it to come back.
/// Includes a brief settle delay after first response to allow async preset loading.
pub async fn wait_for_device(
    address: &str,
    timeout_secs: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);
    let raw_address = format!("{}/raw", address.trim_end_matches('/'));

    // Phase 1: Wait for device to go offline (confirms reboot started)
    loop {
        if tokio::time::Instant::now() >= deadline {
            // Device never went offline — maybe it rebooted very fast, proceed to phase 2
            break;
        }

        let result: Result<(), Box<dyn std::error::Error>> = async {
            let (mut ws, _) = connect_async(&raw_address).await?;
            let msg = midi_controller::property_exchange::build_get_inquiry(
                [0x10, 0x20, 0x30, 0x40],
                [0x01, 0x02, 0x03, 0x04],
                0x01,
                midi_controller::config::DEVICE_INFO_RESOURCE,
            );
            ws.send(Message::Binary(msg.to_vec())).await?;
            match tokio::time::timeout(std::time::Duration::from_millis(500), ws.next()).await {
                Ok(Some(Ok(Message::Binary(_)))) => Ok(()),
                _ => Err("No reply".into()),
            }
        }
        .await;

        if result.is_err() {
            // Device went offline — proceed to phase 2
            break;
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }

    // Phase 2: Wait for device to come back online
    loop {
        if tokio::time::Instant::now() >= deadline {
            return Err("Timeout waiting for device to become ready".into());
        }

        let result: Result<(), Box<dyn std::error::Error>> = async {
            let (mut ws, _) = connect_async(&raw_address).await?;
            let msg = midi_controller::property_exchange::build_get_inquiry(
                [0x10, 0x20, 0x30, 0x40],
                [0x01, 0x02, 0x03, 0x04],
                0x01,
                midi_controller::config::DEVICE_INFO_RESOURCE,
            );
            ws.send(Message::Binary(msg.to_vec())).await?;
            match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
                Ok(Some(Ok(Message::Binary(_)))) => Ok(()),
                _ => Err("No reply".into()),
            }
        }
        .await;

        if result.is_ok() {
            // Device responded — wait briefly for async preset loading to complete
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            return Ok(());
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}
