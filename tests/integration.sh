#!/bin/bash
# Integration test: verifies firmware PE config pipeline end-to-end
set -e

BRIDGE="ws://cm5-dev.home"
CLI="cargo run -q --config 'patch.\"https://github.com/pedalboard/pedalboard-protocol\".pedalboard-protocol.path=\"../pedalboard-protocol\"' --"
TEST_CONFIG="examples/feature-test.yaml"

echo "=== Integration Tests ==="

# Test 1: OpenDeck handshake
echo -n "1. OpenDeck handshake... "
result=$(eval timeout 10 $CLI --address $BRIDGE/config upload $TEST_CONFIG 2>&1 | head -1)
if [[ "$result" == "Connected." ]]; then
  echo "✓"
else
  echo "✗ ($result)"
  exit 1
fi

# Test 2: PE preset upload
echo -n "2. PE preset upload... "
result=$(eval timeout 15 $CLI --address $BRIDGE/raw pe-upload $TEST_CONFIG 2>&1)
acks=$(echo "$result" | grep -c "ACK ✓")
# feature-test.yaml has global config + 3 presets = 4 ACKs
expected_acks=4
if [[ $acks -eq $expected_acks ]]; then
  echo "✓ ($acks/$expected_acks ACKs)"
else
  echo "✗ ($acks/$expected_acks ACKs)"
  echo "$result"
  exit 1
fi

# Test 3: PE read-back (verify content after upload)
echo -n "3. PE read-back... "
failures=0
for i in 0 1; do
  result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read $i 2>&1)
  if [[ "$result" != *"Preset $i:"* ]] || [[ "$result" == *"no reply"* ]] || [[ "$result" == *"not found"* ]]; then
    failures=$((failures + 1))
  fi
done
if [[ $failures -eq 0 ]]; then
  echo "✓ (2/2 presets readable)"
else
  echo "✗ ($((2 - failures))/2 presets readable)"
  exit 1
fi

# Test 4: PE read-back content verification
echo -n "4. Content verification... "
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" == *"Feature Test"* ]] && [[ "$result" == *"Toggle"* ]] && [[ "$result" == *"Reverb"* ]]; then
  echo "✓ (preset 0: name + buttons + encoders match)"
else
  echo "✗ (unexpected content)"
  echo "$result"
  exit 1
fi

# Test 5: Persistence (reboot + read-back without re-upload)
echo -n "5. Persistence (reboot + read-back)... "
# Reboot via SysEx (graceful — allows in-flight flash writes to complete)
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 7
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" == *"Feature Test"* ]]; then
  echo -n "preset 0 ✓ "
else
  echo "✗ (preset 0 lost after reboot)"
  echo "$result"
  exit 1
fi
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 1 2>&1)
if [[ "$result" == *"LED Animations"* ]]; then
  echo "preset 1 ✓"
else
  echo "✗ (preset 1 lost after reboot)"
  echo "$result"
  exit 1
fi

# Test 6: Global config upload
echo -n "6. Global config upload... "
GLOBAL_TEST=/tmp/pedalboard-gc-test.yaml
cat > "$GLOBAL_TEST" << 'EOF'
global:
  din_enabled: true
  din_to_usb_thru: true
  usb_to_din_thru: false
  midi_clock: true
  bpm: 100
presets:
  - name: "GC Test"
    buttons:
      A: { label: "X", cc: 1, color: red }
EOF
result=$(eval timeout 15 $CLI --address $BRIDGE/raw pe-upload $GLOBAL_TEST 2>&1)
gc_ack=$(echo "$result" | grep -A1 "Global config" | grep -c "ACK" || true)
preset_ack=$(echo "$result" | grep -A1 "Preset 0" | grep -c "ACK" || true)
if [[ $gc_ack -eq 1 ]] && [[ $preset_ack -eq 1 ]]; then
  echo "✓ (global + preset ACK)"
else
  echo "✗ (gc_ack=$gc_ack, preset_ack=$preset_ack)"
  echo "$result"
  rm -f "$GLOBAL_TEST"
  exit 1
fi

# Test 7: Global config enables MIDI clock
echo -n "7. MIDI clock active after global config... "
sleep 1
clock_output=$(eval timeout 2 $CLI --address $BRIDGE/monitor monitor 2>&1 || true)
clock_count=$(echo "$clock_output" | grep -c "Clock" || true)
if [[ $clock_count -gt 5 ]]; then
  echo "✓ ($clock_count ticks in 2s)"
else
  echo "✗ (only $clock_count clock ticks)"
  exit 1
fi

# Test 8: Global config persists across reboot
echo -n "8. Global config persists (reboot + clock check)... "
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 7
clock_output=$(eval timeout 2 $CLI --address $BRIDGE/monitor monitor 2>&1 || true)
clock_count=$(echo "$clock_output" | grep -c "Clock" || true)
if [[ $clock_count -gt 5 ]]; then
  echo "✓ (clock still running after reboot)"
else
  echo "✗ (clock stopped after reboot: $clock_count ticks)"
  exit 1
fi

# Test 9: Global config disable clock
echo -n "9. Disable MIDI clock via global config... "
cat > "$GLOBAL_TEST" << 'EOF'
global:
  midi_clock: false
presets:
  - name: "GC Test"
    buttons:
      A: { label: "X", cc: 1, color: red }
EOF
eval timeout 15 $CLI --address $BRIDGE/raw pe-upload $GLOBAL_TEST 2>&1 > /dev/null
sleep 1
clock_output=$(eval timeout 2 $CLI --address $BRIDGE/monitor monitor 2>&1 || true)
clock_count=$(echo "$clock_output" | grep -c "Clock" || true)
rm -f "$GLOBAL_TEST"
if [[ $clock_count -eq 0 ]]; then
  echo "✓ (clock stopped)"
else
  echo "✗ (still $clock_count clock ticks)"
  exit 1
fi

# Test 10: Factory reset clears presets and global config
echo -n "10. Factory reset clears presets and global config... "
# Re-enable clock so we can verify reset clears it
cat > "$GLOBAL_TEST" << 'EOF'
global:
  midi_clock: true
  bpm: 120
presets:
  - name: "GC Test"
    buttons:
      A: { label: "X", cc: 1, color: red }
EOF
eval timeout 15 $CLI --address $BRIDGE/raw pe-upload $GLOBAL_TEST 2>&1 > /dev/null
rm -f "$GLOBAL_TEST"
sleep 1
eval timeout 5 $CLI --address $BRIDGE/config reset 2>&1 > /dev/null || true
sleep 7
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" == *"not found"* ]] || [[ "$result" == *"no reply"* ]]; then
  # Also verify clock stopped (global config cleared)
  clock_output=$(eval timeout 2 $CLI --address $BRIDGE/monitor monitor 2>&1 || true)
  clock_count=$(echo "$clock_output" | grep -c "Clock" || true)
  if [[ $clock_count -eq 0 ]]; then
    echo "✓ (preset cleared + clock stopped)"
  else
    echo "✗ (preset cleared but clock still running)"
    exit 1
  fi
else
  echo "✗ (preset 0 still present after factory reset)"
  echo "$result"
  exit 1
fi

# Test 11: OpenDeck SysEx upload succeeds
echo -n "11. OpenDeck SysEx upload... "
result=$(eval timeout 15 $CLI --address $BRIDGE/config upload $TEST_CONFIG 2>&1)
if [[ "$result" == *"Upload complete"* ]]; then
  echo "✓"
else
  echo "✗"
  echo "$result"
  exit 1
fi

# Test 12: PE presets survive OpenDeck upload
echo -n "12. PE survives OpenDeck upload... "
# Upload PE presets
eval timeout 15 $CLI --address $BRIDGE/raw pe-upload $TEST_CONFIG 2>&1 > /dev/null
sleep 1  # let persist task flush PE presets before OpenDeck writes
# Upload OpenDeck on top
eval timeout 15 $CLI --address $BRIDGE/config upload $TEST_CONFIG 2>&1 > /dev/null
sleep 1  # let bridge recover from OpenDeck session before PE read
# Verify PE presets are still readable
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" == *"Feature Test"* ]]; then
  echo "✓ (PE preset 0 intact after OpenDeck upload)"
else
  echo "✗ (PE preset 0 lost)"
  echo "$result"
  exit 1
fi

# Test 13: OpenDeck + PE coexist across reboot
echo -n "13. Coexistence survives reboot... "
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 7
# Verify PE still readable
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" != *"Feature Test"* ]]; then
  echo "✗ (PE preset 0 lost after reboot)"
  echo "$result"
  exit 1
fi
# Verify OpenDeck handshake still works
result=$(eval timeout 10 $CLI --address $BRIDGE/config upload $TEST_CONFIG 2>&1 | head -1)
if [[ "$result" == "Connected." ]]; then
  echo "✓ (PE + OpenDeck both functional after reboot)"
else
  echo "✗ (OpenDeck handshake failed after reboot)"
  echo "$result"
  exit 1
fi

echo ""
echo "All tests passed."
