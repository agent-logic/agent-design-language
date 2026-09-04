#!/usr/bin/env bash
set -euo pipefail

test -f csdlc-v2/src/store.rs
test -f csdlc-v2/src/bin/csdlc-bind.rs
test -f csdlc-v2/tests/gate5.rs
test -f .csdlc/prepared/issues/665/design.md
test -f .csdlc/prepared/issues/665/diagram.mmd

rg -n "adopt|emergency|topology|bind" .csdlc/prepared/issues/665/design.md >/dev/null
rg -n "adopt|emergency|ready.*bound|ready-to-bound" .csdlc/prepared/issues/665/design.md >/dev/null
