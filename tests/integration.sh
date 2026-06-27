#!/bin/bash
# Integration test: verifies firmware PE config pipeline end-to-end
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

# Test 3: PE read-back (verify content after upload)
echo -n "3. PE read-back... "
failures=0
for i in 0 1 2; do
  result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read $i 2>&1)
  if [[ "$result" != *"Preset $i:"* ]] || [[ "$result" == *"no reply"* ]] || [[ "$result" == *"not found"* ]]; then
    failures=$((failures + 1))
  fi
done
if [[ $failures -eq 0 ]]; then
  echo "✓ (3/3 presets readable)"
else
  echo "✗ ($((3 - failures))/3 presets readable)"
  exit 1
fi

# Test 4: PE read-back content verification
echo -n "4. Content verification... "
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" == *"Autumn Leaves"* ]] && [[ "$result" == *"Verse"* ]] && [[ "$result" == *"Reverb"* ]]; then
  echo "✓ (preset 0: name + buttons + encoders match)"
else
  echo "✗ (unexpected content)"
  echo "$result"
  exit 1
fi

# Test 5: Persistence (reboot + read-back without re-upload)
echo -n "5. Persistence (reboot + read-back)... "
sleep 2
# Reboot via probe reset (not factory reset which erases flash)
probe-rs reset --chip RP2040 --protocol swd 2>/dev/null || true
sleep 5
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 0 2>&1)
if [[ "$result" == *"Autumn Leaves"* ]]; then
  echo -n "preset 0 ✓ "
else
  echo "✗ (preset 0 lost after reboot)"
  echo "$result"
  exit 1
fi
result=$(eval timeout 5 $CLI --address $BRIDGE/raw pe-read 2 2>&1)
if [[ "$result" == *"So What"* ]]; then
  echo "preset 2 ✓"
else
  echo "✗ (preset 2 lost after reboot)"
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

echo ""
echo "All tests passed."
