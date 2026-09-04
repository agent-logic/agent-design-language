#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
design="$root/.csdlc/prepared/issues/674/design.md"
target="$root/docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md"

test -s "$design"
test -s "$target"
rg -q '^# Axioma Polis Welcome Package v1$' "$target"
for heading in 'Where you are' 'Your identity' 'The Polis Shepherd' 'Governed communication' 'Actions you must not take' 'Privacy and credentials' 'When to ask, decline, or escalate'; do
  rg -q "^## ${heading}$" "$target"
done
for phrase in \
  'other residents' \
  'grants no authority' \
  'Runtime admission' \
  'communication eligibility' \
  'Layer 8' \
  'provider availability' \
  'unrestricted autonomous messaging' \
  'credential access' \
  'external side effects' \
  'unbounded loops' \
  'private-data disclosure' \
  'invented capabilities' \
  'ask the Shepherd' \
  'clarify uncertainty' \
  'decline' \
  'welcome' \
  'support'; do
  rg -qi "$phrase" "$target"
done
! rg -n '/Users/|/Volumes/|BEGIN .*PRIVATE KEY|api[_-]?key|bearer token' "$target"
! rg -ni 'sentient|conscious|personhood|mythic|magical|all-powerful|can perform any action' "$target"
