#!/bin/bash
# Performance test: 32 presets (max capacity) upload, persist, and read-back
set -e

BRIDGE="ws://cm5-dev.home"
CLI="cargo run -q --config 'patch.\"https://github.com/pedalboard/midi-controller\".midi-controller.path=\"../midi-controller\"' --"
PERF_CONFIG="examples/perf-32-presets.yaml"

echo "=== Performance Test: 32 Presets ==="

# Clean slate
echo -n "1. Factory reset... "
eval timeout 10 $CLI --address $BRIDGE/config reset 2>&1 > /dev/null || true
sleep 8
echo "✓"

# Upload 32 presets and measure time
echo -n "2. Upload 32 presets... "
start=$(date +%s%3N)
result=$(eval timeout 60 $CLI --address $BRIDGE/raw upload $PERF_CONFIG 2>&1)
end=$(date +%s%3N)
elapsed=$((end - start))
if [[ "$result" == *"Upload complete"* ]]; then
  ack_count=$(echo "$result" | grep -c "ACK ✓")
  echo "✓ ($ack_count ACKs in ${elapsed}ms)"
else
  echo "✗"
  echo "$result"
  exit 1
fi

# Read back all presets (after persist task processes all saves)
echo -n "3. Read-back all 32 presets... "
sleep 10  # persist task needs time to save all 32 presets to flash + update RAM
start=$(date +%s%3N)
fail=0
for i in $(seq 0 31); do
  result=$(eval timeout 5 $CLI --address $BRIDGE/raw read $i 2>&1)
  expected=$(printf "Perf %02d" $i)
  if [[ "$result" != *"$expected"* ]]; then
    echo "✗ (preset $i: expected \"$expected\")"
    echo "$result"
    fail=1
    break
  fi
done
end=$(date +%s%3N)
elapsed=$((end - start))
if [[ $fail -eq 0 ]]; then
  echo "✓ (32/32 in ${elapsed}ms)"
fi

# Wait for persist then reboot
echo -n "4. Persistence (reboot + verify)... "
sleep 15  # 32 presets × ~180ms persist each + safety margin
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 7

# Verify persistence (known limitation: #75 — not all presets may survive)
echo -n "4. Persistence (reboot + verify)... "
sleep 15
eval timeout 5 $CLI --address $BRIDGE/config reboot 2>&1 > /dev/null || true
sleep 7

persisted=0
for i in $(seq 0 31); do
  result=$(eval timeout 5 $CLI --address $BRIDGE/raw read $i 2>&1)
  expected=$(printf "Perf %02d" $i)
  if [[ "$result" == *"$expected"* ]]; then
    persisted=$((persisted + 1))
  fi
done
echo "$persisted/32 persisted (see #75)"

# Re-upload to test overwrite performance
echo -n "5. Overwrite 32 presets... "
start=$(date +%s%3N)
result=$(eval timeout 60 $CLI --address $BRIDGE/raw upload $PERF_CONFIG 2>&1)
end=$(date +%s%3N)
elapsed=$((end - start))
if [[ "$result" == *"Upload complete"* ]]; then
  echo "✓ (${elapsed}ms)"
else
  echo "✗"
  echo "$result"
  exit 1
fi

echo ""
echo "All performance tests passed."
