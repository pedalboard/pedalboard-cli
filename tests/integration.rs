use pedalboard_cli::config::{generate_schema, yaml_to_presets, Setlist};
use std::fs;

#[test]
fn schema_matches_committed_file() {
    let generated = generate_schema();
    let committed = fs::read_to_string("schema/pedalboard.schema.json")
        .expect("schema/pedalboard.schema.json not found");
    if generated.trim() != committed.trim() {
        fs::write("schema/pedalboard.schema.json", &generated).ok();
        panic!(
            "Schema out of date — regenerated schema/pedalboard.schema.json. \
             Commit the updated file."
        );
    }
}

#[test]
fn all_examples_parse_successfully() {
    let examples_dir = std::path::Path::new("examples");
    assert!(examples_dir.exists(), "examples/ directory not found");

    let mut count = 0;
    for entry in fs::read_dir(examples_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|e| e == "yaml").unwrap_or(false) {
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
            let setlist: Setlist = serde_yaml::from_str(&content)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e));
            let presets = yaml_to_presets(&setlist);
            assert!(
                !presets.is_empty(),
                "{} produced no presets",
                path.display()
            );
            for (i, preset) in presets.iter().enumerate() {
                assert!(
                    !preset.name.is_empty(),
                    "{}: preset {} has empty name",
                    path.display(),
                    i
                );
            }
            count += 1;
        }
    }
    assert!(
        count >= 3,
        "Expected at least 3 example files, found {}",
        count
    );
}

#[test]
fn global_config_parsed_from_yaml() {
    use pedalboard_cli::config::yaml_global_to_protocol;

    let yaml = r#"
global:
  din_enabled: true
  din_to_usb_thru: false
  usb_to_din_thru: true
  midi_clock: true
  bpm: 140
presets:
  - name: "Test"
    buttons: {}
"#;
    let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
    let global_yaml = setlist.global.expect("global section missing");
    let gc = yaml_global_to_protocol(&global_yaml);

    assert!(gc.din_enabled);
    assert!(!gc.din_to_usb_thru);
    assert!(gc.usb_to_din_thru);
    assert!(!gc.usb_to_usb_thru);
    assert!(gc.midi_clock);
    assert_eq!(gc.bpm, 140);
}

#[test]
fn global_config_defaults_when_omitted() {
    let yaml = r#"
presets:
  - name: "Test"
    buttons: {}
"#;
    let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
    assert!(setlist.global.is_none());
}

#[test]
fn global_config_partial_fields_use_defaults() {
    use pedalboard_cli::config::yaml_global_to_protocol;

    let yaml = r#"
global:
  midi_clock: true
  bpm: 90
presets:
  - name: "Test"
    buttons: {}
"#;
    let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
    let global_yaml = setlist.global.expect("global section missing");
    let gc = yaml_global_to_protocol(&global_yaml);

    // Unspecified fields use defaults
    assert!(gc.din_enabled);
    assert!(gc.din_to_usb_thru);
    assert!(!gc.usb_to_din_thru);
    assert!(!gc.usb_to_usb_thru);
    // Specified fields
    assert!(gc.midi_clock);
    assert_eq!(gc.bpm, 90);
}

#[test]
fn global_config_calibration_parsed() {
    use pedalboard_cli::config::yaml_global_to_protocol;

    let yaml = r#"
global:
  calibration:
    exp1: { min: 180, max: 3700 }
    exp2: { min: 200, max: 3600 }
presets:
  - name: "Test"
    buttons: {}
"#;
    let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
    let global_yaml = setlist.global.expect("global section missing");
    let gc = yaml_global_to_protocol(&global_yaml);

    assert_eq!(gc.exp1_min, 180);
    assert_eq!(gc.exp1_max, 3700);
    assert_eq!(gc.exp2_min, 200);
    assert_eq!(gc.exp2_max, 3600);
}

#[test]
fn global_config_calibration_defaults_when_omitted() {
    use pedalboard_cli::config::yaml_global_to_protocol;

    let yaml = r#"
global:
  midi_clock: false
presets:
  - name: "Test"
    buttons: {}
"#;
    let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
    let global_yaml = setlist.global.expect("global section missing");
    let gc = yaml_global_to_protocol(&global_yaml);

    assert_eq!(gc.exp1_min, 0);
    assert_eq!(gc.exp1_max, 3750);
    assert_eq!(gc.exp2_min, 0);
    assert_eq!(gc.exp2_max, 3750);
}

#[test]
fn global_config_calibration_partial() {
    use pedalboard_cli::config::yaml_global_to_protocol;

    let yaml = r#"
global:
  calibration:
    exp1: { min: 100, max: 3800 }
presets:
  - name: "Test"
    buttons: {}
"#;
    let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
    let global_yaml = setlist.global.expect("global section missing");
    let gc = yaml_global_to_protocol(&global_yaml);

    // exp1 set explicitly
    assert_eq!(gc.exp1_min, 100);
    assert_eq!(gc.exp1_max, 3800);
    // exp2 uses defaults
    assert_eq!(gc.exp2_min, 0);
    assert_eq!(gc.exp2_max, 3750);
}

#[test]
fn global_config_internal_channel_parsed() {
    use pedalboard_cli::config::yaml_global_to_protocol;

    let yaml = r#"
global:
  internal_channel: 10
presets:
  - name: "Test"
    buttons: {}
"#;
    let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
    let global_yaml = setlist.global.expect("global section missing");
    let gc = yaml_global_to_protocol(&global_yaml);

    assert_eq!(gc.internal_channel, 10);
}

#[test]
fn global_config_internal_channel_defaults_to_16() {
    use pedalboard_cli::config::yaml_global_to_protocol;

    let yaml = r#"
global:
  midi_clock: false
presets:
  - name: "Test"
    buttons: {}
"#;
    let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
    let global_yaml = setlist.global.expect("global section missing");
    let gc = yaml_global_to_protocol(&global_yaml);

    assert_eq!(gc.internal_channel, 16);
}
