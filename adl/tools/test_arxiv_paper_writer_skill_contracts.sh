#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
# Select exact qualified Runtime resources; this is retained ADL consumer proof.
owner_root="${ADL_ARXIV_SKILL_SOURCE_ROOT:-}"
python3 - "$owner_root" <<'PY_OWNER'
import hashlib,pathlib,sys
root=pathlib.Path(sys.argv[1])
if not root.is_absolute(): raise SystemExit("Select absolute ADL_ARXIV_SKILL_SOURCE_ROOT for qualified Runtime resources")
expected={'adl/tools/skills/arxiv-paper-writer/SKILL.md': '686bddca88d2542b7fc79185cee917d8dcc4ccc8306acb0549913671348ca711', 'adl/tools/skills/arxiv-paper-writer/adl-skill.yaml': 'cbb5e90dd32648bb5cfd9d8960c44bb94be6d957c0515119fcc071eeefcf01d3', 'adl/tools/skills/arxiv-paper-writer/agents/openai.yaml': '3b133c476a743b5fc98ea79785baabe5ee8bb66a8f93a2d61011c601817a235a', 'adl/tools/skills/arxiv-paper-writer/references/arxiv-writing-playbook.md': '07c24e6ff3410208d10747a52e217d7ebf1aa04115d6ed6f92da46779a381279', 'adl/tools/skills/arxiv-paper-writer/references/output-contract.md': 'b24eb097fe8d8be8ba1fdb69e55d4f4f3b02fefb8cfc03118851549f957f3f05', 'adl/tools/skills/docs/ARXIV_PAPER_WRITER_SKILL_INPUT_SCHEMA.md': 'ea96de3808a5e55e70f1d4b3011f36abb6eab934ee8a408aab6059a8fa837c21'}
for name,digest in expected.items():
 p=root/name
 if not p.is_file() or p.stat().st_mode & 0o777 != 0o644 or hashlib.sha256(p.read_bytes()).hexdigest()!=digest:
  raise SystemExit("Qualified arXiv resource missing or changed: "+name)
PY_OWNER
skills_root="${owner_root}/adl/tools/skills"

[[ -f "${skills_root}/arxiv-paper-writer/SKILL.md" ]]
[[ -f "${skills_root}/arxiv-paper-writer/adl-skill.yaml" ]]
[[ -f "${skills_root}/arxiv-paper-writer/agents/openai.yaml" ]]
[[ -f "${skills_root}/arxiv-paper-writer/references/arxiv-writing-playbook.md" ]]
[[ -f "${skills_root}/arxiv-paper-writer/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/ARXIV_PAPER_WRITER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "arxiv_paper_writer.v1"' "${skills_root}/arxiv-paper-writer/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ARXIV_PAPER_WRITER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/arxiv-paper-writer/adl-skill.yaml"
grep -Fq "policy.citation_policy_must_be_explicit" "${skills_root}/arxiv-paper-writer/adl-skill.yaml"
grep -Fq "policy.stop_before_submission_must_be_true" "${skills_root}/arxiv-paper-writer/adl-skill.yaml"
grep -Fq "without submitting, publishing, inventing citations" "${skills_root}/arxiv-paper-writer/SKILL.md"
grep -Fq "no arXiv submission was attempted" "${skills_root}/arxiv-paper-writer/SKILL.md"
grep -Fq "This skill stops at a reviewable manuscript packet." "${skills_root}/arxiv-paper-writer/references/arxiv-writing-playbook.md"
grep -Fq "Submission Attempted: true | false" "${skills_root}/arxiv-paper-writer/references/output-contract.md"
grep -Fq 'Schema id: `arxiv_paper_writer.v1`' "${skills_root}/docs/ARXIV_PAPER_WRITER_SKILL_INPUT_SCHEMA.md"

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/arxiv-paper-writer/SKILL.md"

echo "PASS test_arxiv_paper_writer_skill_contracts"
