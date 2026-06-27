# pedalboard-cli

CLI tool for uploading configuration to the Pedalboard MIDI controller.

## Usage

```sh
pedalboard-cli pe-upload <config.yaml>
pedalboard-cli upload <setlist.yaml>
```

The default WebSocket address is `ws://cm5-dev.home/config`. Override it with `--address`:

```sh
pedalboard-cli --address ws://myhost:8080/config upload <setlist.yaml>
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
