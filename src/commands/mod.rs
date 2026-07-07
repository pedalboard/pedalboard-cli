pub mod flash;
pub mod mode;
pub mod monitor;
pub mod read;
pub mod status;
pub mod upload;

use futures_util::SinkExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};

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
