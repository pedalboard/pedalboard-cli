use std::path::PathBuf;

use crate::config::{yaml_global_to_protocol, yaml_to_presets, Setlist};

pub fn compile(file: &PathBuf, output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file)?;
    let setlist: Setlist = serde_yaml::from_str(&content)?;

    let presets = yaml_to_presets(&setlist);

    let mut config = pedalboard_protocol::config::Config::default();

    // Set global config
    if let Some(ref global_yaml) = setlist.global {
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
