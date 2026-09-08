#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

export CODEX_HOME="${tmpdir}/codex-home"

bash "${repo_root}/adl/tools/install_adl_pr_cycle_skill.sh" >/dev/null

installed="${CODEX_HOME}/skills/adl_pr_cycle/SKILL.md"
source_path="${repo_root}/docs/tooling/adl_pr_cycle_skill.md"

[[ -f "${installed}" ]]
cmp -s "${source_path}" "${installed}"
bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" "${installed}"
grep -Fq 'Historical compatibility documentation for the retired adl_pr_cycle route' "${installed}"
grep -Fq 'Do not install, resync, invoke, or route current ADL work through' "${installed}"
grep -Fq 'not a current workflow entrypoint' "${installed}"
grep -Fq 'C-SDLC v2 remains the live lifecycle authority' "${installed}"
grep -Fq 'until explicit V3-F/#505 cutover approval' "${installed}"
grep -Fq 'Any current-work request to use `adl_pr_cycle` is blocked by policy.' "${installed}"

malformed_source="${tmpdir}/bad_adl_pr_cycle_skill.md"
cat >"${malformed_source}" <<'EOF'
---
name: adl_pr_cycle
description: broken: yaml
---
EOF

if ADL_PR_CYCLE_SOURCE_PATH="${malformed_source}" bash "${repo_root}/adl/tools/install_adl_pr_cycle_skill.sh" >/dev/null 2>&1; then
  echo "expected malformed adl_pr_cycle source to fail" >&2
  exit 1
fi

echo "PASS test_install_adl_pr_cycle_skill"
