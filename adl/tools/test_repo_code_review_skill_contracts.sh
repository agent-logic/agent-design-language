#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${ADL_REVIEW_SKILL_CONTRACT_ROOT:-}"
if [[ "$skills_root" != /* || ! -d "$skills_root" ]]; then
  echo "Select the authentic original review-skill fixture with ADL_REVIEW_SKILL_CONTRACT_ROOT; current owner layout uses its separately qualified contracts." >&2
  exit 2
fi

[[ -f "${skills_root}/repo-code-review/SKILL.md" ]]
[[ -f "${skills_root}/repo-code-review/adl-skill.yaml" ]]
[[ -f "${skills_root}/repo-code-review/agents/openai.yaml" ]]
[[ -f "${skills_root}/repo-code-review/references/review-playbook.md" ]]
[[ -f "${skills_root}/repo-code-review/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/REPO_CODE_REVIEW_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "repo_code_review.v1"' "${skills_root}/repo-code-review/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_CODE_REVIEW_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-code-review/adl-skill.yaml"
grep -Fq "policy.review_depth_must_be_explicit" "${skills_root}/repo-code-review/adl-skill.yaml"
grep -Fq "policy.stop_after_review_must_be_true" "${skills_root}/repo-code-review/adl-skill.yaml"
grep -Fq "mode: review_repository | review_path | review_branch | review_diff" "${skills_root}/docs/REPO_CODE_REVIEW_SKILL_INPUT_SCHEMA.md"
grep -Fq "skill_input_schema: repo_code_review.v1" "${skills_root}/docs/REPO_CODE_REVIEW_SKILL_INPUT_SCHEMA.md"
grep -Fq "review_repository" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "repo_code_review.v1" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"

echo "PASS test_repo_code_review_skill_contracts"
