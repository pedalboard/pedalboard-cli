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
