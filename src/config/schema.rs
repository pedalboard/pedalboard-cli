use super::Setlist;

pub fn generate_schema() -> String {
    let schema = schemars::schema_for!(Setlist);
    serde_json::to_string_pretty(&schema).unwrap()
}
