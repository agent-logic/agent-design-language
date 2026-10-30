# Public convenience targets. Existing manifests and validation lanes remain authoritative.
.PHONY: help build plan check packages docs
.DEFAULT_GOAL := help

help:
	@echo 'make build     Build the public CLI'
	@echo 'make plan      Print the provider-free example plan'
	@echo 'make check     Run the public CLI validation lane'
	@echo 'make packages  Run the five public library/compiler checks'
	@echo 'make docs      Check current public documentation contracts'

build:
	cargo build --locked --manifest-path adl/Cargo.toml --bin adl

plan:
	cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan

check:
	bash tools/public_adl/run_cli_ci.sh

packages:
	bash tools/public_adl/run_package_checks.sh

docs:
	cargo run --locked --quiet --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts -- docs --self-test
