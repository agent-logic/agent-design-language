#!/usr/bin/env bash
set -euo pipefail

: "${CARGO_TARGET_DIR:?set CARGO_TARGET_DIR to an operator-approved external build target}"
cargo build --quiet --manifest-path csdlc-v2/Cargo.toml --bins
"$CARGO_TARGET_DIR/debug/csdlc-install" install --repo . --destination .adl/bin/csdlc-v2
"$CARGO_TARGET_DIR/debug/csdlc-install" verify \
  --repo . \
  --bin-dir .adl/bin/csdlc-v2 \
  --inventory csdlc-v2/operator/coexistence.json
.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5597
