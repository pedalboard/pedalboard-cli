//! Compiler integration tests — verifies ADR-005 auto-generated config on real hardware.
//!
//! Run: `cargo test -p hardware-tests --test compiler`

use hardware_tests::Device;
use serial_test::serial;
use std::time::Duration;

fn device() -> Device {
    Device::from_env()
}

#[tokio::test]
#[serial]
async fn compiled_audio_preset_uploads() {
    let d = device();
    let result = d.upload("examples/minimal-audio.yaml").await.unwrap();
    assert_eq!(result.preset_acks, 1, "should upload 1 compiled preset");
}

#[tokio::test]
#[serial]
async fn compiled_preset_has_generated_buttons() {
    let d = device();
    d.upload("examples/minimal-audio.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let p0 = d.read(0).await.unwrap();
    assert_eq!(p0.name, "Compiler Test");
    assert!(
        p0.buttons.contains(&"Clean".to_string()),
        "should have Clean button, got: {:?}",
        p0.buttons
    );
    assert!(
        p0.buttons.contains(&"Crunch".to_string()),
        "should have Crunch button"
    );
    assert!(
        p0.buttons.contains(&"Lead".to_string()),
        "should have Lead button"
    );
}

#[tokio::test]
#[serial]
async fn compiled_preset_persists_across_reboot() {
    let d = device();
    d.upload("examples/minimal-audio.yaml").await.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    d.reboot_and_wait().await;

    let p0 = d.read(0).await.unwrap();
    assert_eq!(p0.name, "Compiler Test");
    assert!(p0.buttons.contains(&"Clean".to_string()));
}
