#!/usr/bin/env bash
set -euo pipefail

mkdir -p .csdlc/evidence/505

cargo run --quiet --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- \
  local \
  --request .csdlc/prepared/issues/505/v3-local-trial-request.json \
  --registry docs/templates/prompts/current.json \
  --registrations .csdlc/prepared/issues/505/v3-local-trial-registrations.json \
  | tee .csdlc/evidence/505/v3-local-trial.json
