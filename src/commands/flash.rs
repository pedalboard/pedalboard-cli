use std::path::PathBuf;

use futures_util::SinkExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn flash(address: &str, file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read(file)?;
    println!("Firmware: {} ({} bytes)", file.display(), data.len());

    // 1. Enter bootloader via PE system command
    println!("Entering bootloader...");
    let raw_address = address.replace("/config", "/raw");
    let (mut ws, _) = connect_async(&raw_address).await?;
    let msg = pedalboard_protocol::property_exchange::build_set_inquiry(
        [0x10, 0x20, 0x30, 0x40],
        [0x01, 0x02, 0x03, 0x04],
        0x70,
        pedalboard_protocol::config::SYSTEM_COMMAND_RESOURCE,
        &[pedalboard_protocol::config::SystemCommand::Bootloader as u8],
    );
    ws.send(Message::Binary(msg.to_vec())).await?;
    drop(ws);

    // 2. Upload UF2 via HTTP POST to bridge /flash (bridge waits for drive internally)
    println!("Uploading firmware to bridge...");
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    let base_url = address
        .replace("ws://", "http://")
        .replace("/config", "/flash");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let form = reqwest::multipart::Form::new().part(
        "firmware",
        reqwest::multipart::Part::bytes(data).file_name("firmware.uf2"),
    );
    let resp = client.post(&base_url).multipart(form).send().await?;
    if resp.status().is_success() {
        let body = resp.text().await?;
        println!("{}", body.trim());
        println!("Device will reboot. Bridge will auto-reconnect.");
    } else {
        let status = resp.status();
        let body = resp.text().await?;
        eprintln!("Flash failed ({}): {}", status, body.trim());
    }
    Ok(())
}
