//! Audio integration tests — verifies the full audio processing pipeline.
//!
//! Run: `cargo test -p hardware-tests --test audio`
//! Requires: JACK + mod-host + bridge running on CM5.

use hardware_tests::{ssh_cmd, Device};
use serial_test::serial;

fn device() -> Device {
    Device::from_env()
}

#[tokio::test]
#[serial]
async fn jack_running() {
    let sr = ssh_cmd("jack_samplerate 2>/dev/null").await.unwrap();
    let buf = ssh_cmd("jack_bufsize 2>/dev/null").await.unwrap();
    assert_eq!(sr, "48000", "JACK sample rate should be 48kHz");
    assert_eq!(buf, "64", "JACK buffer should be 64 frames");
}

#[tokio::test]
#[serial]
async fn bridge_reachable() {
    let d = device();
    let status = d.bridge_status().await.unwrap();
    assert!(!status.version.is_empty(), "bridge should report version");
}

#[tokio::test]
#[serial]
async fn modhost_connected() {
    let d = device();
    let status = d.bridge_status().await.unwrap();
    assert!(status.modhost.connected, "mod-host should be connected");
}

#[tokio::test]
#[serial]
async fn rig_loaded() {
    let d = device();
    let status = d.bridge_status().await.unwrap();
    let audio = status.audio.expect("audio should be present");
    assert!(audio.plugins >= 6, "expected >=6 plugins, got {}", audio.plugins);
    assert!(audio.snapshots >= 3, "expected >=3 snapshots, got {}", audio.snapshots);
}

#[tokio::test]
#[serial]
async fn snapshot_active() {
    let d = device();
    let status = d.bridge_status().await.unwrap();
    let audio = status.audio.expect("audio should be present");
    assert!(
        audio.active_snapshot.is_some(),
        "a snapshot should be active"
    );
}

#[tokio::test]
#[serial]
async fn audio_connections_wired() {
    // mod-host manages the internal plugin chain. The external JACK routing is:
    // mod-monitor:out → system:playback (output monitoring path).
    let output_conn =
        ssh_cmd("jack_lsp -c 2>/dev/null | grep -A1 'mod-monitor:out_1' | grep playback").await;
    assert!(
        output_conn.is_ok() && !output_conn.as_ref().unwrap().is_empty(),
        "mod-monitor:out should be connected to system:playback"
    );
}

#[tokio::test]
#[serial]
async fn amp_models_loaded() {
    let d = device();
    let status = d.bridge_status().await.unwrap();
    let audio = status.audio.expect("audio should be present");
    // Each snapshot corresponds to an AIDA-X amp model (clean/crunch/lead).
    assert!(
        audio.snapshots >= 3,
        "expected >=3 amp model snapshots, got {}",
        audio.snapshots
    );
}

#[tokio::test]
#[serial]
async fn mode_switching() {
    let d = device();

    // Should start in live mode.
    let mode = d.mode().await.unwrap();
    assert_eq!(mode, "live");

    // Switch to design.
    d.set_mode("design").await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let mode = d.mode().await.unwrap();
    assert_eq!(mode, "design");

    // Switch back to live.
    d.set_mode("live").await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let mode = d.mode().await.unwrap();
    assert_eq!(mode, "live");
}
