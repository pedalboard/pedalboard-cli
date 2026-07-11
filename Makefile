.PHONY: help build install lint test test-unit test-hw test-midi test-audio test-compiler test-perf docs clean

.DEFAULT_GOAL := help

PROTOCOL_PATCH := --config 'patch."https://github.com/pedalboard/midi-controller".midi-controller.path="../midi-controller"'
HW_TEST := cargo test -p hardware-tests --test

build: ## build
	cargo build --release $(PROTOCOL_PATCH)

install: build ## install to ~/.cargo/bin
	cp target/release/pedalboard-cli ~/.cargo/bin/

docs: ## regenerate schema JSON
	cargo test --test integration schema_matches_committed_file $(PROTOCOL_PATCH)

lint: ## lint source code
	cargo clippy --all-features $(PROTOCOL_PATCH)

# --- Tests ---

test: test-unit ## run all offline tests (unit + integration parsing)
	@echo "All offline tests passed."

test-unit: ## run unit + offline integration tests (no hardware needed)
	cargo test --all $(PROTOCOL_PATCH)

test-hw: test-midi test-audio test-compiler ## run all hardware tests (device must be connected)
	@echo "All hardware tests passed."

test-midi: ## run MIDI/preset hardware tests
	$(HW_TEST) midi -- --test-threads=1

test-audio: ## run audio pipeline hardware tests
	$(HW_TEST) audio -- --test-threads=1

test-compiler: ## run compiler hardware tests (ADR-005)
	$(HW_TEST) compiler -- --test-threads=1

test-perf: ## run performance tests (32-preset upload)
	./tests/perf.sh

# --- Single test ---
# Usage: make test-one T=preset_upload
test-one: ## run a single hardware test by name (T=test_name)
	$(HW_TEST) midi $(T) -- --test-threads=1 --nocapture

clean: ## clean build results
	cargo clean
	cd tests/hardware && cargo clean

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
