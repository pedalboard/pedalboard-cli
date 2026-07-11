//! MIDI/preset integration tests — verifies the PE config pipeline end-to-end.
//!
//! Run: `cargo test -p hardware-tests --test midi`
//! Single: `cargo test -p hardware-tests --test midi preset_upload`
//!
//! Each test is self-contained: uploads its own data, doesn't depend on other tests.

use hardware_tests::Device;
use serial_test::serial;
use std::time::Duration;

fn device() -> Device {
    Device::from_env()
}

#[tokio::test]
#[serial]
async fn device_reachable() {
    let d = device();
    assert!(d.is_reachable().await, "device should be reachable");
}

#[tokio::test]
#[serial]
async fn preset_upload() {
    let d = device();
    let result = d.upload("examples/feature-test.yaml").await.unwrap();
    assert_eq!(result.preset_acks, 3, "feature-test.yaml has 3 presets");
}

#[tokio::test]
#[serial]
async fn preset_read_back() {
    let d = device();
    d.upload("examples/feature-test.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let p0 = d.read(0).await.unwrap();
    assert_eq!(p0.name, "Feature Test");

    let p1 = d.read(1).await.unwrap();
    assert_eq!(p1.name, "LED Animations");
}

#[tokio::test]
#[serial]
async fn preset_content_verification() {
    let d = device();
    d.upload("examples/feature-test.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let p0 = d.read(0).await.unwrap();
    assert_eq!(p0.name, "Feature Test");
    assert!(p0.buttons.contains(&"Toggle".to_string()));
    assert!(p0.encoders.contains(&"Reverb".to_string()));
}

#[tokio::test]
#[serial]
async fn preset_persists_across_reboot() {
    let d = device();
    d.upload("examples/feature-test.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    d.reboot_and_wait().await;

    let p0 = d.read(0).await.unwrap();
    assert_eq!(p0.name, "Feature Test");

    let p1 = d.read(1).await.unwrap();
    assert_eq!(p1.name, "LED Animations");
}

#[tokio::test]
#[serial]
async fn global_config_upload() {
    let d = device();
    let result = d
        .upload("tests/fixtures/global-config-test.yaml")
        .await
        .unwrap();
    assert!(result.global_ack, "global config should be ACKed");
    assert_eq!(result.preset_acks, 1);
}

#[tokio::test]
#[serial]
async fn midi_clock_enabled_by_global_config() {
    let d = device();
    d.upload("tests/fixtures/global-config-test.yaml")
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let messages = d.monitor(Duration::from_secs(2)).await;
    let clock_count = messages.iter().filter(|m| m.contains("F8")).count();
    assert!(
        clock_count > 5,
        "expected >5 clock ticks, got {clock_count}"
    );
}

#[tokio::test]
#[serial]
async fn midi_clock_persists_across_reboot() {
    let d = device();
    d.upload("tests/fixtures/global-config-test.yaml")
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    d.reboot_and_wait().await;

    let messages = d.monitor(Duration::from_secs(2)).await;
    let clock_count = messages.iter().filter(|m| m.contains("F8")).count();
    assert!(
        clock_count > 5,
        "clock should still run after reboot, got {clock_count} ticks"
    );
}

#[tokio::test]
#[serial]
async fn factory_reset_clears_presets_and_config() {
    let d = device();
    // Upload something first.
    d.upload("tests/fixtures/global-config-test.yaml")
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    d.reset_and_wait().await;

    // Preset should be gone.
    let result = d.read(0).await;
    assert!(result.is_err(), "preset 0 should be cleared after reset");

    // Clock should be stopped (global config cleared).
    let messages = d.monitor(Duration::from_secs(2)).await;
    let clock_count = messages.iter().filter(|m| m.contains("F8")).count();
    assert_eq!(clock_count, 0, "clock should be stopped after reset");
}

#[tokio::test]
#[serial]
async fn upload_after_reset_persists() {
    let d = device();
    d.reset_and_wait().await;

    d.upload("examples/feature-test.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    d.reboot_and_wait().await;

    let p0 = d.read(0).await.unwrap();
    assert_eq!(p0.name, "Feature Test");
}

#[tokio::test]
#[serial]
async fn stale_presets_cleared_on_shorter_upload() {
    let d = device();
    // Upload 3 presets.
    d.upload("examples/feature-test.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let p1 = d.read(1).await.unwrap();
    assert_eq!(p1.name, "LED Animations");

    // Upload 1 preset — should clear slot 1.
    d.upload("tests/fixtures/single-preset.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let p0 = d.read(0).await.unwrap();
    assert_eq!(p0.name, "Solo Preset");

    let p1 = d.read(1).await;
    assert!(p1.is_err(), "slot 1 should be cleared");
}

#[tokio::test]
#[serial]
async fn stale_presets_stay_cleared_across_reboot() {
    let d = device();
    d.upload("tests/fixtures/single-preset.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    d.reboot_and_wait().await;

    let p0 = d.read(0).await.unwrap();
    assert_eq!(p0.name, "Solo Preset");

    let p1 = d.read(1).await;
    assert!(p1.is_err(), "slot 1 should stay cleared after reboot");
}

#[tokio::test]
#[serial]
async fn device_status_via_bridge() {
    let d = device();
    d.upload("tests/fixtures/single-preset.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let status = d.bridge_status().await.unwrap();
    assert_eq!(status.mode, "live");
    assert!(status.midi.connected);
}
