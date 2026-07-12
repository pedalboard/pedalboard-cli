//! Hardware test infrastructure for the pedalboard.
//!
//! Provides a `Device` struct with typed methods for interacting with the
//! real hardware over WebSocket (PE protocol) and HTTP (bridge status).
//!
//! # Running tests
//!
//! ```bash
//! # All hardware tests (sequential):
//! cargo test -p hardware-tests
//!
//! # Just MIDI tests:
//! cargo test -p hardware-tests --test midi
//!
//! # One specific test:
//! cargo test -p hardware-tests --test midi preset_upload
//!
//! # List all tests:
//! cargo test -p hardware-tests -- --list
//! ```
//!
//! # Environment
//!
//! Set `PEDALBOARD_ADDR` to override the default bridge address:
//! ```bash
//! PEDALBOARD_ADDR=ws://192.168.1.50:8080 cargo test -p hardware-tests
//! ```

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// Default bridge address.
const DEFAULT_ADDR: &str = "ws://cm5-dev.home:8080";

/// Root directory of the pedalboard-cli crate (for resolving relative paths to examples/fixtures).
fn cli_root() -> std::path::PathBuf {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // tests/hardware/ → go up two levels to reach pedalboard-cli root
    manifest.parent().unwrap().parent().unwrap().to_path_buf()
}

/// Connection to the pedalboard hardware via the bridge.
pub struct Device {
    pub address: String,
}

/// Result of reading a preset back from the device.
#[derive(Debug)]
pub struct PresetInfo {
    pub name: String,
    pub buttons: Vec<String>,
    pub encoders: Vec<String>,
}

/// Bridge status information.
#[derive(Debug, serde::Deserialize)]
pub struct BridgeStatus {
    pub version: String,
    pub mode: String,
    pub midi: MidiStatus,
    pub modhost: ModHostStatus,
    pub audio: Option<AudioStatus>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MidiStatus {
    pub connected: bool,
    pub input: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ModHostStatus {
    pub connected: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct AudioStatus {
    pub active_snapshot: Option<String>,
    pub snapshots: usize,
    pub plugins: usize,
}

/// Upload result with per-preset ACK status.
#[derive(Debug)]
pub struct UploadResult {
    pub preset_acks: usize,
    pub global_ack: bool,
}

impl Device {
    /// Create a new device connection from environment or default.
    pub fn from_env() -> Self {
        let address = std::env::var("PEDALBOARD_ADDR").unwrap_or(DEFAULT_ADDR.to_string());
        Self { address }
    }

    /// Raw WebSocket address for PE protocol.
    fn raw_url(&self) -> String {
        format!("{}/raw", self.address.trim_end_matches('/'))
    }

    /// HTTP base URL (for bridge REST API).
    fn http_url(&self) -> String {
        self.address
            .replace("ws://", "http://")
            .replace("wss://", "https://")
    }

    /// Check if device is reachable (can connect and get any PE response).
    pub async fn is_reachable(&self) -> bool {
        let Ok((mut ws, _)) = connect_async(&self.raw_url()).await else {
            return false;
        };
        let msg = midi_controller::property_exchange::build_get_inquiry(
            [0x10, 0x20, 0x30, 0x40],
            [0x01, 0x02, 0x03, 0x04],
            0x01,
            midi_controller::config::DEVICE_INFO_RESOURCE,
        );
        if ws.send(Message::Binary(msg.to_vec())).await.is_err() {
            return false;
        }
        matches!(
            tokio::time::timeout(Duration::from_secs(2), ws.next()).await,
            Ok(Some(Ok(Message::Binary(_))))
        )
    }

    /// Read a preset from the device. Returns structured preset info.
    pub async fn read(&self, index: u8) -> Result<PresetInfo, String> {
        let (mut ws, _) = connect_async(&self.raw_url())
            .await
            .map_err(|e| format!("connect: {e}"))?;

        let msg = midi_controller::property_exchange::build_get_inquiry(
            [0x10, 0x20, 0x30, 0x40],
            [0x01, 0x02, 0x03, 0x04],
            index,
            index,
        );
        ws.send(Message::Binary(msg.to_vec()))
            .await
            .map_err(|e| format!("send: {e}"))?;

        match tokio::time::timeout(Duration::from_secs(3), ws.next()).await {
            Ok(Some(Ok(Message::Binary(data)))) => {
                let status = midi_controller::property_exchange::extract_reply_status(&data);
                if let Some(s) = status {
                    if !s.is_ok() {
                        return Err(format!("device error: {:?}", s));
                    }
                }
                if let Some(body) = midi_controller::property_exchange::extract_get_body(&data) {
                    let mut decoded_buf = [0u8; 256];
                    let dec_len =
                        midi_controller::property_exchange::decode_mcoded7(body, &mut decoded_buf);
                    let body = &decoded_buf[..dec_len];
                    match postcard::from_bytes::<midi_controller::config::Preset>(body) {
                        Ok(preset) => Ok(PresetInfo {
                            name: preset.name.to_string(),
                            buttons: preset
                                .buttons
                                .iter()
                                .map(|b| b.label.to_string())
                                .collect(),
                            encoders: preset
                                .encoders
                                .iter()
                                .map(|e| e.label.to_string())
                                .collect(),
                        }),
                        Err(e) => Err(format!("decode: {e}")),
                    }
                } else {
                    Err("not found".to_string())
                }
            }
            _ => Err("no reply".to_string()),
        }
    }

    /// Upload a YAML setlist file to the device (runs compiler + PE upload).
    /// Path is relative to the pedalboard-cli crate root.
    pub async fn upload(&self, yaml_path: &str) -> Result<UploadResult, String> {
        let full_path = cli_root().join(yaml_path);
        let content =
            std::fs::read_to_string(&full_path).map_err(|e| format!("read file {:?}: {e}", full_path))?;
        let setlist: pedalboard_config::Setlist =
            serde_yaml::from_str(&content).map_err(|e| format!("parse yaml: {e}"))?;

        // Run compiler (ADR-005 auto-generation).
        let compiled = pedalboard_config::compile::compile(setlist);
        let setlist = compiled.setlist;

        let presets = pedalboard_config::yaml_to_presets(&setlist);
        let global = setlist
            .global
            .as_ref()
            .map(pedalboard_config::yaml_global_to_protocol);

        let (mut ws, _) = connect_async(&self.raw_url())
            .await
            .map_err(|e| format!("connect: {e}"))?;
        tokio::time::sleep(Duration::from_millis(200)).await;

        let mut global_ack = false;
        let mut preset_acks = 0usize;

        // Upload global config if present.
        if let Some(gc) = &global {
            let serialized =
                postcard::to_allocvec(gc).map_err(|e| format!("serialize gc: {e}"))?;
            let msg = midi_controller::property_exchange::build_set_inquiry(
                [0x10, 0x20, 0x30, 0x40],
                [0x01, 0x02, 0x03, 0x04],
                0x7F,
                midi_controller::config::GLOBAL_CONFIG_RESOURCE,
                &serialized,
            );
            ws.send(Message::Binary(msg.to_vec()))
                .await
                .map_err(|e| format!("send gc: {e}"))?;
            if let Ok(Some(Ok(_))) =
                tokio::time::timeout(Duration::from_secs(3), ws.next()).await
            {
                global_ack = true;
            }
        }

        // Upload presets.
        for (idx, preset) in presets.iter().enumerate() {
            let serialized =
                postcard::to_allocvec(preset).map_err(|e| format!("serialize preset: {e}"))?;
            let msg = midi_controller::property_exchange::build_set_inquiry(
                [0x10, 0x20, 0x30, 0x40],
                [0x01, 0x02, 0x03, 0x04],
                idx as u8 + 1,
                idx as u8,
                &serialized,
            );
            ws.send(Message::Binary(msg.to_vec()))
                .await
                .map_err(|e| format!("send preset {idx}: {e}"))?;
            if let Ok(Some(Ok(_))) =
                tokio::time::timeout(Duration::from_secs(3), ws.next()).await
            {
                preset_acks += 1;
            }
        }

        // Clear stale presets beyond uploaded count (empty body = delete).
        let uploaded_count = presets.len() as u8;
        for idx in uploaded_count..32 {
            let msg = midi_controller::property_exchange::build_set_inquiry(
                [0x10, 0x20, 0x30, 0x40],
                [0x01, 0x02, 0x03, 0x04],
                idx + 1,
                idx,
                &[],
            );
            ws.send(Message::Binary(msg.to_vec()))
                .await
                .map_err(|e| format!("clear preset {idx}: {e}"))?;
        }
        // Drain ACKs.
        for _ in uploaded_count..32 {
            if tokio::time::timeout(Duration::from_millis(100), ws.next())
                .await
                .is_err()
            {
                break;
            }
        }
        // Allow firmware persist queue to drain.
        if uploaded_count < 32 {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Ok(UploadResult {
            preset_acks,
            global_ack,
        })
    }

    /// Send reboot command and wait for device to come back online.
    pub async fn reboot_and_wait(&self) {
        self.send_system_command(midi_controller::config::SystemCommand::Reboot)
            .await;
        self.wait_for_device(15).await;
    }

    /// Send factory reset and wait for device to come back online.
    pub async fn reset_and_wait(&self) {
        self.send_system_command(midi_controller::config::SystemCommand::FactoryReset)
            .await;
        self.wait_for_device(15).await;
    }

    /// Query bridge status (HTTP GET /).
    pub async fn bridge_status(&self) -> Result<BridgeStatus, String> {
        let url = format!("{}/", self.http_url().trim_end_matches('/'));
        let resp = reqwest::get(&url)
            .await
            .map_err(|e| format!("request: {e}"))?;
        resp.json().await.map_err(|e| format!("parse: {e}"))
    }

    /// Get the current mode (live/design).
    pub async fn mode(&self) -> Result<String, String> {
        let status = self.bridge_status().await?;
        Ok(status.mode)
    }

    /// Set mode (live/design).
    pub async fn set_mode(&self, mode: &str) -> Result<(), String> {
        let url = format!("{}/mode?set={}", self.http_url().trim_end_matches('/'), mode);
        reqwest::Client::new()
            .post(&url)
            .send()
            .await
            .map_err(|e| format!("request: {e}"))?;
        Ok(())
    }

    /// Monitor MIDI output for a duration. Returns raw lines.
    pub async fn monitor(&self, duration: Duration) -> Vec<String> {
        let url = format!("{}/monitor", self.address.trim_end_matches('/'));
        let Ok((mut ws, _)) = connect_async(&url).await else {
            return Vec::new();
        };

        let mut messages = Vec::new();
        let deadline = tokio::time::Instant::now() + duration;

        loop {
            let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remaining.is_zero() {
                break;
            }
            match tokio::time::timeout(remaining, ws.next()).await {
                Ok(Some(Ok(Message::Binary(data)))) => {
                    messages.push(format!("{:02X?}", data));
                }
                _ => break,
            }
        }

        messages
    }

    // --- Private ---

    async fn send_system_command(&self, cmd: midi_controller::config::SystemCommand) {
        let Ok((mut ws, _)) = connect_async(&self.raw_url()).await else {
            return;
        };
        let msg = midi_controller::property_exchange::build_set_inquiry(
            [0x10, 0x20, 0x30, 0x40],
            [0x01, 0x02, 0x03, 0x04],
            0x70,
            midi_controller::config::SYSTEM_COMMAND_RESOURCE,
            &[cmd as u8],
        );
        let _ = ws.send(Message::Binary(msg.to_vec())).await;
        // Wait for ACK.
        let _ = tokio::time::timeout(Duration::from_secs(2), ws.next()).await;
    }

    /// Wait for device to go offline then come back (two-phase).
    async fn wait_for_device(&self, timeout_secs: u64) {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);

        // Phase 1: wait for device to go offline.
        loop {
            if tokio::time::Instant::now() >= deadline {
                break;
            }
            if !self.is_reachable().await {
                break;
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        // Phase 2: wait for device to come back.
        loop {
            if tokio::time::Instant::now() >= deadline {
                panic!("Timeout waiting for device after reboot");
            }
            if self.is_reachable().await {
                // Settle time for async preset loading.
                tokio::time::sleep(Duration::from_secs(2)).await;
                return;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
}

/// Run SSH command on the CM5 and return stdout.
pub async fn ssh_cmd(cmd: &str) -> Result<String, String> {
    let host = std::env::var("CM5_HOST").unwrap_or("cm5-dev.home".to_string());
    let output = tokio::process::Command::new("ssh")
        .args([&format!("laenzi@{host}"), cmd])
        .output()
        .await
        .map_err(|e| format!("ssh: {e}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}
