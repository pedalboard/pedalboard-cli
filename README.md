# pedalboard-cli

CLI tool for uploading configuration to the Pedalboard MIDI controller.

## Usage

```sh
pedalboard-cli pe-upload <config.yaml>
pedalboard-cli pe-read <index>
pedalboard-cli reboot
pedalboard-cli bootloader
pedalboard-cli reset
```

The default WebSocket address is `ws://cm5-dev.home/config`. Override it with `--address`:

```sh
pedalboard-cli --address ws://myhost:8080/config pe-upload <setlist.yaml>
```

## Configuration

Presets are defined in YAML. See [docs/config-reference.md](docs/config-reference.md) for the full schema reference.

Quick example:

```yaml
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

More examples in [examples/](examples/).

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
