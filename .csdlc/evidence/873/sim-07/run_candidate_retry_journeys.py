#!/usr/bin/env python3
"""Drive the frozen candidate through the declared common retry corpus."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
from typing import Any

import run_predecessor_retry_journeys as common


ISSUE = 1505
RAW_SCHEMA = common.RAW_SCHEMA


def candidate_plan(root: Path) -> dict[str, Any]:
    path = root / ".git/installed-candidate/plan.json"
    plan = json.loads(path.read_text(encoding="utf-8"))
    plan["publication"]["body"] = f"Closes #{ISSUE}"
    common.write_json(path, plan)
    return plan


def tree_digest(path: Path) -> str:
    digest = hashlib.sha256()
    if not path.exists():
        digest.update(b"missing\0")
        return digest.hexdigest()
    for child in sorted(path.rglob("*")):
        relative = child.relative_to(path).as_posix().encode()
        digest.update(relative + b"\0")
        if child.is_symlink():
            digest.update(b"link\0" + os.readlink(child).encode())
        elif child.is_file():
            digest.update(b"file\0" + child.read_bytes())
        else:
            digest.update(b"dir\0")
    return digest.hexdigest()


def checkout_content_digest(checkout: Path) -> str:
    """Bind tracked/untracked bytes plus ignored native state; omit only remote call telemetry."""
    digest = hashlib.sha256()
    listed = subprocess.check_output(
        ["git", "-C", str(checkout), "ls-files", "-c", "-o", "--exclude-standard", "-z"]
    )
    for encoded in sorted(value for value in listed.split(b"\0") if value):
        relative = encoded.decode("utf-8")
        if relative.endswith("/remote-requests"):
            continue
        path = checkout / relative
        digest.update(encoded + b"\0")
        if path.is_symlink():
            digest.update(b"link\0" + os.readlink(path).encode())
        elif path.is_file():
            digest.update(b"file\0" + path.read_bytes())
        else:
            digest.update(b"missing\0")
    native = checkout / ".csdlc"
    if native.exists():
        for child in sorted(native.rglob("*")):
            relative = child.relative_to(checkout).as_posix()
            if relative.endswith("/remote-requests"):
                continue
            digest.update(relative.encode() + b"\0")
            if child.is_symlink():
                digest.update(b"link\0" + os.readlink(child).encode())
            elif child.is_file():
                digest.update(b"file\0" + child.read_bytes())
            else:
                digest.update(b"dir\0")
    return digest.hexdigest()


def guard_snapshot(root: Path, linked: Path, cleanup_control: Path | None) -> str:
    digest = hashlib.sha256()
    common_dir = Path(subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", "--path-format=absolute", "--git-common-dir"], text=True
    ).strip())
    for checkout in (root, linked, cleanup_control):
        if checkout is None or not checkout.exists():
            continue
        status = subprocess.check_output(
            ["git", "-C", str(checkout), "status", "--porcelain", "--untracked-files=all"]
        )
        digest.update(str(checkout.resolve()).encode() + b"\0" + status)
        digest.update(checkout_content_digest(checkout).encode())
        digest.update(tree_digest(checkout / ".csdlc/v3").encode())
    digest.update(tree_digest(common_dir / "csdlc-v3/local").encode())
    for evidence in (linked / f".csdlc/evidence/{ISSUE}", (cleanup_control / f".csdlc/evidence/{ISSUE}") if cleanup_control else None):
        if evidence is None:
            continue
        for name in ("remote-effects", "remote-pr.json"):
            path = evidence / name
            digest.update(name.encode() + b"\0" + (path.read_bytes() if path.is_file() else b"missing"))
    return digest.hexdigest()


def assert_exact_guard(attempt: dict[str, Any], before: str, after: str) -> None:
    result = attempt["result"]
    envelope = result["envelope"]
    effects = envelope.get("effects")
    exact = (
        attempt.get("exit_code") == 2
        and result.get("performed_mutation") in {None, False}
        and result.get("native_effect_truth") not in {"performed", "created", "updated"}
        and isinstance(effects, dict)
        and effects.get("outcome") == "unknown"
        and before == after
    )
    attempt["guard_invariants"] = {
        "before_sha256": before,
        "after_sha256": after,
        "equal": before == after,
        "exit_code": attempt.get("exit_code"),
        "effects_outcome": effects.get("outcome") if isinstance(effects, dict) else None,
        "performed_mutation": result.get("performed_mutation"),
    }
    if not exact:
        raise RuntimeError("guard did not fail closed with unchanged local and remote effects")


def invoke(
    args: argparse.Namespace,
    attempts: list[dict[str, Any]],
    logs: Path,
    step: dict[str, Any],
    root: Path,
    linked: Path,
    cwd: Path,
    input_value: Any,
    env: dict[str, str] | None = None,
    actual_tokens: list[str] | None = None,
) -> dict[str, Any]:
    canonical = step["argv"]["candidate"]
    actual = common.expand(actual_tokens or canonical, root, linked)
    code, stdout, stderr, elapsed = common.run([str(args.binary), *actual], cwd, env)
    result = common.envelope_result(stdout)
    attempt_id = f"candidate-{args.scenario}-{len(attempts) + 1}"
    (logs / f"{attempt_id}.stdout.json").write_text(stdout, encoding="utf-8")
    (logs / f"{attempt_id}.stderr.log").write_text(stderr, encoding="utf-8")
    (logs / f"{attempt_id}.exit-status").write_text(f"{code}\n", encoding="utf-8")
    attempts.append(
        {
            "argv": canonical,
            "topology": "linked" if cwd == linked else "primary",
            "attempt_id": attempt_id,
            "exit_code": code,
            "elapsed_millis": elapsed,
            "result": result,
            "input": input_value,
            "executed_argv": actual,
        }
    )
    return result


def raw(args: argparse.Namespace, attempts: list[dict[str, Any]], topology: dict[str, Any] | None = None) -> dict[str, Any]:
    value: dict[str, Any] = {
        "schema": RAW_SCHEMA,
        "issue": 873,
        "provenance": {
            "source_head": args.source_revision,
            "source_has_uncommitted_changes": False,
            "installed_binary_blake3": args.binary_blake3,
            "source_binding": "Exact frozen candidate bytes; fresh isolated common-scenario adapter fixture",
        },
        "attempted": len(attempts),
        "attempts": attempts,
    }
    if topology is not None:
        value["cleanup_topology"] = topology
    return value


def execute_primary(args: argparse.Namespace, root: Path, scenario: dict[str, Any]) -> dict[str, Any]:
    plan = candidate_plan(root)
    linked = common.fixture_worktree_parent(root) / "adl-issue-1505-installed-intent-fixture"
    changes = {
        "schema": "csdlc.v3.intent_changes.v1",
        "amendment": {"class": "scope_acceptance", "transition_approved": True},
        "cards": {"sip": {"title": "Edited through ordinary intent"}},
    }
    common.write_json(root / ".git/installed-candidate/changes.json", changes)
    attempts: list[dict[str, Any]] = []
    logs = Path(args.logs)
    logs.mkdir(parents=True, exist_ok=True)
    fixture_head = subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", "HEAD"], text=True
    ).strip()
    env = common.configure_fake_remote(root, fixture_head)
    linked_steps = {
        "status-bound-linked", "validate-bound-linked", "linked-edit-preview-guard",
        "linked-edit", "validate-projection",
    }
    for step in scenario["semantic_steps"]:
        step_id = step["id"]
        cwd = linked if step_id in linked_steps else root
        input_value = changes if step_id in {"linked-edit-preview-guard", "linked-edit", "primary-edit"} else plan if step_id in {"prepare-preview-guard", "prepare"} else {"issue": ISSUE}
        guard = step_id in {"prepare-preview-guard", "linked-edit-preview-guard"}
        before = guard_snapshot(root, linked, None) if guard else ""
        result = invoke(args, attempts, logs, step, root, linked, cwd, input_value, env)
        reason = result["envelope"].get("reason_code")
        expected = "intent_preview_argument_not_supported" if step_id in {"prepare-preview-guard", "linked-edit-preview-guard"} else "command_completed"
        if reason != expected:
            raise RuntimeError(f"{step_id} drifted: {reason}")
        if guard:
            assert_exact_guard(attempts[-1], before, guard_snapshot(root, linked, None))
    if len(attempts) != 13:
        raise RuntimeError("candidate primary common corpus must contain exactly 13 attempts")
    return raw(args, attempts)


def move_bound_worktree_to_cleanup_control(root: Path, linked: Path, logs: Path) -> tuple[Path, dict[str, Any]]:
    """Move the completed bound checkout to the cleanup slot and retain a dirty control peer."""
    parent = common.fixture_worktree_parent(root)
    control = parent / "cleanup-control"
    if control.exists():
        raise RuntimeError("cleanup control already exists")
    head = subprocess.check_output(["git", "-C", str(linked), "rev-parse", "HEAD"], text=True).strip()
    baseline_head = subprocess.check_output(["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip()
    status = subprocess.check_output(["git", "-C", str(linked), "status", "--porcelain", "--untracked-files=all"], text=True)
    if status:
        raise RuntimeError("bound cleanup candidate is not untouched before topology handoff")
    subprocess.run(["git", "-C", str(root), "worktree", "move", str(linked), str(control)], check=True, capture_output=True, text=True)
    subprocess.run(["git", "-C", str(root), "worktree", "add", "--detach", str(linked), head], check=True, capture_output=True, text=True)
    retained = linked / "issue-873-active-retained.tmp"
    retained.write_text("synthetic retained dirty active checkout\n", encoding="utf-8")
    control_head = subprocess.check_output(["git", "-C", str(control), "rev-parse", "HEAD"], text=True).strip()
    control_status = subprocess.check_output(["git", "-C", str(control), "status", "--porcelain", "--untracked-files=all"], text=True)
    active_status = subprocess.check_output(["git", "-C", str(linked), "status", "--porcelain", "--untracked-files=all"], text=True)
    primary_inventory = subprocess.check_output(["git", "-C", str(root), "ls-tree", "-r", "--full-tree", baseline_head])
    control_inventory = subprocess.check_output(["git", "-C", str(control), "ls-tree", "-r", "--full-tree", control_head])
    registrations = subprocess.check_output(["git", "-C", str(root), "worktree", "list", "--porcelain"], text=True)
    registered = {Path(line[9:]).resolve() for line in registrations.splitlines() if line.startswith("worktree ")}
    if control_head != head or head != baseline_head or primary_inventory != control_inventory or control_status or not active_status or not {root.resolve(), linked.resolve(), control.resolve()}.issubset(registered):
        raise RuntimeError("candidate cleanup-control topology is not exact")
    topology = {
        "schema": "csdlc.v3.issue873.cleanup_control_topology.v1",
        "primary_root": str(root.resolve()),
        "active_issue_worktree": str(linked.resolve()),
        "cleanup_control_worktree": str(control.resolve()),
        "baseline_head": baseline_head,
        "control_head": control_head,
        "tracked_inventory_sha256": hashlib.sha256(primary_inventory).hexdigest(),
        "control_status_porcelain": control_status,
        "active_status_porcelain": active_status,
        "registered_paths": sorted(str(path) for path in registered),
        "active_issue_retained": True,
        "handoff": "git_worktree_move_then_detached_retained_peer",
    }
    common.write_json(logs / "cleanup-control-topology.json", topology)
    return control, topology


def review_input(linked: Path, helper: Path, head: str) -> dict[str, Any]:
    proof_path = linked / f".csdlc/v3/issues/{ISSUE}/proof.json"
    proof = json.loads(proof_path.read_text(encoding="utf-8"))
    proof_digest = common.blake3_file(helper, proof_path)
    receipt = {
        "schema": "csdlc.v3.typed_review_receipt.v1",
        "repository": "agent-logic/agent-design-language",
        "issue": ISSUE,
        "implementer": "synthetic-fixture-author",
        "reviewer": "synthetic-independent-fixture-reviewer",
        "reviewed_revision": head,
        "expected_head_sha": head,
        "evidence_digest": proof["payload_digest"],
        "publication_linkage": {"repository": "agent-logic/agent-design-language", "issue": ISSUE, "mode": "closing"},
    }
    fields = [receipt["schema"], receipt["repository"], str(ISSUE), receipt["implementer"], receipt["reviewer"], head, head, receipt["evidence_digest"]]
    payload = linked / f".csdlc/evidence/{ISSUE}/review-payload.bin"
    payload.parent.mkdir(parents=True, exist_ok=True)
    payload.write_bytes(b"".join(value.encode() + b"\0" for value in fields))
    return {"receipt_digest": common.blake3_file(helper, payload), "receipt": receipt, "proof_path": f".csdlc/v3/issues/{ISSUE}/proof.json", "proof_digest": proof_digest}


def execute_terminal(args: argparse.Namespace, root: Path, scenario: dict[str, Any]) -> dict[str, Any]:
    plan = candidate_plan(root)
    linked = common.fixture_worktree_parent(root) / "adl-issue-1505-installed-intent-fixture"
    attempts: list[dict[str, Any]] = []
    logs = Path(args.logs)
    logs.mkdir(parents=True, exist_ok=True)
    helper = logs / "blake3-digest"
    common.compile_blake3_helper(Path(args.blake3_source), Path(args.target_dir), helper)
    fixture_head = subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", "HEAD"], text=True
    ).strip()
    env: dict[str, str] | None = common.configure_fake_remote(root, fixture_head)
    head: str | None = None
    review: dict[str, Any] | None = None
    cleanup_control: Path | None = None
    topology: dict[str, Any] | None = None
    tracked_original: bytes | None = None
    preview_digest: str | None = None
    for step in scenario["semantic_steps"]:
        step_id = step["id"]
        input_value: Any = {"issue": ISSUE}
        cwd = linked if step_id in {"proof", "review", "finish-linked"} else root
        if step_id == "prepare":
            input_value = plan
        elif step_id == "proof":
            head = subprocess.check_output(["git", "-C", str(linked), "rev-parse", "HEAD"], text=True).strip()
        elif step_id == "review":
            assert head is not None
            review = review_input(linked, helper, head)
            common.write_json(root / ".git/installed-candidate/review.json", review)
            input_value = review
        elif step_id == "publish":
            assert head is not None
            if head != fixture_head:
                raise RuntimeError("candidate linked head differs from the synthetic remote head")
        elif step_id == "github-pr-create":
            input_value = {"action": "pull_request_ready"}
            common.write_json(root / ".git/installed-candidate/operation.json", input_value)
        elif step_id in {"merge-internal-input-guard", "merge-remote-apply", "merge-idempotent-observation"}:
            input_value = {"action": "pull_request_merge", "base": "main", "method": "merge"}
            if step_id != "merge-internal-input-guard":
                input_value["operator_approval"] = "synthetic operator authorizes only fixture PR639 exact candidate merge"
            common.write_json(root / ".git/installed-candidate/merge.json", input_value)
        elif step_id == "cleanup-foreign-dirty-guard":
            cleanup_control, topology = move_bound_worktree_to_cleanup_control(root, linked, logs)
            foreign = cleanup_control / "issue-873-foreign-dirty.tmp"
            foreign.write_text("synthetic foreign dirty guard\n", encoding="utf-8")
            input_value = {"issue": ISSUE, "cleanup_candidate": str(cleanup_control), "active_issue_worktree": str(linked)}
        elif step_id == "cleanup-tracked-dirty-guard":
            assert cleanup_control is not None
            (cleanup_control / "issue-873-foreign-dirty.tmp").unlink()
            tracked = cleanup_control / "tracked"
            tracked_original = tracked.read_bytes()
            tracked.write_bytes(tracked_original + b"synthetic tracked dirty guard\n")
            input_value = {"issue": ISSUE, "cleanup_candidate": str(cleanup_control), "active_issue_worktree": str(linked)}
        elif step_id == "cleanup-preview":
            assert cleanup_control is not None and tracked_original is not None
            (cleanup_control / "tracked").write_bytes(tracked_original)
            input_value = {"issue": ISSUE, "cleanup_candidate": str(cleanup_control), "active_issue_worktree": str(linked)}
        actual_tokens: list[str] | None = None
        if step_id == "cleanup-stale-preview-guard":
            assert preview_digest is not None and cleanup_control is not None
            stale = ("0" if preview_digest[0] != "0" else "1") + preview_digest[1:]
            actual_tokens = ["clean", str(ISSUE), "--execute", "--preview", stale]
            input_value = {"issue": ISSUE, "cleanup_candidate": str(cleanup_control), "active_issue_worktree": str(linked), "preview_receipt_digest": stale}
        elif step_id == "cleanup-refresh-preview":
            assert cleanup_control is not None
            input_value = {"issue": ISSUE, "cleanup_candidate": str(cleanup_control), "active_issue_worktree": str(linked)}
        elif step_id == "cleanup-execute":
            assert preview_digest is not None and cleanup_control is not None
            actual_tokens = ["clean", str(ISSUE), "--execute", "--preview", preview_digest]
            input_value = {"issue": ISSUE, "cleanup_candidate": str(cleanup_control), "active_issue_worktree": str(linked), "preview_receipt_digest": preview_digest}
        guard = step_id in {
            "merge-internal-input-guard", "cleanup-foreign-dirty-guard",
            "cleanup-tracked-dirty-guard", "cleanup-stale-preview-guard",
        }
        before = guard_snapshot(root, linked, cleanup_control) if guard else ""
        result = invoke(args, attempts, logs, step, root, linked, cwd, input_value, env, actual_tokens)
        reason = result["envelope"].get("reason_code")
        guards = {
            "merge-internal-input-guard": "intent_merge_internal_inputs_denied",
            "cleanup-foreign-dirty-guard": "cleanup_archive_foreign_or_tracked_dirty",
            "cleanup-tracked-dirty-guard": "cleanup_archive_foreign_or_tracked_dirty",
            "cleanup-stale-preview-guard": "intent_cleanup_preview_stale",
        }
        if reason != guards.get(step_id, "command_completed"):
            raise RuntimeError(f"{step_id} drifted: {reason}")
        if guard:
            assert_exact_guard(attempts[-1], before, guard_snapshot(root, linked, cleanup_control))
        if step_id in {"cleanup-preview", "cleanup-refresh-preview"}:
            cleanup = result.get("result", {}).get("cleanup", {})
            if cleanup.get("decision") != "removable" or not isinstance(cleanup.get("receipt_digest"), str):
                raise RuntimeError("candidate cleanup preview did not retain a removable receipt")
            preview_digest = cleanup["receipt_digest"]
        if step_id == "cleanup-execute":
            assert cleanup_control is not None
            if cleanup_control.exists() or not linked.is_dir():
                raise RuntimeError("candidate cleanup did not remove only the cleanup control")
    if len(attempts) != 18 or topology is None:
        raise RuntimeError("candidate terminal common corpus must contain exactly 18 attempts and exact topology")
    return raw(args, attempts, topology)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--binary-blake3", required=True)
    parser.add_argument("--source-revision", required=True)
    parser.add_argument("--scenario-map", type=Path, required=True)
    parser.add_argument("--harness", type=Path, required=True)
    parser.add_argument("--slot", type=Path, required=True)
    parser.add_argument("--fixture", type=Path, required=True)
    parser.add_argument("--logs", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--target-dir", type=Path, required=True)
    parser.add_argument("--blake3-source", type=Path, required=True)
    parser.add_argument("--scenario", choices=["primary-linked-edit", "terminal-journey"], required=True)
    args = parser.parse_args()
    scenario_map = json.loads(args.scenario_map.read_text(encoding="utf-8"))
    scenario = next(value for value in scenario_map["scenarios"] if value["id"] == args.scenario)
    fixture_filter = "installed_prepare_bind_edit_and_observations_use_canonical_context" if args.scenario == "primary-linked-edit" else "installed_merge_finish_and_exact_bound_cleanup_preserve_authority_and_archive_residue"
    common.capture_fixture(args.harness.resolve(), args.slot.resolve(), fixture_filter, args.fixture.resolve())
    common.relocate_fixture(args.fixture.resolve())
    value = execute_primary(args, args.fixture.resolve(), scenario) if args.scenario == "primary-linked-edit" else execute_terminal(args, args.fixture.resolve(), scenario)
    common.write_json(args.output, value)


if __name__ == "__main__":
    main()
