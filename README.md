# pedalboard-cli

CLI tool for configuring the Pedalboard MIDI controller.

## Usage

```sh
pedalboard-cli upload <setlist.yaml>    # upload presets + global config
pedalboard-cli read <index>             # read back a preset
pedalboard-cli monitor                  # real-time MIDI output display
pedalboard-cli flash <firmware.uf2>     # flash firmware
pedalboard-cli reboot                   # reboot device
pedalboard-cli reset                    # factory reset
pedalboard-cli bootloader               # enter UF2 bootloader
```

The default WebSocket address is `ws://cm5-dev.home/config`. Override with `--address`:

```sh
pedalboard-cli --address ws://myhost:8080/config upload <setlist.yaml>
```

## Install

```sh
make install   # builds release and copies to ~/.cargo/bin/
```

## Configuration

Setlists are defined in YAML with an optional `global:` section and one or more presets.

```yaml
global:
  din_enabled: true
  din_to_usb_thru: true
  usb_to_din_thru: false
  midi_clock: false
  bpm: 120
  calibration:
    exp1: { min: 180, max: 3700 }
    exp2: { min: 200, max: 3750 }

presets:
  - name: "My Song"
    defaults:
      buttons: { B: "on" }
      encoders: { Vol: 100 }
    buttons:
      A: { label: "Verse", note: 60, color: green }
      B: { label: "Chorus", cc: 80, toggle: true, color: blue, animation: blink }
      C: { label: "FX", cc: 20, value: 127, color: yellow, on_long_press: next_preset }
    encoders:
      Vol: { label: "Vol", cc: 7 }
      Gain: { label: "Reverb", cc: 91 }
    analog:
      Exp1: { label: "Wah", cc: 11 }
```

See [schema/pedalboard.schema.json](schema/pedalboard.schema.json) for the full JSON Schema reference.
See [docs/config-reference.md](docs/config-reference.md) for the human-readable configuration guide.
More examples in [examples/](examples/).

## Development

When extending the config schema (new actions, fields, button modes), follow the
[contributing guide](https://github.com/pedalboard/midi-controller/blob/main/CONTRIBUTING.md)
to keep protocol, CLI, and firmware in sync.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
