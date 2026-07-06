#!/bin/bash
# Audio integration tests: verifies the full audio processing pipeline.
# Requires: JACK + mod-host + bridge running on CM5 (via pedalboard-os).
#
# Usage: ./tests/audio-integration.sh [bridge_host]
set -e

HOST="${1:-cm5-dev.home}"
BRIDGE="ws://$HOST:8080"

echo "=== Audio Integration Tests (host: $HOST) ==="

# Test 1: JACK is running with correct config
echo -n "1. JACK running (48kHz, 64 frames, 2ch)... "
result=$(ssh laenzi@$HOST "jack_samplerate 2>/dev/null && jack_bufsize 2>/dev/null && jack_lsp 2>/dev/null | grep 'system:' | wc -l" 2>/dev/null)
SR=$(echo "$result" | sed -n '1p')
BUF=$(echo "$result" | sed -n '2p')
PORTS=$(echo "$result" | sed -n '3p')
if [[ "$SR" == "48000" ]] && [[ "$BUF" == "64" ]] && [[ "$PORTS" == "4" ]]; then
  echo "✓"
else
  echo "✗ (sr=$SR buf=$BUF ports=$PORTS)"
  exit 1
fi

# Test 2: mod-host is responding
echo -n "2. mod-host responding on port 5555... "
result=$(ssh laenzi@$HOST "python3 -c \"
import socket
s = socket.socket()
s.settimeout(2)
s.connect(('localhost', 5555))
s.sendall(b'param_get 0 PREGAIN\n')
import time; time.sleep(0.5)
r = s.recv(256)
print('ok' if b'resp' in r else 'bad')
s.close()
\"" 2>/dev/null)
if [[ "$result" == "ok" ]]; then
  echo "✓"
else
  echo "✗ ($result)"
  exit 1
fi

# Test 3: AIDA-X plugin loaded in JACK
echo -n "3. AIDA-X plugin loaded... "
result=$(ssh laenzi@$HOST "jack_lsp 2>/dev/null | grep 'effect_0:lv2_audio'" 2>/dev/null)
if [[ -n "$result" ]]; then
  echo "✓"
else
  echo "✗ (no effect_0 ports)"
  exit 1
fi

# Test 4: Audio routing is correct (capture → plugin → playback)
echo -n "4. Audio routing (capture_2 → AIDA-X → playback_2)... "
IN=$(ssh laenzi@$HOST "jack_lsp -c 2>/dev/null" | grep -A1 "effect_0:lv2_audio_in_1" | grep "capture_2")
OUT=$(ssh laenzi@$HOST "jack_lsp -c 2>/dev/null" | grep -A1 "effect_0:lv2_audio_out_1" | grep "playback_2")
if [[ -n "$IN" ]] && [[ -n "$OUT" ]]; then
  echo "✓"
else
  echo "✗"
  exit 1
fi

# Test 5: Bridge WebSocket reachable
echo -n "5. Bridge reachable on port 8080... "
HTTP=$(curl -s -o /dev/null -w "%{http_code}" "http://$HOST:8080/raw" 2>/dev/null)
if [[ "$HTTP" == "400" ]]; then
  echo "✓ (WebSocket upgrade expected)"
else
  echo "✗ (http=$HTTP)"
  exit 1
fi

# Test 6: Audio patch switching via Program Change
echo -n "6. PC switches audio patch... "
# Upload a preset that sends PC on button press
CLI="cargo run -q --config 'patch.\"https://github.com/pedalboard/pedalboard-protocol\".pedalboard-protocol.path=\"../pedalboard-protocol\"' --"
TMPFILE=$(mktemp /tmp/audio-test-XXXX.yaml)
cat > "$TMPFILE" << EOF
presets:
  - name: "Audio Test"
    buttons:
      A: { label: "P0", program_change: 0, color: green }
      B: { label: "P1", program_change: 1, color: yellow }
      C: { label: "P2", program_change: 2, color: red }
EOF
eval $CLI --address $BRIDGE/raw upload "$TMPFILE" > /dev/null 2>&1
rm -f "$TMPFILE"
sleep 1
# Verify bridge saw patch 0 load (from on_enter or initial state)
PATCH=$(ssh laenzi@$HOST "journalctl -u pedalboard-bridge --no-pager --since '30 seconds ago'" 2>/dev/null | grep "patch.*active" | tail -1)
if [[ "$PATCH" == *"active"* ]]; then
  echo "✓ ($PATCH)"
else
  echo "✗ (no patch switch in bridge log)"
  exit 1
fi

# Test 7: Services survive reboot
echo -n "7. Services survive reboot... "
ssh laenzi@$HOST "sudo reboot" 2>/dev/null || true
sleep 15
JACK_OK=$(ssh laenzi@$HOST "systemctl is-active pedalboard-jack" 2>/dev/null)
MOD_OK=$(ssh laenzi@$HOST "systemctl is-active pedalboard-modhost" 2>/dev/null)
BRIDGE_OK=$(ssh laenzi@$HOST "systemctl is-active pedalboard-bridge" 2>/dev/null)
if [[ "$JACK_OK" == "active" ]] && [[ "$MOD_OK" == "active" ]] && [[ "$BRIDGE_OK" == "active" ]]; then
  echo "✓ (all services active after reboot)"
else
  echo "✗ (jack=$JACK_OK mod=$MOD_OK bridge=$BRIDGE_OK)"
  exit 1
fi

echo ""
echo "All audio tests passed."
