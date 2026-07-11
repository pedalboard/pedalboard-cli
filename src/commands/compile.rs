use std::path::PathBuf;

use crate::config::{yaml_global_to_protocol, yaml_to_presets, Setlist};

pub fn compile(
    file: &PathBuf,
    output: &PathBuf,
    preview: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file)?;
    let setlist: Setlist = serde_yaml::from_str(&content)?;

    // Run the compiler: auto-generate buttons from audio snapshots.
    let result = pedalboard_config::compile::compile(setlist);

    if !result.notes.is_empty() {
        println!("Compiler notes:");
        for note in &result.notes {
            println!("  {}", note);
        }
        println!();
    }

    if preview {
        // Show what would be generated without writing output.
        if result.notes.is_empty() || result.notes[0].contains("No audio section") {
            println!("Nothing to auto-generate.");
        } else {
            println!("Auto-generated buttons:");
            for note in &result.notes {
                println!("{}", note);
            }
        }
        return Ok(());
    }

    let presets = yaml_to_presets(&result.setlist);

    let mut config = midi_controller::config::Config::default();

    // Set global config
    if let Some(ref global_yaml) = result.setlist.global {
        config.global = yaml_global_to_protocol(global_yaml);
    }

    // Add presets
    for preset in presets {
        config.presets.push(preset).ok();
    }

    let serialized = postcard::to_allocvec(&config)?;
    std::fs::write(output, &serialized)?;

    println!(
        "Compiled {} presets → {} ({} bytes)",
        config.presets.len(),
        output.display(),
        serialized.len()
    );

    Ok(())
}
