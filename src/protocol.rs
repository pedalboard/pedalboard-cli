/// SysEx encoding helpers and message builders.

fn split14bit(value: u16) -> (u8, u8) {
    let mut high = ((value >> 8) & 0xFF) as u8;
    let mut low = (value & 0xFF) as u8;
    high = (high << 1) & 0x7F;
    if (low >> 7) & 0x01 != 0 {
        high |= 0x01;
    } else {
        high &= !0x01;
    }
    low &= 0x7F;
    (high, low)
}

/// Build a label SET SINGLE message (M_ID_2 = 0x44).
fn label_set_single(block: u8, section: u8, raw_index: u16, value: u8) -> Vec<u8> {
    let (idx_h, idx_l) = split14bit(raw_index);
    let (val_h, val_l) = split14bit(value as u16);
    vec![
        0xF0, 0x00, 0x53, 0x44, 0x00, 0x00, 0x01, 0x00, block, section, idx_h, idx_l, val_h,
        val_l, 0xF7,
    ]
}

/// Build an OpenDeck SET SINGLE message (M_ID_2 = 0x43).
fn opendeck_set_single(block: u8, section: u8, index: u16, value: u16) -> Vec<u8> {
    let (idx_h, idx_l) = split14bit(index);
    let (val_h, val_l) = split14bit(value);
    vec![
        0xF0, 0x00, 0x53, 0x43, 0x00, 0x00, 0x01, 0x00, block, section, idx_h, idx_l, val_h,
        val_l, 0xF7,
    ]
}

// Label INDEX layout:
// Switch:  preset * 6 * 16 + comp_index * 16 + char_pos
// Encoder: preset * 2 * 16 + comp_index * 16 + char_pos
// Preset name: preset_index * 16 + char_pos

pub mod label_set_messages {
    use super::*;

    pub fn preset_name(preset: u8, name: &str) -> Vec<Vec<u8>> {
        let mut msgs = Vec::new();
        for (i, ch) in name.bytes().enumerate().take(16) {
            let idx = preset as u16 * 16 + i as u16;
            msgs.push(label_set_single(0x00, 0x06, idx, ch));
        }
        // Zero-terminate
        let idx = preset as u16 * 16 + name.len().min(15) as u16;
        msgs.push(label_set_single(0x00, 0x06, idx, 0));
        msgs
    }

    pub fn button(preset: u8, comp_index: u8, label: &str) -> Vec<Vec<u8>> {
        let mut msgs = Vec::new();
        for (i, ch) in label.bytes().enumerate().take(16) {
            let idx = preset as u16 * 6 * 16 + comp_index as u16 * 16 + i as u16;
            msgs.push(label_set_single(0x01, 0x05, idx, ch));
        }
        let idx = preset as u16 * 6 * 16 + comp_index as u16 * 16 + label.len().min(15) as u16;
        msgs.push(label_set_single(0x01, 0x05, idx, 0));
        msgs
    }

    pub fn encoder(preset: u8, comp_index: u8, label: &str) -> Vec<Vec<u8>> {
        let mut msgs = Vec::new();
        for (i, ch) in label.bytes().enumerate().take(16) {
            let idx = preset as u16 * 2 * 16 + comp_index as u16 * 16 + i as u16;
            msgs.push(label_set_single(0x02, 0x0D, idx, ch));
        }
        let idx = preset as u16 * 2 * 16 + comp_index as u16 * 16 + label.len().min(15) as u16;
        msgs.push(label_set_single(0x02, 0x0D, idx, 0));
        msgs
    }
}

pub mod opendeck_set_messages {
    use super::*;
    use crate::{ButtonConfig, EncoderConfig};

    // OpenDeck Switch block=1: section 0=Type, 1=MessageType, 2=MidiId
    // OpenDeck Output block=4: section 5=ControlType, 3=ActivationId, 6=ActivationValue, 7=Channel

    fn color_to_value(color: &str) -> u16 {
        match color.to_lowercase().as_str() {
            "red" => 1,
            "green" => 2,
            "yellow" => 3,
            "blue" => 4,
            "magenta" => 5,
            "cyan" => 6,
            "white" => 7,
            _ => 2, // default green
        }
    }

    // LED control types
    const LOCAL_NOTE_SINGLE: u16 = 1;
    const LOCAL_CC_SINGLE: u16 = 3;

    pub fn button(preset: u8, comp_index: u8, cfg: &ButtonConfig) -> Vec<Vec<u8>> {
        let mut msgs = Vec::new();
        let idx = comp_index as u16;

        // TODO: per-preset MIDI config requires OpenDeck preset switching
        if preset > 0 {
            return msgs;
        }

        if let Some(note) = cfg.note {
            msgs.push(opendeck_set_single(1, 1, idx, 0x00)); // MessageType = Note
            msgs.push(opendeck_set_single(1, 2, idx, note as u16)); // MidiId
        } else if let Some(cc) = cfg.cc {
            let msg_type = if cfg.toggle.unwrap_or(false) { 0x03 } else { 0x02 };
            msgs.push(opendeck_set_single(1, 1, idx, msg_type));
            msgs.push(opendeck_set_single(1, 2, idx, cc as u16));
        }

        if cfg.toggle.unwrap_or(false) {
            msgs.push(opendeck_set_single(1, 0, idx, 1)); // Type = Latching
        }

        // LED config: button index maps to LED index
        if cfg.color.is_some() || cfg.note.is_some() || cfg.cc.is_some() {
            let led_idx = idx;
            // Control type
            let control_type = if cfg.note.is_some() {
                LOCAL_NOTE_SINGLE
            } else {
                LOCAL_CC_SINGLE
            };
            msgs.push(opendeck_set_single(4, 5, led_idx, control_type));

            // Activation ID = note or CC number
            let act_id = cfg.note.unwrap_or(0) | cfg.cc.unwrap_or(0);
            msgs.push(opendeck_set_single(4, 3, led_idx, act_id as u16));

            // Activation value
            msgs.push(opendeck_set_single(4, 6, led_idx, 127));

            // Color (stored separately — uses a custom section or we skip for now)
            // For now, color is applied via the firmware's set_output_color
            // which isn't exposed via standard OpenDeck protocol.
            // We'll handle color through the label protocol extension later.
        }

        msgs
    }

    pub fn encoder(preset: u8, comp_index: u8, cfg: &EncoderConfig) -> Vec<Vec<u8>> {
        let mut msgs = Vec::new();
        let idx = comp_index as u16;

        if preset > 0 {
            return msgs;
        }

        if let Some(cc) = cfg.cc {
            msgs.push(opendeck_set_single(2, 3, idx, cc)); // MIDI ID
        }

        msgs
    }
}
