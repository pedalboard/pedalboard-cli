# Setlist Configuration Reference

A setlist YAML file defines everything your pedalboard does: button behavior, encoder mapping, expression pedal routing, LED appearance, and MIDI routing. Upload it with `pedalboard-cli upload setlist.yaml`.

## File Structure

```yaml
global:         # Optional. Device-wide settings (MIDI routing, clock, calibration).
  ...

presets:        # Required. List of presets (max 32).
  - name: "..."
    buttons: { ... }
    encoders: { ... }
    analog: { ... }
    defaults: { ... }
    on_enter: [ ... ]
    on_exit: [ ... ]
    triggers: [ ... ]
```

---

## Global Config

System-wide settings independent of presets. Applied once on upload.

```yaml
global:
  din_enabled: true       # DIN MIDI output for locally-generated messages (default: true)
  din_to_usb_thru: true   # Route DIN MIDI input → USB MIDI out (default: true)
  usb_to_din_thru: false  # Route USB MIDI input → DIN MIDI out (default: false)
  usb_to_usb_thru: false  # Echo USB MIDI input back to USB out (default: false)
  midi_clock: false       # Enable MIDI Clock output at 24 PPQ (default: false)
  bpm: 120                # Clock tempo, 30-300 BPM (default: 120)
  calibration:            # Expression pedal ADC calibration
    exp1:
      min: 0              # ADC value at heel/rest position (default: 0)
      max: 3750           # ADC value at toe/full position (default: 3750)
    exp2:
      min: 0
      max: 3750
```

All fields are optional. Omitted values use defaults.

---

## Presets

Each preset defines the complete button/encoder/expression layout for one song or scene. Switching presets changes everything at once.

```yaml
presets:
  - name: "My Preset"    # Required. Shown on OLED (max 16 chars).
    buttons: { ... }
    encoders: { ... }
    analog: { ... }
```

---

## Buttons

Six buttons (A–F) with configurable MIDI output, LED appearance, and behavior mode.

Keys: `A`, `B`, `C`, `D`, `E`, `F`

### Simple Shortcuts

For common single-message buttons, use the shorthand fields:

```yaml
buttons:
  A: { label: "Drive", cc: 80, color: green }              # CC#80 = 127 on press
  B: { label: "Amp 2", program_change: 5, color: blue }    # PC 5 on press
  C: { label: "Tap", note: 64, color: yellow }             # Note On press, Note Off release
```

| Field | Type | Description |
|-------|------|-------------|
| `label` | string | **Required.** OLED display label (max 16 chars). Empty string = hidden. |
| `cc` | 0-127 | Send CC on press. Combine with `toggle`, `values`, or `value`. |
| `note` | 0-127 | Send Note On (vel 127) on press, Note Off on release. Implies momentary. |
| `program_change` | 0-127 | Send Program Change on press. |
| `value` | 0-127 | CC value to send (default: 127). For toggle, this is the ON value. |
| `channel` | 1-16 | MIDI channel (default: 1). Applies to all actions on this button. |

### Button Modes

| Mode | YAML | Behavior |
|------|------|----------|
| **Momentary** | (default) | Active while pressed. LED on during press. Fires `on_press` on press, `on_release` on release. |
| **Toggle** | `toggle: true` | Alternates active/inactive on each press. LED stays lit while active. Press when inactive → fires `on_press`. Press when active → fires `on_release`. |
| **Radio Group** | `radio_group: N` | Only one button in group N can be active. Pressing one deactivates others (silently — no `on_release` for deactivated buttons). |

```yaml
buttons:
  # Toggle: CC#80=127 when activated, CC#80=0 when deactivated
  A: { label: "Drive", cc: 80, toggle: true, color: green }

  # Radio group: only one board active at a time
  B: { label: "Board 1", program_change: 0, color: blue, radio_group: 1 }
  C: { label: "Board 2", program_change: 1, color: green, radio_group: 1 }
  D: { label: "Board 3", program_change: 2, color: cyan, radio_group: 1 }
```

### CC Cycle

Send a different CC value on each press, cycling through a list:

```yaml
buttons:
  A:
    label: "Kit+"
    cc: 8
    values: [0, 8, 17, 26, 35, 43, 51]   # Cycles forward through these values
    color: cyan
  B:
    label: "Kit-"
    cc: 8
    values: [0, 8, 17, 26, 35, 43, 51]
    reverse: true                           # Cycles backward
    color: red
```

| Field | Type | Description |
|-------|------|-------------|
| `values` | list of 0-127 | CC values to cycle through (max 12). Requires `cc` field. |
| `reverse` | bool | Cycle backward through the list (default: false). |

**Behavior:** First press sends `values[0]`, advances index. Second press sends `values[1]`, etc. After the last value, wraps to the beginning. With `reverse: true`, the index decrements (wrapping from 0 to the last entry). Cycle state persists across power cycles via EEPROM.

### Long Press

Hold a button for >500ms to trigger a secondary action. Short press still fires the normal action.

```yaml
buttons:
  A:
    label: "FX1"
    cc: 20
    color: yellow
    on_long_press: next_preset    # Hold > 500ms → switch to next preset
```

| Value | Action |
|-------|--------|
| `next_preset` | Switch to the next preset |
| `prev_preset` | Switch to the previous preset |

When a button has `on_long_press`, the normal `on_press` is deferred until release (if released before 500ms). If held past 500ms, only the long-press action fires.

### Multi-Action Sequences

For complex button behavior, use `actions` — a list of MIDI messages executed in order:

```yaml
buttons:
  F:
    label: "Clear"
    color: red
    actions:
      - { cc: 3, value: 127 }     # CC#3 = 127
      - { delay: 50 }             # Wait 50ms
      - { cc: 3, value: 0 }       # CC#3 = 0
```

`actions` overrides the simple `cc`/`note`/`program_change` fields.

#### Action Types

| Action | Fields | Description |
|--------|--------|-------------|
| CC | `cc`, `value?`, `channel?` | Send Control Change (value default: 127) |
| Program Change | `program_change`, `channel?` | Send Program Change |
| Note On | `note`, `channel?` | Send Note On (velocity 127) |
| Delay | `delay` | Wait N milliseconds before next action |

Per-action `channel` overrides the button-level `channel`. If both omitted, defaults to channel 1.

### LED Configuration

```yaml
buttons:
  A:
    label: "FX"
    cc: 80
    toggle: true
    color: green          # Color when active
    animation: pulse      # Animation when active
    renderer: dots        # Spatial pattern
    renderer_param: 3     # 3 evenly-spaced dots
```

| Field | Values | Description |
|-------|--------|-------------|
| `color` | `red`, `green`, `blue`, `yellow`, `cyan`, `magenta`, `white`, `orange`, `purple`, `off`, or `#RRGGBB` | LED color when active |
| `animation` | `solid`, `blink`, `pulse`, `rotate`, `colorcycle` | Temporal animation (default: solid) |
| `renderer` | `solid`, `fill`, `single`, `dots` | Spatial pattern across 12 LEDs (default: solid) |
| `renderer_param` | integer | Fill count (1-12), single position (0-11), or dot count (1-6) |

LED is off when the button is inactive. For momentary buttons, the LED is on only while pressed.

### Reactive LED (listen_cc)

Make a button's LED ring respond to incoming MIDI CC (from external gear):

```yaml
buttons:
  A:
    label: "Gain"
    cc: 80
    toggle: true
    color: green
    listen_cc:
      cc: 100             # React to incoming CC#100
      channel: 1          # On channel 1 (default: 1)
      mode: heatmap       # "heatmap" (fill proportional to value) or "trigger" (on/off)
      threshold: 64       # For trigger mode: value ≥ 64 = on (default: 64)
```

| Mode | Behavior |
|------|----------|
| `heatmap` | LED ring fill level proportional to CC value (0=off, 127=all 12 LEDs) |
| `trigger` | LED on (with button's color/animation) when value ≥ threshold, off below |

---

## Encoders

Two rotary encoders: `Vol` (left) and `Gain` (right). Each click sends an incremented/decremented CC value.

```yaml
encoders:
  Vol: { label: "Vol", cc: 7 }
  Gain: { label: "Reverb", cc: 91, channel: 2 }
```

| Field | Type | Description |
|-------|------|-------------|
| `label` | string | **Required.** Shown on OLED overlay when turning. |
| `cc` | 0-127 | CC number to send. Each detent increments/decrements the value by 1. |
| `channel` | 1-16 | MIDI channel (default: 1). |

**Behavior:** Encoder values are absolute (0-127), clamped at both ends. Turning clockwise sends the new value; turning fast (acceleration) skips multiple steps but still sends a single message with the final value. Encoder values persist across preset switches and power cycles.

The OLED shows a large value overlay briefly on each turn.

---

## Expression Pedals (Analog)

Two expression pedal inputs: `Exp1` and `Exp2`. Send continuous CC based on pedal position.

```yaml
analog:
  Exp1: { label: "Wah", cc: 11 }
  Exp2: { label: "Volume", cc: 7, min: 0, max: 100 }
```

| Field | Type | Description |
|-------|------|-------------|
| `label` | string | **Required.** Shown on OLED overlay when moving. |
| `cc` | 0-127 | **Required.** CC number to send. |
| `channel` | 1-16 | MIDI channel (default: 1). |
| `min` | 0-127 | CC value at heel/rest position (default: 0). |
| `max` | 0-127 | CC value at toe/full position (default: 127). |

**Behavior:** The raw ADC reading is clamped to the calibrated range (from `global.calibration`), then linearly mapped to the `min`–`max` CC range. Values below `adc_min` produce `min`; values above `adc_max` produce `max`.

---

## Defaults (Initial State)

Set the starting state for toggles and encoders when a preset is first activated after upload:

```yaml
defaults:
  buttons:
    B: "on"       # Button B starts active (LED on, as if pressed once)
    D: "on"
  encoders:
    Vol: 100      # Vol encoder starts at CC value 100
    Gain: 64
```

Omitted buttons default to off. Omitted encoders default to 0. After initial activation, runtime state is preserved in EEPROM across power cycles.

---

## on_enter / on_exit

MIDI messages sent automatically on preset transitions:

```yaml
on_enter:                            # Fired when this preset becomes active
  - { program_change: 0, channel: 2 }
  - { cc: 7, value: 100 }

on_exit:                             # Fired when leaving this preset
  - { cc: 123, value: 0 }           # All Notes Off
```

Uses the same action format as button `actions`. Fired in order, supports delays.

`on_enter` fires on:
- Switching to this preset (via encoder scroll, long-press, or trigger)
- Boot (for the active preset at power-on)

`on_exit` fires before switching away to another preset.

---

## Triggers

React to incoming MIDI messages by changing button state or switching presets. Useful for bidirectional sync with external gear.

```yaml
triggers:
  # When multi-FX sends CC#100 ≥ 64 on ch1, light up button A
  - match: { cc: 100, channel: 1, value_gte: 64 }
    action: { activate: "A" }

  # When CC#100 < 64, turn off button A
  - match: { cc: 100, channel: 1, value_gte: 0, value_lt: 64 }
    action: { deactivate: "A" }

  # When receiving PC 5 on ch1, switch to preset 2
  - match: { program_change: 5, channel: 1 }
    action: { preset_select: 2 }

  # When receiving Note On 60 on ch1, fire button C's on_press actions
  - match: { note: 60, channel: 1 }
    action: { execute: "C" }
```

### Match Patterns

| Pattern | Fields | Description |
|---------|--------|-------------|
| CC | `cc`, `channel`, `value_gte?`, `value_lt?` | Match Control Change. Optional value range (default: 0-127). |
| Program Change | `program_change`, `channel` | Match Program Change number. |
| Note On | `note`, `channel` | Match Note On (velocity > 0 only; velocity 0 = Note Off, not matched). |

`value_gte` = value must be ≥ this (default: 0). `value_lt` = value must be < this (default: 128, i.e., ≤ 127).

### Actions

| Action | Format | Description |
|--------|--------|-------------|
| Activate | `{ activate: "A" }` | Set button active (LED on). No outgoing MIDI. |
| Deactivate | `{ deactivate: "B" }` | Set button inactive (LED off). No outgoing MIDI. |
| Preset Select | `{ preset_select: 0 }` | Switch to preset by index (0-based). |
| Execute | `{ execute: "C" }` | Fire the button's `on_press` actions as if physically pressed. |

Multiple triggers can match the same incoming message — all matching triggers fire.

---

## Complete Example

```yaml
global:
  din_enabled: true
  midi_clock: false
  calibration:
    exp1: { min: 180, max: 3700 }

presets:
  - name: "Live FX"
    defaults:
      buttons: { A: "on" }
      encoders: { Vol: 100 }
    on_enter:
      - { program_change: 0, channel: 2 }
    buttons:
      A: { label: "Board 1", program_change: 0, channel: 2, color: blue, radio_group: 1 }
      B: { label: "Board 2", program_change: 1, channel: 2, color: green, radio_group: 1 }
      C: { label: "Bypass", cc: 0, channel: 3, toggle: true, color: red, animation: blink }
      D: { label: "Drive", cc: 80, toggle: true, color: green, on_long_press: prev_preset }
      E: { label: "", color: off }
      F: { label: "Next", cc: 99, color: magenta, on_long_press: next_preset }
    encoders:
      Vol: { label: "HotKnob1", cc: 107, channel: 2 }
      Gain: { label: "HotKnob2", cc: 108, channel: 2 }
    analog:
      Exp1: { label: "Volume", cc: 7 }
    triggers:
      - match: { cc: 80, channel: 1, value_gte: 64 }
        action: { activate: "D" }
      - match: { cc: 80, channel: 1, value_lt: 64 }
        action: { deactivate: "D" }
```

---

## Limits

| Item | Maximum |
|------|---------|
| Presets | 32 |
| Buttons per preset | 6 (A–F) |
| Encoders per preset | 2 (Vol, Gain) |
| Expression pedals | 2 (Exp1, Exp2) |
| Actions per button | 8 |
| Cycle values per button | 12 |
| Triggers per preset | 8 |
| Label length | 16 characters |
| Preset name length | 16 characters |
