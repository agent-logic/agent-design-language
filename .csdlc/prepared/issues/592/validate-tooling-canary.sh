#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

for path in \
  .csdlc/issues/592/index.json \
  .csdlc/issues/592/cards/sip.values.json \
  .csdlc/issues/592/cards/stp.values.json \
  .csdlc/issues/592/cards/spp.values.json \
  .csdlc/issues/592/cards/vpp.values.json \
  .csdlc/issues/592/cards/srp.values.json \
  .csdlc/issues/592/cards/sor.values.json \
  .csdlc/prepared/issues/592/design.md \
  .csdlc/prepared/issues/592/diagram.mmd \
  .csdlc/prepared/issues/592/read-528-request.json; do
  test -s "$path"
done
test -s .csdlc/prepared/issues/592/bind-request.json

git merge-base --is-ancestor edbc3ebc9b4e7c0862595345eebff8e04c9d5260 HEAD

echo 'issue 592 retained tooling canary: pass'
