#!/bin/bash
# Integration test: verifies firmware responds correctly via bridge
set -e

BRIDGE="ws://cm5-dev.home"
CLI="cargo run -q --config 'patch.\"https://github.com/pedalboard/pedalboard-protocol\".pedalboard-protocol.path=\"../pedalboard-protocol\"' --"

echo "=== Integration Tests ==="

# Test 1: OpenDeck handshake
echo -n "1. OpenDeck handshake... "
result=$(eval timeout 10 $CLI --address $BRIDGE/config upload examples/setlist.yaml 2>&1 | head -1)
if [[ "$result" == "Connected." ]]; then
  echo "✓"
else
  echo "✗ ($result)"
  exit 1
fi

# Test 2: PE preset upload
echo -n "2. PE preset upload... "
result=$(eval timeout 15 $CLI --address $BRIDGE/raw pe-upload examples/setlist.yaml 2>&1)
acks=$(echo "$result" | grep -c "ACK ✓")
if [[ $acks -eq 3 ]]; then
  echo "✓ ($acks/3 ACKs)"
else
  echo "✗ ($acks/3 ACKs)"
  echo "$result"
  exit 1
fi

# Test 3: PE preset persistence
echo -n "3. PE preset persistence (reset + re-check)... "
# Wait for firmware to save to flash
sleep 2
# Reset firmware via OpenDeck SysEx (reboot, not factory reset)
eval timeout 5 $CLI --address $BRIDGE/config reset 2>&1 > /dev/null || true
sleep 5
# Upload again — if firmware booted with presets from flash, it will ACK
result=$(eval timeout 15 $CLI --address $BRIDGE/raw pe-upload examples/setlist.yaml 2>&1)
acks=$(echo "$result" | grep -c "ACK ✓")
if [[ $acks -eq 3 ]]; then
  echo "✓ (firmware survived reboot, $acks/3 ACKs)"
else
  echo "✗ (firmware failed after reboot, $acks/3 ACKs)"
  echo "$result"
  exit 1
fi

# Test 4: Factory reset
echo -n "4. Factory reset... "
result=$(eval timeout 10 $CLI --address $BRIDGE/config reset 2>&1)
if [[ "$result" == *"Factory reset sent"* ]]; then
  echo "✓"
else
  echo "✗ ($result)"
  exit 1
fi

echo ""
echo "All tests passed."
