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
if [[ $acks -eq 2 ]]; then
  echo "✓ ($acks/2 ACKs)"
else
  echo "✗ ($acks/2 ACKs)"
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
sleep 2
# Reboot via SysEx (graceful — allows in-flight flash writes to complete)
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 5
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" == *"Feature Test"* ]]; then
  echo -n "preset 0 ✓ "
else
  echo "✗ (preset 0 lost after reboot)"
  echo "$result"
  exit 1
fi
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 1 2>&1)
if [[ "$result" == *"Cycle Test"* ]]; then
  echo "preset 1 ✓"
else
  echo "✗ (preset 1 lost after reboot)"
  echo "$result"
  exit 1
fi

# Test 6: Factory reset clears presets
echo -n "6. Factory reset clears presets... "
eval timeout 5 $CLI --address $BRIDGE/config reset 2>&1 > /dev/null || true
sleep 5
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" == *"not found"* ]] || [[ "$result" == *"no reply"* ]]; then
  echo "✓ (preset 0 cleared)"
else
  echo "✗ (preset 0 still present after factory reset)"
  echo "$result"
  exit 1
fi

# Test 7: OpenDeck SysEx upload succeeds
echo -n "7. OpenDeck SysEx upload... "
result=$(eval timeout 15 $CLI --address $BRIDGE/config upload $TEST_CONFIG 2>&1)
if [[ "$result" == *"Upload complete"* ]]; then
  echo "✓"
else
  echo "✗"
  echo "$result"
  exit 1
fi

# Test 8: PE presets survive OpenDeck upload
echo -n "8. PE survives OpenDeck upload... "
# Upload PE presets
eval timeout 15 $CLI --address $BRIDGE/raw pe-upload $TEST_CONFIG 2>&1 > /dev/null
sleep 2  # let persist task flush to flash before hammering with OpenDeck writes
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

# Test 9: OpenDeck + PE coexist across reboot
echo -n "9. Coexistence survives reboot... "
sleep 5  # let persist task flush SaveBulk to flash before rebooting
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 5
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
