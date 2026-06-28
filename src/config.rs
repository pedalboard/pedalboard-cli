use schemars::JsonSchema;
use serde::Deserialize;

/// A setlist file containing one or more presets.
#[derive(Deserialize, JsonSchema)]
pub struct Setlist {
    /// List of presets. Each preset defines the complete button/encoder/expression layout for one song or scene.
    pub presets: Vec<PresetConfig>,
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
}

/// Default initial state for a preset on first activation after upload.
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
    /// Toggle mode: alternates between on_press (active) and on_release (inactive) on each press. LED stays lit while active.
    #[serde(default)]
    pub toggle: Option<bool>,
    /// Radio group ID (0-255): only one button in the group can be active at a time. Pressing one deactivates others.
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
    #[serde(default)]
    pub on_long_press: Option<String>,
    /// CC cycle values: each press sends the next value in the list. Use with cc field.
    #[serde(default)]
    pub values: Option<Vec<u8>>,
    /// Reverse cycle direction (cycle values list goes backward).
    #[serde(default)]
    pub reverse: Option<bool>,
    /// Multi-action sequence: list of MIDI messages sent in order on press. Overrides cc/note/program_change fields.
    #[serde(default)]
    pub actions: Option<Vec<ActionYaml>>,
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
}

/// Rotary encoder configuration.
#[derive(Deserialize, JsonSchema)]
pub struct EncoderConfig {
    /// Display label shown on OLED overlay when turning.
    pub label: String,
    /// MIDI CC number to send (0-127). Each detent click increments/decrements the value.
    #[serde(default)]
    pub cc: Option<u16>,
    /// MIDI channel (1-16). Default: 1.
    #[serde(default)]
    pub channel: Option<u8>,
}

pub const BUTTON_KEYS: &[&str] = &["A", "B", "C", "D", "E", "F"];
pub const ENCODER_KEYS: &[&str] = &["Vol", "Gain"];

pub fn yaml_to_presets(setlist: &Setlist) -> Vec<pedalboard_protocol::config::Preset> {
    use pedalboard_protocol::config as pc;

    setlist
        .presets
        .iter()
        .map(|p| {
            let mut buttons = heapless::Vec::new();
            for key in BUTTON_KEYS {
                let btn_cfg = if let Some(btn) = p.buttons.get(*key) {
                    let mut on_press: heapless::Vec<pc::Action, { pc::MAX_ACTIONS }> =
                        heapless::Vec::new();

                    if let Some(actions) = &btn.actions {
                        for action in actions {
                            let _ = match action {
                                ActionYaml::Delay { delay } => {
                                    on_press.push(pc::Action::Delay(*delay))
                                }
                                ActionYaml::Cc { cc, value, channel } => {
                                    on_press.push(pc::Action::Cc {
                                        cc: *cc,
                                        value: value.unwrap_or(127),
                                        channel: channel.or(btn.channel).unwrap_or(1),
                                    })
                                }
                                ActionYaml::ProgramChange {
                                    program_change,
                                    channel,
                                } => on_press.push(pc::Action::ProgramChange {
                                    program: *program_change,
                                    channel: channel.or(btn.channel).unwrap_or(1),
                                }),
                                ActionYaml::NoteOn { note, channel } => {
                                    on_press.push(pc::Action::NoteOn {
                                        note: *note,
                                        channel: channel.or(btn.channel).unwrap_or(1),
                                    })
                                }
                            };
                        }
                    } else if let Some(prog) = btn.program_change {
                        let _ = on_press.push(pc::Action::ProgramChange {
                            program: prog,
                            channel: btn.channel.unwrap_or(1),
                        });
                    } else if let Some(cc) = btn.cc {
                        if btn.values.is_some() {
                            let _ = on_press.push(pc::Action::CcCycle {
                                cc,
                                channel: btn.channel.unwrap_or(1),
                                reverse: btn.reverse.unwrap_or(false),
                            });
                        } else if btn.toggle == Some(true) {
                            let _ = on_press.push(pc::Action::CcToggle {
                                cc,
                                value_a: btn.value.unwrap_or(127),
                                value_b: 0,
                                channel: btn.channel.unwrap_or(1),
                            });
                        } else {
                            let _ = on_press.push(pc::Action::Cc {
                                cc,
                                value: btn.value.unwrap_or(127),
                                channel: btn.channel.unwrap_or(1),
                            });
                        }
                    } else if let Some(note) = btn.note {
                        let _ = on_press.push(pc::Action::NoteOn {
                            note,
                            channel: btn.channel.unwrap_or(1),
                        });
                    }

                    let color = match btn.color.as_deref() {
                        Some("red") => pc::Color::Red,
                        Some("green") => pc::Color::Green,
                        Some("blue") => pc::Color::Blue,
                        Some("yellow") => pc::Color::Yellow,
                        Some("cyan") => pc::Color::Cyan,
                        Some("magenta") => pc::Color::Magenta,
                        Some("white") => pc::Color::White,
                        Some("orange") => pc::Color::Orange,
                        Some("purple") => pc::Color::Purple,
                        Some("off") => pc::Color::Off,
                        Some(hex) if hex.starts_with('#') && hex.len() == 7 => {
                            let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
                            let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
                            let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
                            pc::Color::Custom(r, g, b)
                        }
                        _ => pc::Color::Off,
                    };

                    let mode = if btn.toggle == Some(true) {
                        pc::ButtonMode::Toggle
                    } else if let Some(group) = btn.radio_group {
                        pc::ButtonMode::RadioGroup(group)
                    } else {
                        pc::ButtonMode::Momentary
                    };

                    pc::ButtonConfig {
                        label: pc::Label::try_from(btn.label.as_str()).unwrap_or_default(),
                        color: pc::LedConfig {
                            on: color,
                            off: pc::Color::Off,
                            animation: match btn.animation.as_deref() {
                                Some("blink") => pc::LedAnimation::Blink,
                                Some("pulse") => pc::LedAnimation::Pulse,
                                Some("rotate") => pc::LedAnimation::Rotate,
                                Some("colorcycle") => pc::LedAnimation::ColorCycle,
                                _ => pc::LedAnimation::Solid,
                            },
                            renderer: match btn.renderer.as_deref() {
                                Some("fill") => pc::LedRenderer::Fill,
                                Some("single") => pc::LedRenderer::Single,
                                Some("dots") => pc::LedRenderer::Dots,
                                _ => pc::LedRenderer::Solid,
                            },
                            renderer_param: btn.renderer_param.unwrap_or(0),
                        },
                        mode,
                        on_press,
                        on_release: heapless::Vec::new(),
                        on_long_press: {
                            let mut lp = heapless::Vec::new();
                            match btn.on_long_press.as_deref() {
                                Some("next_preset") => {
                                    lp.push(pc::Action::PresetNext).ok();
                                }
                                Some("prev_preset") => {
                                    lp.push(pc::Action::PresetPrev).ok();
                                }
                                _ => {}
                            }
                            lp
                        },
                        cycle_values: {
                            let mut cv = heapless::Vec::new();
                            if let Some(vals) = &btn.values {
                                for &v in vals.iter().take(pc::MAX_CYCLE_VALUES) {
                                    cv.push(v).ok();
                                }
                            }
                            cv
                        },
                    }
                } else {
                    pc::ButtonConfig {
                        label: pc::Label::new(),
                        color: pc::LedConfig::default(),
                        mode: pc::ButtonMode::default(),
                        on_press: heapless::Vec::new(),
                        on_release: heapless::Vec::new(),
                        on_long_press: heapless::Vec::new(),
                        cycle_values: heapless::Vec::new(),
                    }
                };
                let _ = buttons.push(btn_cfg);
            }

            let mut encoders = heapless::Vec::new();
            for key in ENCODER_KEYS {
                let enc_cfg = if let Some(enc) = p.encoders.get(*key) {
                    pc::EncoderConfig {
                        label: pc::Label::try_from(enc.label.as_str()).unwrap_or_default(),
                        action: pc::EncoderAction::Cc {
                            cc: enc.cc.unwrap_or(0),
                            channel: enc.channel.unwrap_or(1),
                            min: 0,
                            max: 127,
                        },
                    }
                } else {
                    pc::EncoderConfig {
                        label: pc::Label::new(),
                        action: pc::EncoderAction::Cc {
                            cc: 0,
                            channel: 1,
                            min: 0,
                            max: 127,
                        },
                    }
                };
                let _ = encoders.push(enc_cfg);
            }

            const ANALOG_KEYS: &[&str] = &["Exp2", "Exp1"];
            let mut analog = heapless::Vec::new();
            for key in ANALOG_KEYS {
                if let Some(a) = p.analog.get(*key) {
                    let _ = analog.push(pc::AnalogConfig {
                        label: pc::Label::try_from(a.label.as_str()).unwrap_or_default(),
                        cc: a.cc,
                        channel: a.channel.unwrap_or(1),
                        min: a.min.unwrap_or(0),
                        max: a.max.unwrap_or(127),
                    });
                } else if !analog.is_empty() || p.analog.values().any(|_| true) {
                    let _ = analog.push(pc::AnalogConfig {
                        label: pc::Label::new(),
                        cc: 0,
                        channel: 0,
                        min: 0,
                        max: 0,
                    });
                }
            }

            pc::Preset {
                name: pc::Label::try_from(p.name.as_str()).unwrap_or_default(),
                buttons,
                encoders,
                analog,
                defaults: {
                    let mut initial = pc::InitialState::default();
                    if let Some(ref defs) = p.defaults {
                        for key in BUTTON_KEYS {
                            let active = defs.buttons.get(*key).map(|v| v == "on").unwrap_or(false);
                            initial.button_active.push(active).ok();
                        }
                        for key in ENCODER_KEYS {
                            let val = defs.encoders.get(*key).copied().unwrap_or(0);
                            initial.encoder_values.push(val).ok();
                        }
                    }
                    initial
                },
            }
        })
        .collect()
}

pub fn generate_schema() -> String {
    let schema = schemars::schema_for!(Setlist);
    serde_json::to_string_pretty(&schema).unwrap()
}
