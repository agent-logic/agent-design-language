#!/usr/bin/env bash
set -euo pipefail

test -s docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md
test -s .csdlc/prepared/issues/708/design.md
test -s .csdlc/prepared/issues/708/diagram.mmd
test -s .csdlc/issues/708/index.json
git diff --check -- .csdlc/prepared/issues/708 .csdlc/issues/708
