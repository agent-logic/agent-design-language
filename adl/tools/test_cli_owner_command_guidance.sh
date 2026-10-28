#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

required_surfaces=(
  "AGENTS.md"
  "CONTRIBUTING.md"
  "docs/default_workflow.md"
  "adl/tools/generate_active_command_reference_scan.py"
  "adl/tools/generate_tool_surface_registry.sh"
  "adl/src/cli/public_dispatch.rs"
  "adl/tools/editor_action.sh"
  "adl/tools/README.md"
  "docs/tooling/editor/command_adapter.md"
  "docs/tooling/editor/current_skill_wiring_demo.md"
  "docs/tooling/editor/five_command_demo.md"
  "docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md"
  "docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md"
  "docs/tooling/PREP_SCOUT_NEXT_ISSUE_READINESS_LANE.md"
  "docs/tooling/README.md"
  "docs/tooling/issue-prompts/issue-prompt-template.md"
  "docs/tooling/worktree_governance.md"
)

forbidden_guidance_surfaces=(
  "AGENTS.md"
  "CONTRIBUTING.md"
  "docs/default_workflow.md"
  "adl/src/cli/public_dispatch.rs"
  "adl/tools/editor_action.sh"
  "adl/tools/README.md"
  "docs/tooling/editor/command_adapter.md"
  "docs/tooling/editor/current_skill_wiring_demo.md"
  "docs/tooling/editor/five_command_demo.md"
  "docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md"
  "docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md"
  "docs/tooling/PREP_SCOUT_NEXT_ISSUE_READINESS_LANE.md"
  "docs/tooling/README.md"
  "docs/tooling/issue-prompts/issue-prompt-template.md"
  "docs/tooling/worktree_governance.md"
)

forbidden_patterns=(
  "adl/tools/pr.sh run <issue>"
  "./adl/tools/pr.sh run"
  "bash adl/tools/pr.sh run"
  "adl/tools/pr.sh run"
  "adl-csdlc issue run"
  "adl pr run"
  "adl/tools/pr.sh start"
  "editor_action.sh start"
  "adl pr create"
  "adl pr init"
  "adl pr doctor"
  "adl pr finish"
  "adl pr closeout"
  "adl tooling "
  "adl-csdlc tooling "
)

# Discover every tracked CLI implementation file instead of relying on a
# hand-maintained subset. This keeps new help/diagnostic modules inside the
# final-authority guard automatically.
while IFS= read -r surface; do
  forbidden_guidance_surfaces+=("$surface")
  required_surfaces+=("$surface")
done < <(git ls-files 'adl/src/cli/*.rs' 'adl/src/cli/**/*.rs')

# Missing retained guidance must fail rather than be ignored by rg inside an if.
for surface in "${required_surfaces[@]}"; do
  [[ -f "$surface" ]] || {
    echo "FAIL: retained command guidance surface is missing: $surface" >&2
    exit 1
  }
done

for pattern in "${forbidden_patterns[@]}"; do
  if rg -n --fixed-strings "$pattern" "${forbidden_guidance_surfaces[@]}"; then
    echo "FAIL: live command guidance still teaches deprecated command: $pattern" >&2
    exit 1
  fi
done

required_patterns=(
  ".adl/bin/native-v3/csdlc"
  "csdlc bind"
  "csdlc review"
)

for pattern in "${required_patterns[@]}"; do
  if ! rg -n --fixed-strings "$pattern" "${required_surfaces[@]}" >/dev/null; then
    echo "FAIL: live command guidance is missing required command: $pattern" >&2
    exit 1
  fi
done

echo "PASS: live CLI owner command guidance is aligned with current native C-SDLC v3 authority"
