#!/bin/bash
# Integration test: verifies firmware PE config pipeline end-to-end
set -e

BRIDGE="ws://cm5-dev.home:8080"
CLI="cargo run -q --config 'patch.\"https://github.com/pedalboard/pedalboard-protocol\".pedalboard-protocol.path=\"../pedalboard-protocol\"' --"
TEST_CONFIG="examples/feature-test.yaml"

echo "=== Integration Tests ==="

# Test 1: OpenDeck handshake
# Test 1: Device reachable via PE
echo -n "1. Device reachable... "
result=$(eval timeout 10 $CLI --address $BRIDGE/raw read 0 2>&1)
if [[ "$result" == *"Preset 0:"* ]] || [[ "$result" == *"not found"* ]]; then
  echo "✓"
else
  echo "✗ ($result)"
  exit 1
fi

# Test 2: PE preset upload
echo -n "2. PE preset upload... "
result=$(eval timeout 15 $CLI --address $BRIDGE/raw upload $TEST_CONFIG 2>&1)
preset_acks=$(echo "$result" | grep -A1 "Preset" | grep -c "ACK" || true)
# feature-test.yaml has 3 presets
if [[ $preset_acks -eq 3 ]]; then
  echo "✓ ($preset_acks/3 preset ACKs)"
else
  echo "✗ ($preset_acks/3 preset ACKs)"
  echo "$result"
  exit 1
fi

# Test 3: PE read-back (verify content after upload)
echo -n "3. PE read-back... "
failures=0
for i in 0 1; do
  result=$(eval timeout 5 $CLI --address $BRIDGE/raw read $i 2>&1)
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
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 0 2>&1)
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
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 0 2>&1)
if [[ "$result" == *"Feature Test"* ]]; then
  echo -n "preset 0 ✓ "
else
  echo "✗ (preset 0 lost after reboot)"
  echo "$result"
  exit 1
fi
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 1 2>&1)
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
result=$(eval timeout 15 $CLI --address $BRIDGE/raw upload $GLOBAL_TEST 2>&1)
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
eval timeout 15 $CLI --address $BRIDGE/raw upload $GLOBAL_TEST 2>&1 > /dev/null
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
eval timeout 15 $CLI --address $BRIDGE/raw upload $GLOBAL_TEST 2>&1 > /dev/null
rm -f "$GLOBAL_TEST"
sleep 1
eval timeout 5 $CLI --address $BRIDGE/config reset 2>&1 > /dev/null || true
sleep 7
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 0 2>&1)
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

# Test 11: PE presets survive reboot after factory reset + re-upload
echo -n "11. PE re-upload after reset... "
eval timeout 15 $CLI --address $BRIDGE/raw upload $TEST_CONFIG 2>&1 > /dev/null
sleep 1
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 7
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 0 2>&1)
if [[ "$result" == *"Feature Test"* ]]; then
  echo "✓ (preset persisted after reset + re-upload + reboot)"
else
  echo "✗ (preset lost)"
  echo "$result"
  exit 1
fi

# Test 12: Stale preset clearing (upload 3, then upload 1, verify slot 1 is gone)
echo -n "12. Stale preset clearing on shorter upload... "
# First upload the 3-preset test config
eval timeout 15 $CLI --address $BRIDGE/raw upload $TEST_CONFIG 2>&1 > /dev/null
sleep 1
# Verify preset 1 exists
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 1 2>&1)
if [[ "$result" != *"LED Animations"* ]]; then
  echo "✗ (preset 1 not present after 3-preset upload)"
  echo "$result"
  exit 1
fi
# Now upload a 1-preset config — should clear slots 1+
SINGLE_TEST=/tmp/pedalboard-single-test.yaml
cat > "$SINGLE_TEST" << 'EOF'
presets:
  - name: "Solo Preset"
    buttons:
      A: { label: "X", cc: 1, color: red }
EOF
eval timeout 15 $CLI --address $BRIDGE/raw upload $SINGLE_TEST 2>&1 > /dev/null
rm -f "$SINGLE_TEST"
sleep 1
# Verify preset 0 is the new one
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 0 2>&1)
if [[ "$result" != *"Solo Preset"* ]]; then
  echo "✗ (preset 0 not updated)"
  echo "$result"
  exit 1
fi
# Verify preset 1 is gone (cleared)
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 1 2>&1)
if [[ "$result" == *"not found"* ]]; then
  echo "✓ (slot 1 cleared after single-preset upload)"
else
  echo "✗ (preset 1 still present)"
  echo "$result"
  exit 1
fi

# Test 13: Stale presets stay cleared across reboot
echo -n "13. Cleared presets stay cleared across reboot... "
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 7
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 0 2>&1)
if [[ "$result" != *"Solo Preset"* ]]; then
  echo "✗ (preset 0 lost after reboot)"
  echo "$result"
  exit 1
fi
result=$(eval timeout 5 $CLI --address $BRIDGE/raw read 1 2>&1)
if [[ "$result" == *"not found"* ]]; then
  echo "✓ (slot 1 still cleared after reboot)"
else
  echo "✗ (preset 1 reappeared after reboot)"
  echo "$result"
  exit 1
fi

# Test 14: Device status reports correct preset count
echo -n "14. Device status reports correct state... "
result=$(eval timeout 5 $CLI --address $BRIDGE/raw status 2>&1)
if [[ "$result" == *"Presets loaded:"* ]] && [[ "$result" == *"Flash format version:"* ]]; then
  loaded=$(echo "$result" | grep "Presets loaded" | awk '{print $NF}')
  if [[ "$loaded" == "1" ]]; then
    echo "✓ (1 preset loaded, status OK)"
  else
    echo "✗ (expected 1 preset loaded, got $loaded)"
    echo "$result"
    exit 1
  fi
else
  echo "✗ (unexpected status output)"
  echo "$result"
  exit 1
fi

echo ""
echo "All tests passed."
