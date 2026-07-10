use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::send_system_command;

pub async fn device_status(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("System Status:");
    println!(
        "  CLI:        {}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );

    // Query bridge /status (HTTP).
    let bridge_url = address
        .replace("ws://", "http://")
        .replace("wss://", "https://")
        .trim_end_matches('/')
        .to_string()
        + "/";

    let bridge_status = query_bridge_status(&bridge_url).await;
    match &bridge_status {
        Ok(info) => {
            println!("  Bridge:     {}", info.version);
            println!("  Mode:       {}", info.mode);
            let midi_state = if info.midi_connected {
                let mut parts = Vec::new();
                if info.midi_input.is_some() {
                    parts.push("in");
                }
                if info.midi_output.is_some() {
                    parts.push("out");
                }
                format!("connected ({})", parts.join(" + "))
            } else {
                "disconnected".to_string()
            };
            println!("  MIDI:       {}", midi_state);
            let modhost_state = if info.modhost_connected {
                "connected"
            } else {
                "disconnected"
            };
            println!("  mod-host:   {}", modhost_state);
            if let Some(ref audio) = info.audio {
                let snapshot = audio.active_snapshot.as_deref().unwrap_or("none");
                println!(
                    "  Audio:      {} plugins, {} snapshots (active: {})",
                    audio.plugins, audio.snapshots, snapshot
                );
            } else {
                println!("  Audio:      disabled");
            }
        }
        Err(e) => {
            println!("  Bridge:     offline ({})", e);
        }
    }

    // Query firmware device info (PE via WebSocket).
    let raw_address = format!("{}/raw", address.trim_end_matches('/'));
    match query_firmware_status(&raw_address).await {
        Ok(info) => {
            println!("  Firmware:   {}", info.version);
            println!("  Flash fmt:  {}", info.flash_format);
            println!("  Presets:    {} loaded", info.presets_loaded);
            if info.presets_skipped > 0 {
                println!(
                    "              {} skipped (version mismatch — re-upload setlist)",
                    info.presets_skipped
                );
            }
        }
        Err(e) => {
            println!("  Firmware:   offline ({})", e);
        }
    }

    Ok(())
}

struct BridgeInfo {
    version: String,
    mode: String,
    midi_connected: bool,
    midi_input: Option<String>,
    midi_output: Option<String>,
    modhost_connected: bool,
    audio: Option<AudioInfo>,
}

struct AudioInfo {
    active_snapshot: Option<String>,
    snapshots: usize,
    plugins: usize,
}

async fn query_bridge_status(url: &str) -> Result<BridgeInfo, String> {
    let resp = reqwest::get(url).await.map_err(|e| format!("{e}"))?;

    let json: serde_json::Value = resp.json().await.map_err(|e| format!("parse: {e}"))?;

    let version = json["version"].as_str().unwrap_or("unknown").to_string();
    let mode = json["mode"].as_str().unwrap_or("unknown").to_string();
    let midi_connected = json["midi"]["connected"].as_bool().unwrap_or(false);
    let midi_input = json["midi"]["input"].as_str().map(|s| s.to_string());
    let midi_output = json["midi"]["output"].as_str().map(|s| s.to_string());
    let modhost_connected = json["modhost"]["connected"].as_bool().unwrap_or(false);

    let audio = json["audio"].as_object().map(|a| AudioInfo {
        active_snapshot: a
            .get("active_snapshot")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        snapshots: a.get("snapshots").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        plugins: a.get("plugins").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
    });

    Ok(BridgeInfo {
        version,
        mode,
        midi_connected,
        midi_input,
        midi_output,
        modhost_connected,
        audio,
    })
}

struct FirmwareInfo {
    version: String,
    flash_format: u8,
    presets_loaded: u8,
    presets_skipped: u8,
}

async fn query_firmware_status(raw_address: &str) -> Result<FirmwareInfo, String> {
    let (mut ws, _) = connect_async(raw_address)
        .await
        .map_err(|e| format!("{e}"))?;

    let msg = midi_controller::property_exchange::build_get_inquiry(
        [0x10, 0x20, 0x30, 0x40],
        [0x01, 0x02, 0x03, 0x04],
        0x01,
        midi_controller::config::DEVICE_INFO_RESOURCE,
    );
    ws.send(Message::Binary(msg.to_vec()))
        .await
        .map_err(|e| format!("{e}"))?;

    match tokio::time::timeout(std::time::Duration::from_secs(2), ws.next()).await {
        Ok(Some(Ok(Message::Binary(data)))) => {
            let status = midi_controller::property_exchange::extract_reply_status(&data);
            if let Some(s) = status {
                if !s.is_ok() {
                    return Err(format!("device error: {:?}", s));
                }
            }
            if let Some(body) = midi_controller::property_exchange::extract_get_body(&data) {
                let mut decoded_buf = [0u8; 64];
                let dec_len =
                    midi_controller::property_exchange::decode_mcoded7(body, &mut decoded_buf);
                let body = &decoded_buf[..dec_len];
                match postcard::from_bytes::<midi_controller::config::DeviceInfo>(body) {
                    Ok(info) => Ok(FirmwareInfo {
                        version: info.version.to_string(),
                        flash_format: info.flash_format,
                        presets_loaded: info.presets_loaded,
                        presets_skipped: info.presets_skipped,
                    }),
                    Err(e) => Err(format!("decode: {e}")),
                }
            } else {
                Err("no body in reply".to_string())
            }
        }
        _ => Err("no reply".to_string()),
    }
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
