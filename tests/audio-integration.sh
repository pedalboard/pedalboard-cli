#!/bin/bash
# Audio integration tests: verifies the full audio processing pipeline.
# Requires: JACK + mod-host + bridge running on CM5 with audio-rig.yaml.
#
# Usage: ./tests/audio-integration.sh [bridge_host]
set -e

HOST="${1:-cm5-dev.home}"
BRIDGE="http://$HOST:8080"

echo "=== Audio Integration Tests (host: $HOST) ==="

# Test 1: JACK is running with correct config
echo -n "1. JACK running (48kHz, 64 frames)... "
SR=$(ssh laenzi@$HOST "jack_samplerate 2>/dev/null")
BUF=$(ssh laenzi@$HOST "jack_bufsize 2>/dev/null")
if [[ "$SR" == "48000" ]] && [[ "$BUF" == "64" ]]; then
  echo "✓ (${SR}Hz, ${BUF} frames)"
else
  echo "✗ (sr=$SR buf=$BUF)"
  exit 1
fi

# Test 2: mod-host connected (bridge holds the connection)
echo -n "2. mod-host connected (via bridge)... "
# Bridge logs "mod-host connected" on startup
MOD_LOG=$(ssh laenzi@$HOST "journalctl -u pedalboard-bridge --no-pager -n 200 | grep -m1 'mod-host connected'" 2>/dev/null)
if [[ -n "$MOD_LOG" ]]; then
  echo "✓"
else
  echo "✗ (no mod-host connection in bridge log)"
  exit 1
fi

# Test 3: Bridge is running and reachable
echo -n "3. Bridge reachable... "
VERSION=$(curl -s "$BRIDGE/")
if [[ "$VERSION" == *"pedalboard-bridge"* ]]; then
  echo "✓ ($VERSION)"
else
  echo "✗"
  exit 1
fi

# Test 4: Rig loaded (all plugins present in JACK)
echo -n "4. Rig loaded (6 plugins)... "
PLUGIN_COUNT=$(ssh laenzi@$HOST "jack_lsp 2>/dev/null | grep 'effect_[0-9]:' | sort -u | wc -l")
if [[ "$PLUGIN_COUNT" -ge 6 ]]; then
  echo "✓ ($PLUGIN_COUNT plugin ports)"
else
  echo "✗ (only $PLUGIN_COUNT plugin ports found)"
  exit 1
fi

# Test 5: Audio connections wired correctly
echo -n "5. Audio routing (capture → distortion → amps → reverb → delay → playback)... "
CAPTURE_CONN=$(ssh laenzi@$HOST "jack_lsp -c 2>/dev/null | grep -A1 'effect_0:in$' | grep capture_2")
PLAYBACK_CONN=$(ssh laenzi@$HOST "jack_lsp -c 2>/dev/null | grep -A1 'system:playback_2' | grep 'effect_5\|effect_4'")
if [[ -n "$CAPTURE_CONN" ]] && [[ -n "$PLAYBACK_CONN" ]]; then
  echo "✓"
else
  echo "✗ (capture→effect_0: $CAPTURE_CONN, output: $PLAYBACK_CONN)"
  exit 1
fi

# Test 6: AIDA-X models loaded (check journal for model load messages)
echo -n "6. AIDA-X models loaded... "
MODEL_COUNT=$(ssh laenzi@$HOST "journalctl -u pedalboard-bridge --no-pager -n 50 | grep -c 'loaded model'" 2>/dev/null)
if [[ "$MODEL_COUNT" -ge 3 ]]; then
  echo "✓ ($MODEL_COUNT models)"
else
  echo "✗ (only $MODEL_COUNT models loaded)"
  exit 1
fi

# Test 7: Snapshot active
echo -n "7. Snapshot active... "
SNAPSHOT=$(ssh laenzi@$HOST "journalctl -u pedalboard-bridge --no-pager -n 50 | grep 'snapshot.*active' | tail -1" 2>/dev/null)
if [[ "$SNAPSHOT" == *"active"* ]]; then
  echo "✓ ($(echo "$SNAPSHOT" | grep -oP "snapshot '\K[^']+"))"
else
  echo "✗ (no active snapshot)"
  exit 1
fi

# Test 8: Plugins are loaded and accepting commands (verified via rig load log)
echo -n "8. Plugins loaded and functional... "
RIG_LOG=$(ssh laenzi@$HOST "journalctl -u pedalboard-bridge --no-pager -n 50 | grep 'rig loaded'" 2>/dev/null)
if [[ "$RIG_LOG" == *"6 plugins"* ]]; then
  echo "✓ (rig loaded with 6 plugins)"
else
  echo "✗ ($RIG_LOG)"
  exit 1
fi

# Test 9: Mode switching (live → design → live)
echo -n "9. Mode switching... "
MODE=$(curl -s "$BRIDGE/mode")
if [[ "$MODE" == "live" ]]; then
  # Switch to design
  curl -s -X POST "$BRIDGE/mode?set=design" > /dev/null
  sleep 2
  DESIGN=$(curl -s "$BRIDGE/mode")
  # Switch back
  curl -s -X POST "$BRIDGE/mode?set=live" > /dev/null
  sleep 3
  LIVE=$(curl -s "$BRIDGE/mode")
  if [[ "$DESIGN" == *"design"* ]] && [[ "$LIVE" == *"live"* ]]; then
    echo "✓ (live → design → live)"
  else
    echo "✗ (design=$DESIGN live=$LIVE)"
    exit 1
  fi
else
  echo "✗ (initial mode: $MODE, expected 'live')"
  exit 1
fi

echo ""
echo "All audio tests passed."
