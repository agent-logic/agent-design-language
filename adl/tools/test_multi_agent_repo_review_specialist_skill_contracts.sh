#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${ADL_REVIEW_SKILL_CONTRACT_ROOT:-}"
if [[ "$skills_root" != /* || ! -d "$skills_root" ]]; then
  echo "Select the authentic original review-skill fixture with ADL_REVIEW_SKILL_CONTRACT_ROOT; current owner layout uses its separately qualified contracts." >&2
  exit 2
fi
docs_root="${skills_root}/docs"

for skill in repo-review-code repo-review-docs repo-review-security repo-review-synthesis repo-review-tests; do
  [[ -f "${skills_root}/${skill}/SKILL.md" ]]
  [[ -f "${skills_root}/${skill}/adl-skill.yaml" ]]
  [[ -f "${skills_root}/${skill}/agents/openai.yaml" ]]
  [[ -f "${skills_root}/${skill}/references/output-contract.md" ]]
done

[[ -f "${docs_root}/REPO_REVIEW_CODE_SKILL_INPUT_SCHEMA.md" ]]
[[ -f "${docs_root}/REPO_REVIEW_DOCS_SKILL_INPUT_SCHEMA.md" ]]
[[ -f "${docs_root}/REPO_REVIEW_SECURITY_SKILL_INPUT_SCHEMA.md" ]]
[[ -f "${docs_root}/REPO_REVIEW_SYNTHESIS_SKILL_INPUT_SCHEMA.md" ]]
[[ -f "${docs_root}/REPO_REVIEW_TESTS_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'reference_doc: "../docs/REPO_REVIEW_CODE_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-review-code/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_REVIEW_DOCS_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-review-docs/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_REVIEW_SECURITY_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-review-security/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_REVIEW_SYNTHESIS_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-review-synthesis/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_REVIEW_TESTS_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-review-tests/adl-skill.yaml"

for skill in repo-review-code repo-review-docs repo-review-security repo-review-synthesis repo-review-tests; do
  grep -Fq 'structured_contract: "references/output-contract.md"' "${skills_root}/${skill}/adl-skill.yaml"
  grep -Fq "${skill}" "${docs_root}/OPERATIONAL_SKILLS_GUIDE.md"
  grep -Fq "${skill}" "${docs_root}/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"
done

grep -Fq "repo_review_code.v1" "${docs_root}/REPO_REVIEW_CODE_SKILL_INPUT_SCHEMA.md"
grep -Fq "repo_review_docs.v1" "${docs_root}/REPO_REVIEW_DOCS_SKILL_INPUT_SCHEMA.md"
grep -Fq "repo_review_security.v1" "${docs_root}/REPO_REVIEW_SECURITY_SKILL_INPUT_SCHEMA.md"
grep -Fq "repo_review_synthesis.v1" "${docs_root}/REPO_REVIEW_SYNTHESIS_SKILL_INPUT_SCHEMA.md"
grep -Fq "repo_review_tests.v1" "${docs_root}/REPO_REVIEW_TESTS_SKILL_INPUT_SCHEMA.md"

echo "PASS test_multi_agent_repo_review_specialist_skill_contracts"
