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

    // OpenDeck Switch block=1: section 1=MessageType, section 2=MidiId, section 0=Type
    // OpenDeck Encoder block=2: section 3=MidiId

    pub fn button(preset: u8, comp_index: u8, cfg: &ButtonConfig) -> Vec<Vec<u8>> {
        let mut msgs = Vec::new();
        let idx = comp_index as u16;

        // TODO: preset switching in OpenDeck config is separate — for now configure preset 0 only
        if preset > 0 {
            return msgs;
        }

        if let Some(note) = cfg.note {
            // Message type = Note (0x00)
            msgs.push(opendeck_set_single(1, 1, idx, 0x00));
            // MIDI ID = note number
            msgs.push(opendeck_set_single(1, 2, idx, note as u16));
        } else if let Some(cc) = cfg.cc {
            // Message type = CC (0x02) or CC with reset (0x03) for toggle
            let msg_type = if cfg.toggle.unwrap_or(false) {
                0x03
            } else {
                0x02
            };
            msgs.push(opendeck_set_single(1, 1, idx, msg_type));
            msgs.push(opendeck_set_single(1, 2, idx, cc as u16));
        }

        // Set type = Latching if toggle
        if cfg.toggle.unwrap_or(false) {
            msgs.push(opendeck_set_single(1, 0, idx, 1)); // Latching
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
            // MIDI ID
            msgs.push(opendeck_set_single(2, 3, idx, cc));
        }

        msgs
    }
}
