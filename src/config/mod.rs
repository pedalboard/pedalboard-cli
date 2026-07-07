pub mod color;
pub mod convert;
pub mod schema;

use schemars::JsonSchema;
use serde::Deserialize;

/// Current setlist schema version. Bump major on breaking changes, minor on additions.
pub const SCHEMA_VERSION: u8 = 1;

/// A setlist file containing one or more presets and optional global config.
#[derive(Deserialize, JsonSchema)]
pub struct Setlist {
    /// Schema version (currently 1). Tools use this to determine compatibility.
    /// Omit for v1 (assumed). Required from v2 onward.
    #[serde(default = "default_schema_version")]
    pub version: u8,
    /// Global device settings (MIDI routing, clock, etc.). Applied once on upload.
    #[serde(default)]
    pub global: Option<GlobalYamlConfig>,
    /// List of presets. Each preset defines the complete button/encoder/expression layout for one song or scene.
    pub presets: Vec<PresetConfig>,
}

fn default_schema_version() -> u8 {
    1
}

/// Global device configuration — system-wide settings independent of presets.
#[derive(Deserialize, JsonSchema)]
pub struct GlobalYamlConfig {
    /// Enable DIN MIDI output for locally-generated messages. Default: true.
    #[serde(default)]
    pub din_enabled: Option<bool>,
    /// Route incoming DIN MIDI to USB MIDI out. Default: true.
    #[serde(default)]
    pub din_to_usb_thru: Option<bool>,
    /// Route incoming USB MIDI to DIN MIDI out. Default: false.
    #[serde(default)]
    pub usb_to_din_thru: Option<bool>,
    /// Route incoming USB MIDI back to USB MIDI out (echo). Default: false.
    #[serde(default)]
    pub usb_to_usb_thru: Option<bool>,
    /// Enable MIDI Clock output. Default: false.
    #[serde(default)]
    pub midi_clock: Option<bool>,
    /// MIDI Clock tempo in BPM (30-300). Default: 120.
    #[serde(default)]
    pub bpm: Option<u16>,
    /// Expression pedal ADC calibration values.
    #[serde(default)]
    pub calibration: Option<CalibrationYaml>,
}

/// ADC calibration for expression pedals.
#[derive(Deserialize, JsonSchema)]
pub struct CalibrationYaml {
    /// Expression pedal 1 calibration.
    #[serde(default)]
    pub exp1: Option<ExpCalibration>,
    /// Expression pedal 2 calibration.
    #[serde(default)]
    pub exp2: Option<ExpCalibration>,
}

/// Min/max ADC values for a single expression pedal (0-4095).
#[derive(Deserialize, JsonSchema)]
pub struct ExpCalibration {
    /// ADC value at heel (rest) position. Default: 0.
    #[serde(default)]
    pub min: u16,
    /// ADC value at toe (full) position. Default: 3750.
    #[serde(default = "default_adc_max")]
    pub max: u16,
}

fn default_adc_max() -> u16 {
    3750
}

/// A single preset — one song or scene in your setlist.
#[derive(Deserialize, JsonSchema)]
pub struct PresetConfig {
    /// Preset name displayed on the OLED (max 16 characters).
    pub name: String,
    /// Button configurations keyed by position: A, B, C, D, E, F.
    #[serde(default)]
    pub buttons: std::collections::HashMap<String, ButtonConfig>,
    /// Encoder configurations keyed by position: Vol (left), Gain (right).
    #[serde(default)]
    pub encoders: std::collections::HashMap<String, EncoderConfig>,
    /// Expression pedal configurations keyed by jack: Exp1, Exp2.
    #[serde(default)]
    pub analog: std::collections::HashMap<String, AnalogYamlConfig>,
    /// Initial state on first activation after upload. Determines which toggles start ON and encoder starting positions.
    #[serde(default)]
    pub defaults: Option<DefaultsConfig>,
    /// MIDI messages sent automatically when this preset becomes active (on switch or boot).
    /// Fires on: preset switch (encoder scroll, long-press, or trigger), and boot (for the active preset).
    #[serde(default)]
    pub on_enter: Option<Vec<ActionYaml>>,
    /// MIDI messages sent automatically when leaving this preset (before switching to another).
    #[serde(default)]
    pub on_exit: Option<Vec<ActionYaml>>,
    /// Incoming MIDI triggers: react to external messages by changing button state or switching presets.
    #[serde(default)]
    pub triggers: Option<Vec<TriggerYaml>>,
}

/// A trigger that reacts to incoming MIDI.
///
/// Multiple triggers can match the same incoming message — all matching triggers fire.
/// Triggers only affect LED state and system actions; they don't generate outgoing MIDI
/// unless using Execute (which fires a button's on_press actions).
#[derive(Deserialize, JsonSchema)]
pub struct TriggerYaml {
    /// What MIDI message to match. One of: { cc, channel, value_gte?, value_lt? }, { program_change, channel }, { note, channel }
    #[serde(rename = "match")]
    pub match_msg: TriggerMatchYaml,
    /// Action to perform. One of: { activate: "A" }, { deactivate: "B" }, { preset_select: 0 }, { execute: "C" }
    pub action: TriggerActionYaml,
}

/// MIDI message pattern to match.
///
/// CC: matches Control Change with optional value range (value_gte/value_lt).
/// ProgramChange: matches exact program number on channel.
/// NoteOn: matches Note On with velocity > 0 only (velocity 0 = Note Off, not matched).
#[derive(Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TriggerMatchYaml {
    Cc {
        cc: u8,
        channel: u8,
        #[serde(default)]
        value_gte: Option<u8>,
        #[serde(default)]
        value_lt: Option<u8>,
    },
    ProgramChange {
        program_change: u8,
        channel: u8,
    },
    NoteOn {
        note: u8,
        channel: u8,
    },
}

/// Action to perform when a trigger matches.
///
/// - Activate: set button LED on (no outgoing MIDI).
/// - Deactivate: set button LED off (no outgoing MIDI).
/// - PresetSelect: switch to preset by 0-based index.
/// - Execute: fire the button's on_press actions as if physically pressed.
#[derive(Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TriggerActionYaml {
    Activate { activate: String },
    Deactivate { deactivate: String },
    PresetSelect { preset_select: u8 },
    Execute { execute: String },
}

/// Default initial state for a preset on first activation after upload.
///
/// Determines which toggles start ON and encoder starting positions.
/// After initial activation, runtime state is preserved in EEPROM across power cycles
/// and takes precedence over these defaults.
#[derive(Deserialize, JsonSchema)]
pub struct DefaultsConfig {
    /// Button keys (A-F) mapped to "on" or "off". Omitted buttons default to off.
    #[serde(default)]
    pub buttons: std::collections::HashMap<String, String>,
    /// Encoder keys (Vol, Gain) mapped to initial value (0-127). Omitted encoders default to 0.
    #[serde(default)]
    pub encoders: std::collections::HashMap<String, u8>,
}

/// Expression pedal (analog input) configuration.
///
/// The raw ADC reading is clamped to the calibrated range (from global.calibration),
/// then linearly mapped to the min–max CC range. Values below adc_min produce `min`;
/// values above adc_max produce `max`. A value overlay appears on the OLED when moving.
#[derive(Deserialize, JsonSchema)]
pub struct AnalogYamlConfig {
    /// Display label for the expression pedal overlay.
    pub label: String,
    /// MIDI CC number to send (0-127).
    pub cc: u8,
    /// MIDI channel (1-16). Default: 1.
    #[serde(default)]
    pub channel: Option<u8>,
    /// Minimum CC value at heel position. Default: 0.
    #[serde(default)]
    pub min: Option<u8>,
    /// Maximum CC value at toe position. Default: 127.
    #[serde(default)]
    pub max: Option<u8>,
}

/// Button configuration. Use one of: note, cc, program_change, or actions for the MIDI message.
///
/// **Modes:**
/// - Momentary (default): active while pressed. Fires on_press on press, on_release on release.
/// - Toggle (`toggle: true`): alternates active/inactive each press. Press when OFF → fires on_press.
///   Press when ON → fires on_release (deactivation). LED stays lit while active.
/// - RadioGroup (`radio_group: N`): only one button in group N active at a time. Pressing one
///   silently deactivates others (no on_release fired for deactivated buttons).
///
/// **CcCycle** (`cc` + `values`): each press sends the next value in the list, wrapping around.
/// With `reverse: true`, cycles backward. Cycle index persists in EEPROM across power cycles.
///
/// **Long press** (`on_long_press`): hold >500ms for secondary action. When configured,
/// the normal on_press is deferred until release (if released before 500ms threshold).
#[derive(Deserialize, JsonSchema)]
pub struct ButtonConfig {
    /// Display label shown on OLED (max 16 characters).
    pub label: String,
    /// Send Note On/Off. Button press = Note On (velocity 127), release = Note Off.
    #[serde(default)]
    pub note: Option<u8>,
    /// Send Control Change. Combined with toggle/values for different behaviors.
    #[serde(default)]
    pub cc: Option<u8>,
    /// Send Program Change on press.
    #[serde(default)]
    pub program_change: Option<u8>,
    /// CC value to send (default: 127). For toggle mode, this is the ON value.
    #[serde(default)]
    pub value: Option<u8>,
    /// Toggle mode: alternates between on_press (active) and on_release (inactive) on each press.
    /// LED stays lit while active. First press activates (sends CC=value), second press
    /// deactivates (sends CC=0 for the shorthand form, or fires on_release actions).
    #[serde(default)]
    pub toggle: Option<bool>,
    /// Radio group ID (0-255): only one button in the group can be active at a time.
    /// Pressing one deactivates others silently (no on_release MIDI sent for deactivated buttons).
    #[serde(default)]
    pub radio_group: Option<u8>,
    /// Level mode: LED brightness reflects CC value (for multi-LED visualization).
    #[serde(default)]
    pub level: Option<bool>,
    /// LED ring color when active. Values: red, green, blue, yellow, cyan, magenta, white, orange, purple, off, or #RRGGBB hex.
    #[serde(default)]
    pub color: Option<String>,
    /// LED animation when active. Values: solid, blink, pulse, rotate, colorcycle.
    #[serde(default)]
    pub animation: Option<String>,
    /// LED spatial renderer. Values: solid (all 12), fill (partial arc), single (one LED), dots (evenly-spaced).
    #[serde(default)]
    pub renderer: Option<String>,
    /// Renderer parameter: fill count (1-12), single position (0-11), or dot count (1-6).
    #[serde(default)]
    pub renderer_param: Option<u8>,
    /// MIDI channel (1-16). Default: 1. Applies to all actions on this button unless overridden per-action.
    #[serde(default)]
    pub channel: Option<u8>,
    /// Action on long press (hold > 500ms). Values: next_preset, prev_preset.
    /// When configured, the normal on_press is deferred until release — if released before
    /// 500ms, fires on_press; if held past 500ms, fires only the long-press action.
    #[serde(default)]
    pub on_long_press: Option<String>,
    /// CC cycle values: each press sends the next value in the list (max 12). Use with cc field.
    /// First press sends values[0], second sends values[1], etc. Wraps around after the last value.
    /// Cycle index persists in EEPROM across power cycles.
    #[serde(default)]
    pub values: Option<Vec<u8>>,
    /// Reverse cycle direction: index decrements on each press, wrapping from 0 to last entry.
    #[serde(default)]
    pub reverse: Option<bool>,
    /// Multi-action sequence: list of MIDI messages sent in order on press (max 8).
    /// Supports CC, Program Change, Note On, and Delay between actions.
    /// Overrides cc/note/program_change shorthand fields.
    #[serde(default)]
    pub actions: Option<Vec<ActionYaml>>,
    /// Reactive LED: ring shows heatmap proportional to incoming CC value. Format: { cc: N, channel: N }
    #[serde(default)]
    pub listen_cc: Option<ListenCcYaml>,
    /// Tap tempo mode: each press measures the interval to compute BPM for MIDI clock.
    /// Averages the last 3 intervals (4 taps). Resets after 2 seconds idle.
    #[serde(default)]
    pub tap_tempo: Option<bool>,
}

/// Reactive CC binding for LED visualization.
///
/// Makes the button's LED ring respond to incoming MIDI CC from external gear.
/// In heatmap mode, ring fill is proportional to CC value (0=off, 127=all 12 LEDs).
/// In trigger mode, LED turns on (with button's color/animation) when value ≥ threshold.
#[derive(Deserialize, JsonSchema)]
pub struct ListenCcYaml {
    /// MIDI CC number to listen for (0-127).
    pub cc: u8,
    /// MIDI channel to listen on (1-16). Default: 1.
    #[serde(default = "default_channel")]
    pub channel: u8,
    /// Visualization mode: "heatmap" (default) or "trigger".
    #[serde(default)]
    pub mode: Option<String>,
    /// Threshold for trigger mode (0-127). Value ≥ threshold = on. Default: 64.
    #[serde(default)]
    pub threshold: Option<u8>,
}

fn default_channel() -> u8 {
    1
}

/// A single action in a multi-action sequence. Exactly one type per entry.
#[derive(Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ActionYaml {
    /// Wait between actions (milliseconds).
    Delay {
        /// Delay in milliseconds before the next action.
        delay: u16,
    },
    /// Send a Control Change message.
    Cc {
        /// CC number (0-127).
        cc: u8,
        /// CC value (0-127). Default: 127.
        value: Option<u8>,
        /// MIDI channel (1-16). Inherits from button if omitted.
        channel: Option<u8>,
    },
    /// Send a Program Change message.
    ProgramChange {
        /// Program number (0-127).
        program_change: u8,
        /// MIDI channel (1-16). Inherits from button if omitted.
        channel: Option<u8>,
    },
    /// Send a Note On message (velocity 127).
    NoteOn {
        /// MIDI note number (0-127).
        note: u8,
        /// MIDI channel (1-16). Inherits from button if omitted.
        channel: Option<u8>,
    },
    /// Set the button's LED ring color and animation mid-sequence.
    /// Useful for visual feedback during multi-step actions.
    SetLed {
        /// LED color name or #RRGGBB hex.
        color: String,
        /// Animation: solid, blink, pulse, rotate, colorcycle. Default: solid.
        #[serde(default)]
        animation: Option<String>,
    },
}

/// Rotary encoder configuration.
///
/// Three modes:
/// - **cc** (default): Absolute CC (0-127), increments/decrements per detent click.
///   Clamped at min/max. Acceleration sends a single message with the final value.
/// - **relative**: Sends fixed increment/decrement values (for gear expecting relative CC encoding).
/// - **preset_scroll**: No MIDI output. Scrolls through presets (CW = next, CCW = prev).
///
/// Encoder values persist across preset switches and power cycles via EEPROM.
/// A large value overlay appears on the OLED briefly when turning.
#[derive(Deserialize, JsonSchema)]
pub struct EncoderConfig {
    /// Display label shown on OLED overlay when turning.
    pub label: String,
    /// Encoder mode: "cc" (default), "relative", or "preset_scroll".
    #[serde(default)]
    pub mode: Option<String>,
    /// MIDI CC number to send (0-127). Used in cc and relative modes.
    #[serde(default)]
    pub cc: Option<u16>,
    /// MIDI channel (1-16). Default: 1. Used in cc and relative modes.
    #[serde(default)]
    pub channel: Option<u8>,
    /// Minimum CC value (default: 0). Only for cc mode.
    #[serde(default)]
    pub min: Option<u8>,
    /// Maximum CC value (default: 127). Only for cc mode.
    #[serde(default)]
    pub max: Option<u8>,
    /// Value sent on clockwise turn. Only for relative mode. Default: 65.
    #[serde(default)]
    pub increment: Option<u8>,
    /// Value sent on counter-clockwise turn. Only for relative mode. Default: 63.
    #[serde(default)]
    pub decrement: Option<u8>,
}

pub const BUTTON_KEYS: &[&str] = &["A", "B", "C", "D", "E", "F"];
pub const ENCODER_KEYS: &[&str] = &["Vol", "Gain"];

// Re-export public API for backwards compatibility
pub use convert::{convert_actions, convert_triggers, yaml_global_to_protocol, yaml_to_presets};
pub use schema::generate_schema;
