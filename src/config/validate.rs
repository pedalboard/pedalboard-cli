use crate::config::Setlist;

/// Validation error with context about where in the config the issue is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub preset: String,
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "preset '{}', {}: {}",
            self.preset, self.field, self.message
        )
    }
}

/// Validate a setlist config. Returns a list of errors (empty = valid).
/// This is a pure function on the config types — no I/O, no side effects.
/// Designed to be extractable into a shared crate later.
pub fn validate(setlist: &Setlist) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    for preset in &setlist.presets {
        for (key, btn) in &preset.buttons {
            // toggle and radio_group are mutually exclusive modes
            if btn.toggle == Some(true) && btn.radio_group.is_some() {
                errors.push(ValidationError {
                    preset: preset.name.clone(),
                    field: format!("button {key}"),
                    message: "toggle and radio_group are mutually exclusive. Use radio_group alone (it already stays lit when active).".to_string(),
                });
            }

            // audio_snapshot on a button requires audio section
            if btn.audio_snapshot.is_some() && setlist.audio.is_none() {
                errors.push(ValidationError {
                    preset: preset.name.clone(),
                    field: format!("button {key}"),
                    message: "audio_snapshot requires a top-level 'audio' section.".to_string(),
                });
            }
        }

        // audio_snapshot on preset requires audio section
        if preset.audio_snapshot.is_some() && setlist.audio.is_none() {
            errors.push(ValidationError {
                preset: preset.name.clone(),
                field: "audio_snapshot".to_string(),
                message: "requires a top-level 'audio' section.".to_string(),
            });
        }

        // audio_snapshot references must resolve to defined snapshots
        if let (Some(ref audio), Some(ref snap_name)) = (&setlist.audio, &preset.audio_snapshot) {
            if !audio.snapshots.iter().any(|s| &s.name == snap_name) {
                errors.push(ValidationError {
                    preset: preset.name.clone(),
                    field: "audio_snapshot".to_string(),
                    message: format!("snapshot '{}' not found in audio.snapshots.", snap_name),
                });
            }
        }

        // same for button audio_snapshot references
        if let Some(ref audio) = setlist.audio {
            for (key, btn) in &preset.buttons {
                if let Some(ref snap_name) = btn.audio_snapshot {
                    if !audio.snapshots.iter().any(|s| &s.name == snap_name) {
                        errors.push(ValidationError {
                            preset: preset.name.clone(),
                            field: format!("button {key}"),
                            message: format!(
                                "audio_snapshot '{}' not found in audio.snapshots.",
                                snap_name
                            ),
                        });
                    }
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(yaml: &str) -> Setlist {
        serde_yaml::from_str(yaml).unwrap()
    }

    #[test]
    fn valid_config_no_errors() {
        let setlist = parse(
            r#"
presets:
  - name: "Test"
    buttons:
      A: { label: "X", cc: 80, toggle: true, color: green }
      B: { label: "Y", radio_group: 1, cc: 10, color: blue }
"#,
        );
        assert!(validate(&setlist).is_empty());
    }

    #[test]
    fn toggle_radio_group_conflict() {
        let setlist = parse(
            r#"
presets:
  - name: "Bad"
    buttons:
      A: { label: "X", cc: 80, toggle: true, radio_group: 1 }
"#,
        );
        let errors = validate(&setlist);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("mutually exclusive"));
    }

    #[test]
    fn audio_snapshot_without_audio_section() {
        let setlist = parse(
            r#"
presets:
  - name: "Bad"
    audio_snapshot: "Clean"
    buttons:
      A: { label: "X", audio_snapshot: "Clean", color: green }
"#,
        );
        let errors = validate(&setlist);
        assert_eq!(errors.len(), 2); // one for preset, one for button
    }

    #[test]
    fn audio_snapshot_reference_not_found() {
        let setlist = parse(
            r#"
audio:
  plugins: []
  connections: []
  snapshots:
    - name: "Clean"
      state: {}
presets:
  - name: "Bad"
    audio_snapshot: "Nonexistent"
    buttons:
      A: { label: "X", color: green }
"#,
        );
        let errors = validate(&setlist);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("not found"));
    }

    #[test]
    fn valid_audio_config() {
        let setlist = parse(
            r#"
audio:
  plugins: []
  connections: []
  snapshots:
    - name: "Clean"
      state: {}
    - name: "Lead"
      state: {}
presets:
  - name: "Song"
    audio_snapshot: "Clean"
    buttons:
      A: { label: "Lead", audio_snapshot: "Lead", color: red }
"#,
        );
        assert!(validate(&setlist).is_empty());
    }
}
