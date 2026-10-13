#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
# Select the exact qualified Runtime source-resource layout; no installed proof implied.
owner_root="${ADL_MEDIUM_SKILL_SOURCE_ROOT:-}"
python3 - "$owner_root" <<'PY_OWNER'
import hashlib,pathlib,sys
root=pathlib.Path(sys.argv[1])
if not root.is_absolute(): raise SystemExit("Select absolute ADL_MEDIUM_SKILL_SOURCE_ROOT for the qualified Runtime resources")
expected={'adl/tools/skills/docs/MEDIUM_ARTICLE_WRITER_SKILL_INPUT_SCHEMA.md': '22e7983f8ccb20c980a9c34fe7825d409083fa11f2e508ae80d10739c72d4b35', 'adl/tools/skills/medium-article-writer/SKILL.md': '1d882a06c45b77af6587569a9928c22b018b5b79c437c0e3e7847421b2319c6f', 'adl/tools/skills/medium-article-writer/adl-skill.yaml': 'a886670d52210e0f71169b6d8790d62195b305905c7c283f2cada03d3ad97dda', 'adl/tools/skills/medium-article-writer/agents/openai.yaml': 'bd0e45acb2a4fdd7ae469fd087475b60d8fa123570822950e203318d2e21068e', 'adl/tools/skills/medium-article-writer/references/medium-writing-playbook.md': 'edd8209ed4398f157313875c32a19c616012b43c102f5138c297ea70d5469278', 'adl/tools/skills/medium-article-writer/references/output-contract.md': '55755f7a612ffd0296094def6c994fea9d018fb0b27bd4d826b6d973f40fd0fe'}
for name,digest in expected.items():
 p=root/name
 if not p.is_file() or p.stat().st_mode & 0o777 != 0o644 or hashlib.sha256(p.read_bytes()).hexdigest()!=digest:
  raise SystemExit("Qualified Medium resource missing or changed: "+name)
PY_OWNER
skills_root="${owner_root}/adl/tools/skills"

[[ -f "${skills_root}/medium-article-writer/SKILL.md" ]]
[[ -f "${skills_root}/medium-article-writer/adl-skill.yaml" ]]
[[ -f "${skills_root}/medium-article-writer/agents/openai.yaml" ]]
[[ -f "${skills_root}/medium-article-writer/references/medium-writing-playbook.md" ]]
[[ -f "${skills_root}/medium-article-writer/references/output-contract.md" ]]

grep -Fq 'id: "medium_article_writer.v1"' "${skills_root}/medium-article-writer/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/MEDIUM_ARTICLE_WRITER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/medium-article-writer/adl-skill.yaml"
grep -Fq "policy.writing_mode_must_be_explicit" "${skills_root}/medium-article-writer/adl-skill.yaml"
grep -Fq "policy.stop_before_publish_must_be_true" "${skills_root}/medium-article-writer/adl-skill.yaml"
grep -Fq "reviewer-friendly Medium article packet" "${skills_root}/medium-article-writer/SKILL.md"
grep -Fq "stopping before publication" "${skills_root}/medium-article-writer/SKILL.md"
grep -Fq "This skill stops at a reviewable draft packet." "${skills_root}/medium-article-writer/references/medium-writing-playbook.md"
grep -Fq "Publication Attempted: true | false" "${skills_root}/medium-article-writer/references/output-contract.md"

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/medium-article-writer/SKILL.md"

echo "PASS test_medium_article_writer_skill_contracts"
