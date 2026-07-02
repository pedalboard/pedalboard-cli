.PHONY: help build install lint test integration clean

.DEFAULT_GOAL := help

PROTOCOL_PATCH := --config 'patch."https://github.com/pedalboard/pedalboard-protocol".pedalboard-protocol.path="../pedalboard-protocol"'

build: ## build
	cargo build --release $(PROTOCOL_PATCH)

install: build ## install to ~/.cargo/bin
	cp target/release/pedalboard-cli ~/.cargo/bin/

docs: ## regenerate config-reference.md from schema
	mise exec -- generate-schema-doc --config template_name=md schema/pedalboard.schema.json docs/config-reference.md

lint: ## lint source code
	cargo clippy --all-features $(PROTOCOL_PATCH)

test: docs ## run unit tests
	cargo test --all $(PROTOCOL_PATCH)

integration: ## run integration tests (requires device connected)
	./tests/integration.sh

clean: ## clean build results
	cargo clean

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
