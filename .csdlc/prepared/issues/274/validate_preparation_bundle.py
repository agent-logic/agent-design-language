#!/usr/bin/env python3
"""Fail-closed pre-bind packet validator for current-repo issue #274."""

from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess


ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE = ROOT / ".csdlc/issues/274"
PREP = ROOT / ".csdlc/prepared/issues/274"
CARD_KINDS = {"sip", "stp", "spp", "vpp", "srp", "sor"}
DEPENDENCIES = (191, 199, 200, 201, 202, 203, 272, 273, 350, 356, 358)
OWNED_PRODUCT = {
    "adl-runtime/src/distributed/observatory_serving_eligibility.rs",
    "adl-runtime/tests/distributed_observatory_serving_eligibility.rs",
}
FORBIDDEN_PRODUCT = {
    "adl-runtime/src/distributed/serving_authority.rs",
    "adl-runtime/src/distributed/shepherd_serving_eligibility.rs",
    "adl-runtime/tests/distributed_shepherd_serving_eligibility.rs",
    "adl-runtime/src/distributed/authority_store_adapters.rs",
}


def fail(message: str) -> None:
    print(json.dumps({"schema": "adl.issue274.preparation.v1", "status": "failed", "message": message}, sort_keys=True))
    raise SystemExit(1)


def read_json(path: pathlib.Path) -> dict:
    try:
        value = json.loads(path.read_text())
    except (FileNotFoundError, json.JSONDecodeError) as exc:
        fail(f"cannot read {path.relative_to(ROOT)}: {exc}")
    if not isinstance(value, dict):
        fail(f"expected object: {path.relative_to(ROOT)}")
    return value


def run(*argv: str) -> str:
    result = subprocess.run(argv, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    if result.returncode:
        fail(f"command failed ({result.returncode}): {' '.join(argv)}: {result.stderr.strip()}")
    return result.stdout.strip()


def sha256(path: pathlib.Path) -> str:
    try:
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except FileNotFoundError:
        fail(f"missing {path.relative_to(ROOT)}")


def main() -> None:
    index = read_json(ISSUE / "index.json")
    if index.get("issue") != 274 or index.get("repository") != "agent-logic/agent-design-language":
        fail("wrong issue/repository identity")
    if index.get("phase") != "initialized" or not isinstance(index.get("generation"), int):
        fail("packet must remain initialized with a typed generation")
    if index.get("branch") is not None or index.get("worktree") is not None:
        fail("packet is unexpectedly bound")
    digest = index.get("digest")
    if not isinstance(digest, str) or len(digest) != 64 or any(c not in "0123456789abcdef" for c in digest):
        fail("invalid typed digest")
    cards = index.get("cards")
    if not isinstance(cards, dict) or set(cards) != CARD_KINDS:
        fail("not exactly six canonical cards")

    texts = [(PREP / "design.md").read_text(), (PREP / "diagram.mmd").read_text()]
    for kind in sorted(CARD_KINDS):
        values = read_json(ISSUE / "cards" / f"{kind}.values.json")
        if values.get("identity", {}).get("issue") != 274:
            fail(f"{kind} issue identity drift")
        if values.get("content", {}).get("card_kind") != kind:
            fail(f"{kind} card identity drift")
        texts.extend([(ISSUE / "cards" / f"{kind}.values.json").read_text(), (ISSUE / "cards" / f"{kind}.md").read_text()])
    combined = "\n".join(texts)

    for path in OWNED_PRODUCT:
        if path not in combined:
            fail(f"missing owned product path: {path}")
    spp = read_json(ISSUE / "cards" / "spp.values.json")
    affected = set(spp.get("content", {}).get("values", {}).get("affected_areas", []))
    if not OWNED_PRODUCT.issubset(affected):
        fail("SPP affected areas omit an owned product path")
    overlap = FORBIDDEN_PRODUCT & affected
    if overlap:
        fail(f"forbidden product ownership appears in SPP affected areas: {sorted(overlap)}")
    for marker in ("#205 remains coordination-only", "#273 is terminal and ancestral", "shared registration", "#272", "No UI"):
        if marker not in combined:
            fail(f"required boundary marker missing: {marker}")

    common = pathlib.Path(run("git", "rev-parse", "--path-format=absolute", "--git-common-dir"))
    deps = {}
    for number in DEPENDENCIES:
        terminal = read_json(common / "csdlc-v2/derived-terminal" / f"{number}.json")
        if terminal.get("issue") != number or terminal.get("repository") != "agent-logic/agent-design-language":
            fail(f"terminal identity mismatch for #{number}")
        if terminal.get("disposition") != "merged" or terminal.get("issue_state") != "closed_by_merged_pr":
            fail(f"#{number} is not canonical merged authority")
        merge_sha = terminal.get("merge_sha")
        if not isinstance(merge_sha, str) or not merge_sha:
            fail(f"#{number} terminal cache lacks merge SHA")
        run("git", "merge-base", "--is-ancestor", merge_sha, "HEAD")
        deps[str(number)] = {"digest": terminal.get("digest"), "merge_sha": merge_sha}

    print(json.dumps({
        "schema": "adl.issue274.preparation.v1",
        "status": "passed",
        "issue": 274,
        "phase": index["phase"],
        "generation": index["generation"],
        "digest": digest,
        "design_sha256": sha256(PREP / "design.md"),
        "diagram_sha256": sha256(PREP / "diagram.mmd"),
        "dependencies": deps,
        "registration_gate": "implementation_serial_after_terminal_ancestral_273_unless_fresh_review_proves_no_mod_rs_touch",
        "product_proof": "deferred_until_future_typed_bind",
    }, sort_keys=True))


if __name__ == "__main__":
    main()
