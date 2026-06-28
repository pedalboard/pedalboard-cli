use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct Setlist {
    pub presets: Vec<PresetConfig>,
}

#[derive(Deserialize, JsonSchema)]
pub struct PresetConfig {
    pub name: String,
    #[serde(default)]
    pub buttons: std::collections::HashMap<String, ButtonConfig>,
    #[serde(default)]
    pub encoders: std::collections::HashMap<String, EncoderConfig>,
    #[serde(default)]
    pub analog: std::collections::HashMap<String, AnalogYamlConfig>,
}

#[derive(Deserialize, JsonSchema)]
pub struct AnalogYamlConfig {
    pub label: String,
    pub cc: u8,
    #[serde(default)]
    pub channel: Option<u8>,
    #[serde(default)]
    pub min: Option<u8>,
    #[serde(default)]
    pub max: Option<u8>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ButtonConfig {
    pub label: String,
    #[serde(default)]
    pub note: Option<u8>,
    #[serde(default)]
    pub cc: Option<u8>,
    #[serde(default)]
    pub program_change: Option<u8>,
    #[serde(default)]
    pub value: Option<u8>,
    #[serde(default)]
    pub toggle: Option<bool>,
    #[serde(default)]
    pub radio_group: Option<u8>,
    #[serde(default)]
    pub level: Option<bool>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub animation: Option<String>,
    #[serde(default)]
    pub channel: Option<u8>,
    #[serde(default)]
    pub on_long_press: Option<String>,
    #[serde(default)]
    pub values: Option<Vec<u8>>,
    #[serde(default)]
    pub reverse: Option<bool>,
    #[serde(default)]
    pub actions: Option<Vec<ActionYaml>>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ActionYaml {
    Delay {
        delay: u16,
    },
    Cc {
        cc: u8,
        value: Option<u8>,
        channel: Option<u8>,
    },
    ProgramChange {
        program_change: u8,
        channel: Option<u8>,
    },
    NoteOn {
        note: u8,
        channel: Option<u8>,
    },
}

#[derive(Deserialize, JsonSchema)]
pub struct EncoderConfig {
    pub label: String,
    #[serde(default)]
    pub cc: Option<u16>,
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
                                _ => pc::LedAnimation::Solid,
                            },
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
            }
        })
        .collect()
}

pub fn generate_schema() -> String {
    let schema = schemars::schema_for!(Setlist);
    serde_json::to_string_pretty(&schema).unwrap()
}
