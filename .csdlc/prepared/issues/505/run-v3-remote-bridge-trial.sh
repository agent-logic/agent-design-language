#!/usr/bin/env bash
set -euo pipefail

mkdir -p .csdlc/evidence/505

cargo run --quiet --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- \
  remote \
  --repo-root . \
  --request .csdlc/prepared/issues/505/v3-remote-bridge-trial-request.json \
  | tee .csdlc/evidence/505/v3-remote-bridge-trial.json
