#!/usr/bin/env bash
set -euo pipefail
cat >&2 <<'MSG'
closeout_completed_issue_wave.sh is retired.
Wave closeout must move to a Rust/octocrab-backed C-SDLC lane before it can be
used again. Resolve the selected generation with `csdlc-install resolve`, then
submit one explicit typed `csdlc-closeout` request per issue.
MSG
exit 2
