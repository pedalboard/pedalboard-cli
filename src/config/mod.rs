// Re-export everything from the shared pedalboard-config crate.
// The CLI is a thin consumer — all config logic lives in the shared crate.

pub use pedalboard_config::convert::yaml_global_to_protocol;
pub use pedalboard_config::convert::yaml_to_presets;
pub use pedalboard_config::schema::generate_schema;
pub use pedalboard_config::validate::validate;
pub use pedalboard_config::*;
