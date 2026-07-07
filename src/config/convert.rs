use pedalboard_protocol::config as pc;

use super::color::parse_color;
use super::{
    ActionYaml, GlobalYamlConfig, Setlist, TriggerActionYaml, TriggerMatchYaml, TriggerYaml,
    BUTTON_KEYS, ENCODER_KEYS,
};

pub fn yaml_global_to_protocol(yaml: &GlobalYamlConfig) -> pc::GlobalConfig {
    pc::GlobalConfig {
        din_enabled: yaml.din_enabled.unwrap_or(true),
        din_to_usb_thru: yaml.din_to_usb_thru.unwrap_or(true),
        usb_to_din_thru: yaml.usb_to_din_thru.unwrap_or(false),
        usb_to_usb_thru: yaml.usb_to_usb_thru.unwrap_or(false),
        midi_clock: yaml.midi_clock.unwrap_or(false),
        bpm: yaml.bpm.unwrap_or(120),
        exp1_min: yaml
            .calibration
            .as_ref()
            .and_then(|c| c.exp1.as_ref())
            .map(|e| e.min)
            .unwrap_or(0),
        exp1_max: yaml
            .calibration
            .as_ref()
            .and_then(|c| c.exp1.as_ref())
            .map(|e| e.max)
            .unwrap_or(3750),
        exp2_min: yaml
            .calibration
            .as_ref()
            .and_then(|c| c.exp2.as_ref())
            .map(|e| e.min)
            .unwrap_or(0),
        exp2_max: yaml
            .calibration
            .as_ref()
            .and_then(|c| c.exp2.as_ref())
            .map(|e| e.max)
            .unwrap_or(3750),
    }
}

pub fn convert_actions(
    actions: &Option<Vec<ActionYaml>>,
) -> heapless::Vec<pc::Action, { pc::MAX_ACTIONS }> {
    let mut result: heapless::Vec<pc::Action, { pc::MAX_ACTIONS }> = heapless::Vec::new();
    if let Some(actions) = actions {
        for action in actions {
            let _ = match action {
                ActionYaml::Delay { delay } => result.push(pc::Action::Delay(*delay)),
                ActionYaml::Cc { cc, value, channel } => result.push(
                    pc::Action::cc(*cc, value.unwrap_or(127), channel.unwrap_or(1))
                        .expect("invalid CC: value or channel out of range"),
                ),
                ActionYaml::ProgramChange {
                    program_change,
                    channel,
                } => result.push(
                    pc::Action::program_change(*program_change, channel.unwrap_or(1))
                        .expect("invalid Program Change: program or channel out of range"),
                ),
                ActionYaml::NoteOn { note, channel } => result.push(
                    pc::Action::note_on(*note, channel.unwrap_or(1))
                        .expect("invalid Note On: note or channel out of range"),
                ),
                ActionYaml::SetLed { color, animation } => {
                    let c = parse_color(color);
                    let a = match animation.as_deref() {
                        Some("blink") => pc::LedAnimation::Blink,
                        Some("pulse") => pc::LedAnimation::Pulse,
                        Some("rotate") => pc::LedAnimation::Rotate,
                        Some("colorcycle") => pc::LedAnimation::ColorCycle,
                        _ => pc::LedAnimation::Solid,
                    };
                    result.push(pc::Action::SetLed {
                        color: c,
                        animation: a,
                    })
                }
            };
        }
    }
    result
}

fn button_key_to_index(key: &str) -> u8 {
    match key {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "D" => 3,
        "E" => 4,
        "F" => 5,
        _ => 0,
    }
}

pub fn convert_triggers(
    triggers: &Option<Vec<TriggerYaml>>,
) -> heapless::Vec<pc::Trigger, { pc::MAX_TRIGGERS }> {
    let mut result: heapless::Vec<pc::Trigger, { pc::MAX_TRIGGERS }> = heapless::Vec::new();
    if let Some(triggers) = triggers {
        for t in triggers {
            let match_msg = match &t.match_msg {
                TriggerMatchYaml::Cc {
                    cc,
                    channel,
                    value_gte,
                    value_lt,
                } => pc::TriggerMatch::Cc {
                    cc: *cc,
                    channel: *channel,
                    value_min: value_gte.unwrap_or(0),
                    value_max: value_lt.map(|v| v.saturating_sub(1)).unwrap_or(127),
                },
                TriggerMatchYaml::ProgramChange {
                    program_change,
                    channel,
                } => pc::TriggerMatch::ProgramChange {
                    program: *program_change,
                    channel: *channel,
                },
                TriggerMatchYaml::NoteOn { note, channel } => pc::TriggerMatch::NoteOn {
                    note: *note,
                    channel: *channel,
                },
            };
            let action = match &t.action {
                TriggerActionYaml::Activate { activate } => {
                    pc::TriggerAction::Activate(button_key_to_index(activate))
                }
                TriggerActionYaml::Deactivate { deactivate } => {
                    pc::TriggerAction::Deactivate(button_key_to_index(deactivate))
                }
                TriggerActionYaml::PresetSelect { preset_select } => {
                    pc::TriggerAction::PresetSelect(*preset_select)
                }
                TriggerActionYaml::Execute { execute } => {
                    pc::TriggerAction::Execute(button_key_to_index(execute))
                }
            };
            result.push(pc::Trigger { match_msg, action }).ok();
        }
    }
    result
}

pub fn yaml_to_presets(setlist: &Setlist) -> Vec<pc::Preset> {
    setlist
        .presets
        .iter()
        .map(|p| {
            let mut buttons = heapless::Vec::new();
            for key in BUTTON_KEYS {
                let btn_cfg = if let Some(btn) = p.buttons.get(*key) {
                    let mut on_press: heapless::Vec<pc::Action, { pc::MAX_ACTIONS }> =
                        heapless::Vec::new();
                    let mut on_release: heapless::Vec<pc::Action, { pc::MAX_ACTIONS }> =
                        heapless::Vec::new();

                    if let Some(actions) = &btn.actions {
                        for action in actions {
                            let _ = match action {
                                ActionYaml::Delay { delay } => {
                                    on_press.push(pc::Action::Delay(*delay))
                                }
                                ActionYaml::Cc { cc, value, channel } => on_press.push(
                                    pc::Action::cc(
                                        *cc,
                                        value.unwrap_or(127),
                                        channel.or(btn.channel).unwrap_or(1),
                                    )
                                    .expect("invalid CC: value or channel out of range"),
                                ),
                                ActionYaml::ProgramChange {
                                    program_change,
                                    channel,
                                } => on_press.push(
                                    pc::Action::program_change(
                                        *program_change,
                                        channel.or(btn.channel).unwrap_or(1),
                                    )
                                    .expect(
                                        "invalid Program Change: program or channel out of range",
                                    ),
                                ),
                                ActionYaml::NoteOn { note, channel } => on_press.push(
                                    pc::Action::note_on(
                                        *note,
                                        channel.or(btn.channel).unwrap_or(1),
                                    )
                                    .expect("invalid Note On: note or channel out of range"),
                                ),
                                ActionYaml::SetLed { color, animation } => {
                                    let c = parse_color(color);
                                    let a = match animation.as_deref() {
                                        Some("blink") => pc::LedAnimation::Blink,
                                        Some("pulse") => pc::LedAnimation::Pulse,
                                        Some("rotate") => pc::LedAnimation::Rotate,
                                        Some("colorcycle") => pc::LedAnimation::ColorCycle,
                                        _ => pc::LedAnimation::Solid,
                                    };
                                    on_press.push(pc::Action::SetLed {
                                        color: c,
                                        animation: a,
                                    })
                                }
                            };
                        }
                    } else if let Some(prog) = btn.program_change {
                        let _ = on_press.push(
                            pc::Action::program_change(prog, btn.channel.unwrap_or(1))
                                .expect("invalid Program Change: program or channel out of range"),
                        );
                    } else if let Some(cc) = btn.cc {
                        if btn.values.is_some() {
                            let _ = on_press.push(pc::Action::CcCycle {
                                cc,
                                channel: btn.channel.unwrap_or(1),
                                reverse: btn.reverse.unwrap_or(false),
                            });
                        } else if btn.toggle == Some(true) {
                            // Toggle shorthand: on_press sends value, on_release sends 0
                            let _ = on_press.push(
                                pc::Action::cc(
                                    cc,
                                    btn.value.unwrap_or(127),
                                    btn.channel.unwrap_or(1),
                                )
                                .expect("invalid CC: value or channel out of range"),
                            );
                            on_release
                                .push(
                                    pc::Action::cc(cc, 0, btn.channel.unwrap_or(1))
                                        .expect("invalid CC: channel out of range"),
                                )
                                .ok();
                        } else {
                            let _ = on_press.push(
                                pc::Action::cc(
                                    cc,
                                    btn.value.unwrap_or(127),
                                    btn.channel.unwrap_or(1),
                                )
                                .expect("invalid CC: value or channel out of range"),
                            );
                        }
                    } else if let Some(note) = btn.note {
                        let _ = on_press.push(
                            pc::Action::note_on(note, btn.channel.unwrap_or(1))
                                .expect("invalid Note On: note or channel out of range"),
                        );
                    } else if btn.tap_tempo == Some(true) {
                        let _ = on_press.push(pc::Action::TapTempo);
                    }

                    let color = btn
                        .color
                        .as_deref()
                        .map(parse_color)
                        .unwrap_or(pc::Color::Off);

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
                        on_release,
                        on_long_press: {
                            let mut lp = heapless::Vec::new();
                            match btn.on_long_press.as_deref() {
                                Some("next_preset") => {
                                    lp.push(pc::Action::PresetNext).ok();
                                }
                                Some("prev_preset") => {
                                    lp.push(pc::Action::PresetPrev).ok();
                                }
                                Some("tap_tempo") => {
                                    lp.push(pc::Action::TapTempo).ok();
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
                        listen_cc: btn.listen_cc.as_ref().map(|l| pc::ListenCc {
                            cc: l.cc,
                            channel: l.channel,
                            mode: match l.mode.as_deref() {
                                Some("trigger") => pc::ListenMode::Trigger,
                                _ => pc::ListenMode::Heatmap,
                            },
                            threshold: l.threshold.unwrap_or(64),
                        }),
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
                        listen_cc: None,
                    }
                };
                let _ = buttons.push(btn_cfg);
            }

            let mut encoders = heapless::Vec::new();
            for key in ENCODER_KEYS {
                let enc_cfg = if let Some(enc) = p.encoders.get(*key) {
                    let action = match enc.mode.as_deref() {
                        Some("relative") => pc::EncoderAction::CcRelative {
                            cc: enc.cc.unwrap_or(0) as u8,
                            channel: enc.channel.unwrap_or(1),
                            increment: enc.increment.unwrap_or(65),
                            decrement: enc.decrement.unwrap_or(63),
                        },
                        Some("preset_scroll") => pc::EncoderAction::PresetScroll,
                        _ => pc::EncoderAction::Cc {
                            cc: enc.cc.unwrap_or(0),
                            channel: enc.channel.unwrap_or(1),
                            min: enc.min.unwrap_or(0),
                            max: enc.max.unwrap_or(127),
                        },
                    };
                    pc::EncoderConfig {
                        label: pc::Label::try_from(enc.label.as_str()).unwrap_or_default(),
                        action,
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
                on_enter: convert_actions(&p.on_enter),
                on_exit: convert_actions(&p.on_exit),
                triggers: convert_triggers(&p.triggers),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_actions_none() {
        let result = convert_actions(&None);
        assert!(result.is_empty());
    }

    #[test]
    fn convert_actions_empty_vec() {
        let result = convert_actions(&Some(vec![]));
        assert!(result.is_empty());
    }

    #[test]
    fn convert_actions_cc() {
        let actions = vec![ActionYaml::Cc {
            cc: 10,
            value: Some(100),
            channel: Some(2),
        }];
        let result = convert_actions(&Some(actions));
        assert_eq!(result.len(), 1);
        // CC actions are encoded as Midi { data: [0xBn, cc, value], len: 3 }
        match &result[0] {
            pc::Action::Midi { data, len } => {
                assert_eq!(*len, 3);
                assert_eq!(data[0], 0xB0 | 1); // channel 2 → index 1
                assert_eq!(data[1], 10);
                assert_eq!(data[2], 100);
            }
            other => panic!("Expected Midi (CC) action, got {:?}", other),
        }
    }

    #[test]
    fn convert_actions_cc_defaults() {
        let actions = vec![ActionYaml::Cc {
            cc: 5,
            value: None,
            channel: None,
        }];
        let result = convert_actions(&Some(actions));
        assert_eq!(result.len(), 1);
        // Defaults: value=127, channel=1
        match &result[0] {
            pc::Action::Midi { data, len } => {
                assert_eq!(*len, 3);
                assert_eq!(data[0], 0xB0); // channel 1 → index 0
                assert_eq!(data[1], 5);
                assert_eq!(data[2], 127);
            }
            other => panic!("Expected Midi (CC) action, got {:?}", other),
        }
    }

    #[test]
    fn convert_actions_program_change() {
        let actions = vec![ActionYaml::ProgramChange {
            program_change: 5,
            channel: Some(3),
        }];
        let result = convert_actions(&Some(actions));
        assert_eq!(result.len(), 1);
        // PC actions are encoded as Midi { data: [0xCn, program, 0], len: 2 }
        match &result[0] {
            pc::Action::Midi { data, len } => {
                assert_eq!(*len, 2);
                assert_eq!(data[0], 0xC0 | 2); // channel 3 → index 2
                assert_eq!(data[1], 5);
            }
            other => panic!("Expected Midi (PC) action, got {:?}", other),
        }
    }

    #[test]
    fn convert_actions_note_on() {
        let actions = vec![ActionYaml::NoteOn {
            note: 60,
            channel: Some(1),
        }];
        let result = convert_actions(&Some(actions));
        assert_eq!(result.len(), 1);
        // Note On actions are encoded as Midi { data: [0x9n, note, 127], len: 3 }
        match &result[0] {
            pc::Action::Midi { data, len } => {
                assert_eq!(*len, 3);
                assert_eq!(data[0], 0x90); // channel 1 → index 0
                assert_eq!(data[1], 60);
                assert_eq!(data[2], 127);
            }
            other => panic!("Expected Midi (NoteOn) action, got {:?}", other),
        }
    }

    #[test]
    fn convert_actions_delay() {
        let actions = vec![ActionYaml::Delay { delay: 250 }];
        let result = convert_actions(&Some(actions));
        assert_eq!(result.len(), 1);
        match &result[0] {
            pc::Action::Delay(250) => {}
            other => panic!("Expected Delay(250), got {:?}", other),
        }
    }

    #[test]
    fn convert_actions_set_led() {
        let actions = vec![ActionYaml::SetLed {
            color: "red".to_string(),
            animation: Some("blink".to_string()),
        }];
        let result = convert_actions(&Some(actions));
        assert_eq!(result.len(), 1);
        match &result[0] {
            pc::Action::SetLed { color, animation } => {
                assert_eq!(*color, pc::Color::Red);
                assert_eq!(*animation, pc::LedAnimation::Blink);
            }
            other => panic!("Expected SetLed action, got {:?}", other),
        }
    }

    #[test]
    fn convert_actions_multiple() {
        let actions = vec![
            ActionYaml::Cc {
                cc: 1,
                value: Some(127),
                channel: Some(1),
            },
            ActionYaml::Delay { delay: 100 },
            ActionYaml::ProgramChange {
                program_change: 3,
                channel: None,
            },
        ];
        let result = convert_actions(&Some(actions));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn convert_triggers_none() {
        let result = convert_triggers(&None);
        assert!(result.is_empty());
    }

    #[test]
    fn convert_triggers_cc_activate() {
        let triggers = vec![TriggerYaml {
            match_msg: TriggerMatchYaml::Cc {
                cc: 80,
                channel: 1,
                value_gte: Some(64),
                value_lt: None,
            },
            action: TriggerActionYaml::Activate {
                activate: "A".to_string(),
            },
        }];
        let result = convert_triggers(&Some(triggers));
        assert_eq!(result.len(), 1);
        match &result[0].match_msg {
            pc::TriggerMatch::Cc {
                cc,
                channel,
                value_min,
                value_max,
            } => {
                assert_eq!(*cc, 80);
                assert_eq!(*channel, 1);
                assert_eq!(*value_min, 64);
                assert_eq!(*value_max, 127);
            }
            other => panic!("Expected Cc trigger match, got {:?}", other),
        }
        match &result[0].action {
            pc::TriggerAction::Activate(idx) => assert_eq!(*idx, 0), // "A" = 0
            other => panic!("Expected Activate action, got {:?}", other),
        }
    }

    #[test]
    fn convert_triggers_cc_with_value_lt() {
        let triggers = vec![TriggerYaml {
            match_msg: TriggerMatchYaml::Cc {
                cc: 80,
                channel: 1,
                value_gte: None,
                value_lt: Some(64),
            },
            action: TriggerActionYaml::Deactivate {
                deactivate: "B".to_string(),
            },
        }];
        let result = convert_triggers(&Some(triggers));
        assert_eq!(result.len(), 1);
        match &result[0].match_msg {
            pc::TriggerMatch::Cc {
                value_min,
                value_max,
                ..
            } => {
                assert_eq!(*value_min, 0);
                assert_eq!(*value_max, 63); // value_lt - 1
            }
            other => panic!("Expected Cc trigger match, got {:?}", other),
        }
        match &result[0].action {
            pc::TriggerAction::Deactivate(idx) => assert_eq!(*idx, 1), // "B" = 1
            other => panic!("Expected Deactivate action, got {:?}", other),
        }
    }

    #[test]
    fn convert_triggers_program_change() {
        let triggers = vec![TriggerYaml {
            match_msg: TriggerMatchYaml::ProgramChange {
                program_change: 5,
                channel: 2,
            },
            action: TriggerActionYaml::PresetSelect { preset_select: 3 },
        }];
        let result = convert_triggers(&Some(triggers));
        assert_eq!(result.len(), 1);
        match &result[0].match_msg {
            pc::TriggerMatch::ProgramChange { program, channel } => {
                assert_eq!(*program, 5);
                assert_eq!(*channel, 2);
            }
            other => panic!("Expected ProgramChange trigger match, got {:?}", other),
        }
        match &result[0].action {
            pc::TriggerAction::PresetSelect(idx) => assert_eq!(*idx, 3),
            other => panic!("Expected PresetSelect action, got {:?}", other),
        }
    }

    #[test]
    fn convert_triggers_note_on_execute() {
        let triggers = vec![TriggerYaml {
            match_msg: TriggerMatchYaml::NoteOn {
                note: 60,
                channel: 1,
            },
            action: TriggerActionYaml::Execute {
                execute: "C".to_string(),
            },
        }];
        let result = convert_triggers(&Some(triggers));
        assert_eq!(result.len(), 1);
        match &result[0].match_msg {
            pc::TriggerMatch::NoteOn { note, channel } => {
                assert_eq!(*note, 60);
                assert_eq!(*channel, 1);
            }
            other => panic!("Expected NoteOn trigger match, got {:?}", other),
        }
        match &result[0].action {
            pc::TriggerAction::Execute(idx) => assert_eq!(*idx, 2), // "C" = 2
            other => panic!("Expected Execute action, got {:?}", other),
        }
    }

    #[test]
    fn button_key_to_index_all_keys() {
        assert_eq!(button_key_to_index("A"), 0);
        assert_eq!(button_key_to_index("B"), 1);
        assert_eq!(button_key_to_index("C"), 2);
        assert_eq!(button_key_to_index("D"), 3);
        assert_eq!(button_key_to_index("E"), 4);
        assert_eq!(button_key_to_index("F"), 5);
        assert_eq!(button_key_to_index("unknown"), 0);
    }

    #[test]
    fn yaml_to_presets_encoder_modes() {
        let yaml = r#"
presets:
  - name: "Encoder Test"
    encoders:
      Vol:
        label: "Volume"
        mode: "cc"
        cc: 7
        channel: 1
        min: 0
        max: 100
      Gain:
        label: "Scroll"
        mode: "preset_scroll"
"#;
        let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
        let presets = yaml_to_presets(&setlist);
        assert_eq!(presets.len(), 1);
        assert_eq!(presets[0].encoders.len(), 2);

        match &presets[0].encoders[0].action {
            pc::EncoderAction::Cc {
                cc,
                channel,
                min,
                max,
            } => {
                assert_eq!(*cc, 7);
                assert_eq!(*channel, 1);
                assert_eq!(*min, 0);
                assert_eq!(*max, 100);
            }
            other => panic!("Expected Cc encoder action, got {:?}", other),
        }

        match &presets[0].encoders[1].action {
            pc::EncoderAction::PresetScroll => {}
            other => panic!("Expected PresetScroll encoder action, got {:?}", other),
        }
    }

    #[test]
    fn yaml_to_presets_encoder_relative_mode() {
        let yaml = r#"
presets:
  - name: "Relative Test"
    encoders:
      Vol:
        label: "Pan"
        mode: "relative"
        cc: 10
        channel: 1
        increment: 65
        decrement: 63
"#;
        let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
        let presets = yaml_to_presets(&setlist);
        assert_eq!(presets.len(), 1);

        match &presets[0].encoders[0].action {
            pc::EncoderAction::CcRelative {
                cc,
                channel,
                increment,
                decrement,
            } => {
                assert_eq!(*cc, 10);
                assert_eq!(*channel, 1);
                assert_eq!(*increment, 65);
                assert_eq!(*decrement, 63);
            }
            other => panic!("Expected CcRelative encoder action, got {:?}", other),
        }
    }

    #[test]
    fn yaml_to_presets_button_modes() {
        let yaml = r#"
presets:
  - name: "Button Mode Test"
    buttons:
      A:
        label: "Momentary"
        cc: 10
        channel: 1
      B:
        label: "Toggle"
        cc: 11
        channel: 1
        toggle: true
      C:
        label: "Radio"
        cc: 12
        channel: 1
        radio_group: 1
"#;
        let setlist: Setlist = serde_yaml::from_str(yaml).unwrap();
        let presets = yaml_to_presets(&setlist);
        assert_eq!(presets.len(), 1);

        // Button A (index 0) = Momentary
        assert_eq!(presets[0].buttons[0].mode, pc::ButtonMode::Momentary);
        // Button B (index 1) = Toggle
        assert_eq!(presets[0].buttons[1].mode, pc::ButtonMode::Toggle);
        // Button C (index 2) = RadioGroup(1)
        assert_eq!(presets[0].buttons[2].mode, pc::ButtonMode::RadioGroup(1));
    }
}
