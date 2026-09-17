#!/usr/bin/env python3
"""Drive the retained typed predecessor against fresh isolated installed fixtures."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
from typing import Any

RAW_SCHEMA = "csdlc.v3.installed_intent_attempts.v1"
COMMANDS = [
    "prepare_issue", "bind_worktree", "edit_cards", "plan_pvf",
    "doctor", "schedule", "shepherd", "eligibility",
]


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def run(argv: list[str], cwd: Path, env: dict[str, str] | None = None) -> tuple[int, str, str, int]:
    started = time.monotonic_ns()
    completed = subprocess.run(argv, cwd=cwd, env=env, text=True, capture_output=True)
    elapsed = (time.monotonic_ns() - started) // 1_000_000
    return completed.returncode, completed.stdout, completed.stderr, elapsed


def capture_fixture(harness: Path, slot: Path, test_filter: str, destination: Path) -> Path:
    if destination.exists():
        raise RuntimeError(f"capture destination already exists: {destination}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    backup = slot.with_name(f"{slot.name}.issue873-capture-{os.getpid()}")
    shutil.copy2(slot, backup)
    shim = f'''#!/bin/sh
set -eu
for arg in "$@"; do
 case "$arg" in
 */.git/installed-candidate/plan.json)
   root=${{arg%/.git/installed-candidate/plan.json}}
   if ! test -e "$ADL_ISSUE873_CAPTURE_DEST"; then
    cp -R "$root" "$ADL_ISSUE873_CAPTURE_DEST"
   fi
   break
   ;;
 esac
done
printf '%s\\n' '{{"schema":"csdlc.v3.command_failure.v1","status":"failed","reason_code":"fixture_capture_only","correlation_id":"fixture-capture-only"}}'
exit 2
'''
    try:
        slot.write_text(shim, encoding="utf-8")
        slot.chmod(0o755)
        env = os.environ.copy()
        env["ADL_ISSUE873_CAPTURE_DEST"] = str(destination)
        subprocess.run(
            [str(harness), test_filter, "--exact", "--nocapture"],
            cwd=harness.parents[3], env=env, text=True, capture_output=True,
        )
        if not (destination / ".git/installed-candidate/plan.json").is_file():
            raise RuntimeError("installed harness did not yield a fresh fixture")
    finally:
        os.replace(backup, slot)
    return destination


def relocate_fixture(root: Path) -> None:
    policy_path = root / ".adl/worktree-policy.json"
    policy = json.loads(policy_path.read_text(encoding="utf-8"))
    parent = Path(policy["required_parent"])
    if not parent.is_absolute() or parent.name != "worktrees":
        raise RuntimeError("captured fixture has an invalid worktree policy")
    parent.mkdir(parents=True, exist_ok=False)


def fixture_worktree_parent(root: Path) -> Path:
    policy = json.loads((root / ".adl/worktree-policy.json").read_text(encoding="utf-8"))
    return Path(policy["required_parent"])


def create_cleanup_control(root: Path, linked: Path, logs: Path) -> Path:
    """Create the untouched cleanup target used by the retained #868 journey."""
    approved_parent = fixture_worktree_parent(root)
    control = approved_parent / "cleanup-control"
    if control.exists():
        raise RuntimeError("cleanup control already exists")
    baseline_head = subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", "HEAD"], text=True
    ).strip()
    subprocess.run(
        ["git", "-C", str(root), "worktree", "add", "--detach", str(control), baseline_head],
        text=True, capture_output=True, check=True,
    )
    control_head = subprocess.check_output(
        ["git", "-C", str(control), "rev-parse", "HEAD"], text=True
    ).strip()
    control_status = subprocess.check_output(
        ["git", "-C", str(control), "status", "--porcelain", "--untracked-files=all"],
        text=True,
    )
    primary_common = Path(subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", "--path-format=absolute", "--git-common-dir"],
        text=True,
    ).strip()).resolve()
    control_common = Path(subprocess.check_output(
        ["git", "-C", str(control), "rev-parse", "--path-format=absolute", "--git-common-dir"],
        text=True,
    ).strip()).resolve()
    primary_inventory = subprocess.check_output(
        ["git", "-C", str(root), "ls-tree", "-r", "--full-tree", baseline_head]
    )
    control_inventory = subprocess.check_output(
        ["git", "-C", str(control), "ls-tree", "-r", "--full-tree", control_head]
    )
    registrations = subprocess.check_output(
        ["git", "-C", str(root), "worktree", "list", "--porcelain"], text=True
    )
    registered_paths = {
        Path(line.removeprefix("worktree ")).resolve()
        for line in registrations.splitlines()
        if line.startswith("worktree ")
    }
    required_paths = {root.resolve(), linked.resolve(), control.resolve()}
    if baseline_head != control_head:
        raise RuntimeError("cleanup control head differs from the normalized baseline")
    if control_status:
        raise RuntimeError("cleanup control is not untouched and clean")
    if primary_common != control_common:
        raise RuntimeError("cleanup control does not share the primary git common directory")
    if primary_inventory != control_inventory:
        raise RuntimeError("cleanup control tracked inventory differs from the primary baseline")
    if not required_paths.issubset(registered_paths):
        raise RuntimeError("cleanup topology lacks an exact registered path")
    inventory_digest = hashlib.sha256(primary_inventory).hexdigest()
    write_json(
        logs / "cleanup-control-topology.json",
        {
            "schema": "csdlc.v3.issue873.cleanup_control_topology.v1",
            "primary_root": str(root.resolve()),
            "active_issue_worktree": str(linked.resolve()),
            "cleanup_control_worktree": str(control.resolve()),
            "approved_parent": str(approved_parent.resolve()),
            "baseline_head": baseline_head,
            "control_head": control_head,
            "primary_git_common_dir": str(primary_common),
            "control_git_common_dir": str(control_common),
            "tracked_inventory_sha256": inventory_digest,
            "control_status_porcelain": control_status,
            "registered_paths": sorted(str(path) for path in registered_paths),
            "active_issue_retained": True,
        },
    )
    return control


def envelope_result(stdout: str) -> dict[str, Any]:
    try:
        value = json.loads(stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(f"predecessor stdout was not one JSON document: {error}") from error
    if not isinstance(value, dict) or not isinstance(value.get("envelope"), dict):
        raise RuntimeError("predecessor result lacks the common envelope")
    return value


def current_version(result: dict[str, Any], digest: str | None, generation: int | None) -> tuple[str | None, int | None]:
    owner = result.get("result")
    if isinstance(owner, dict):
        if isinstance(owner.get("digest"), str):
            digest = owner["digest"]
        if isinstance(owner.get("generation"), int):
            generation = owner["generation"]
    return digest, generation


def expand(tokens: list[str], root: Path, linked: Path) -> list[str]:
    return [token.replace("$FIXTURE_ROOT", str(root)).replace("$BOUND_WORKTREE", str(linked)) for token in tokens]


def compile_blake3_helper(source: Path, target_dir: Path, output: Path) -> None:
    libraries = sorted((target_dir / "debug/deps").glob("libblake3-*.rlib"))
    if not libraries:
        raise RuntimeError("the existing validation target lacks libblake3")
    subprocess.run(
        [
            "rustc", "--edition=2021", str(source), "--extern", f"blake3={libraries[0]}",
            "-L", f"dependency={target_dir / 'debug/deps'}", "-o", str(output),
        ],
        check=True,
    )


def blake3_file(helper: Path, path: Path) -> str:
    return subprocess.check_output([str(helper), str(path)], text=True).strip()


def linked_terminal_state(linked: Path, issue: int) -> dict[str, str]:
    return {
        "repository_root": str(linked),
        "state_path": f".csdlc/v3/issues/{issue}/terminal.json",
        "receipt_path": f".csdlc/evidence/{issue}/terminal-receipt.json",
    }


def primary_terminal_state(primary: Path, issue: int) -> dict[str, str]:
    git_common_dir = Path(
        subprocess.check_output(
            ["git", "-C", str(primary), "rev-parse", "--path-format=absolute", "--git-common-dir"],
            text=True,
        ).strip()
    )
    return {
        "repository_root": str(primary),
        "state_path": str(git_common_dir / f"csdlc-v3/local/v3/issues/{issue}/terminal.json"),
        "receipt_path": str(git_common_dir / f"csdlc-v3/local/evidence/{issue}/terminal-receipt.json"),
    }


def terminal_topology(step_id: str) -> str:
    if step_id in {"proof-input-admission", "proof", "review", "finish-linked"}:
        return "linked"
    return "primary"


def merge_review_receipt_path(primary: Path) -> Path:
    path = (primary / ".csdlc/evidence/1505/review.json").resolve()
    try:
        path.relative_to(primary.resolve())
    except ValueError as error:
        raise RuntimeError("merge review receipt escapes primary fixture") from error
    return path


def stage_review_receipt_for_primary(linked: Path, primary: Path, logs: Path) -> Path:
    source = linked / ".csdlc/evidence/1505/review.json"
    destination = merge_review_receipt_path(primary)
    if not source.is_file():
        raise RuntimeError("linked typed review receipt is missing")
    if destination.exists():
        raise RuntimeError("primary review receipt destination already exists")
    source_bytes = source.read_bytes()
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(source_bytes)
    destination_bytes = destination.read_bytes()
    if destination_bytes != source_bytes:
        raise RuntimeError("primary review receipt copy changed bytes")
    digest = __import__("hashlib").sha256(source_bytes).hexdigest()
    write_json(
        logs / "primary-review-receipt-copy.json",
        {
            "schema": "csdlc.v3.issue873.review_receipt_copy.v1",
            "source": str(source),
            "destination": str(destination),
            "source_sha256": digest,
            "destination_sha256": __import__("hashlib").sha256(destination_bytes).hexdigest(),
            "byte_equal": True,
        },
    )
    return destination


def execute_primary(args: argparse.Namespace, root: Path, scenario: dict[str, Any]) -> dict[str, Any]:
    evidence = root / ".git/installed-predecessor"
    evidence.mkdir(parents=True, exist_ok=True)
    plan = json.loads((root / ".git/installed-candidate/plan.json").read_text(encoding="utf-8"))
    linked = fixture_worktree_parent(root) / "adl-issue-1505-installed-intent-fixture"
    base = {
        "issue": 1505,
        "title": "Installed intent fixture",
        "repository": "agent-logic/agent-design-language",
        "branch": "codex/1505-installed-intent-fixture",
        "worktree": str(linked),
        "registry_version": "1.0.5",
        "commands": COMMANDS,
        "card_values": plan["cards"],
    }
    registrations: list[dict[str, Any]] = []
    write_json(evidence / "registrations.json", registrations)
    digest: str | None = None
    generation: int | None = None
    attempts: list[dict[str, Any]] = []
    logs = Path(args.logs)
    logs.mkdir(parents=True, exist_ok=True)

    def request_for(step_id: str) -> tuple[Path, dict[str, Any]]:
        nonlocal registrations
        request = dict(base)
        if digest is not None:
            request["expected_lifecycle_digest"] = digest
        if step_id in {"linked-edit-preview-guard", "linked-edit", "primary-edit"}:
            request["card_updates"] = {"sip": {"title": "Edited through ordinary intent"}}
        names = {
            "prepare-preview-guard": "issue-request.json", "prepare": "issue-request.json",
            "bind": "bind-request.json", "linked-edit-preview-guard": "linked-edit-request.json",
            "linked-edit": "linked-edit-request.json", "primary-edit": "primary-edit-request.json",
        }
        name = names.get(step_id, f"{step_id}-request.json")
        path = evidence / name
        write_json(path, request)
        write_json(evidence / "registrations.json", registrations)
        return path, request

    def invoke(step: dict[str, Any], tokens: list[str], topology: str, input_value: Any, suffix: str = "") -> dict[str, Any]:
        nonlocal digest, generation
        canonical = tokens
        actual = expand(tokens, root, linked)
        cwd = linked if topology == "linked" else root
        code, stdout, stderr, elapsed = run([str(args.binary), *actual], cwd)
        result = envelope_result(stdout)
        attempt_id = f"predecessor-primary-{len(attempts)+1}{suffix}"
        (logs / f"{attempt_id}.stdout.json").write_text(stdout, encoding="utf-8")
        (logs / f"{attempt_id}.stderr.log").write_text(stderr, encoding="utf-8")
        (logs / f"{attempt_id}.exit-status").write_text(f"{code}\n", encoding="utf-8")
        attempts.append({
            "argv": canonical, "topology": topology, "attempt_id": attempt_id,
            "exit_code": code, "elapsed_millis": elapsed, "result": result,
            "input": input_value,
        })
        digest, generation = current_version(result, digest, generation)
        return result

    for step in scenario["semantic_steps"]:
        step_id = step["id"]
        path, request = request_for(step_id)
        tokens = step["argv"]["predecessor"]
        topology = "linked" if step_id in {
            "status-bound-linked", "validate-bound-linked", "linked-edit-preview-guard",
            "linked-edit", "validate-projection",
        } else "primary"
        result = invoke(step, tokens, topology, request)
        reason = result["envelope"].get("reason_code")
        if step_id == "prepare-preview-guard":
            if reason != "usage": raise RuntimeError(f"{step_id} drifted: {reason}")
        elif step_id == "linked-edit-preview-guard":
            if reason != "usage": raise RuntimeError(f"{step_id} drifted: {reason}")
        elif step_id in {"status-bound-primary", "validate-bound-primary", "primary-edit"}:
            if reason != "issue_already_bound":
                raise RuntimeError(f"{step_id} no longer exposes predecessor retry: {reason}")
            retry_tokens = step["retry_argv"]["predecessor"]
            retry_result = invoke(step, retry_tokens, "linked", request, "-retry")
            if retry_result["envelope"].get("reason_code") not in {"command_completed", "lifecycle_digest_valid"}:
                raise RuntimeError(f"{step_id} linked retry did not complete")
        elif reason not in {"command_completed", "lifecycle_digest_valid"}:
            raise RuntimeError(f"{step_id} drifted: {reason}")
        if step_id == "prepare":
            registrations = [{"branch": base["branch"], "worktree": str(linked), "primary": False}]
            write_json(evidence / "registrations.json", registrations)

    return {
        "schema": RAW_SCHEMA,
        "issue": 873,
        "provenance": {
            "source_head": args.source_revision,
            "source_has_uncommitted_changes": False,
            "installed_binary_blake3": args.binary_blake3,
            "source_binding": "Exact retained predecessor bytes; isolated fixture generated by the installed candidate harness",
        },
        "attempted": len(attempts),
        "attempts": attempts,
    }


def execute_terminal(args: argparse.Namespace, root: Path, scenario: dict[str, Any]) -> dict[str, Any]:
    private = root / ".git/installed-predecessor"
    private.mkdir(parents=True, exist_ok=True)
    linked = fixture_worktree_parent(root) / "adl-issue-1505-installed-intent-fixture"
    plan = json.loads((root / ".git/installed-candidate/plan.json").read_text(encoding="utf-8"))
    base = {
        "issue": 1505,
        "title": "Installed intent fixture",
        "repository": "agent-logic/agent-design-language",
        "branch": "codex/1505-installed-intent-fixture",
        "worktree": str(linked),
        "registry_version": "1.0.5",
        "commands": COMMANDS,
        "card_values": plan["cards"],
    }
    registrations: list[dict[str, Any]] = []
    write_json(private / "registrations.json", registrations)
    digest: str | None = None
    generation: int | None = None
    attempts: list[dict[str, Any]] = []
    logs = Path(args.logs)
    logs.mkdir(parents=True, exist_ok=True)
    local_binary = linked / ".csdlc/evidence/1505/csdlc-predecessor"
    helper = logs / "blake3-digest"
    remote_env: dict[str, str] | None = None
    review_digest: str | None = None
    head: str | None = None
    selector_digest: str | None = None
    tracked_original: bytes | None = None
    cleanup_control: Path | None = None
    preview_digest: str | None = None

    def assert_operation_request_deserializes(step: dict[str, Any], request_path: Path) -> None:
        schema_path = request_path.with_name(f"{request_path.stem}.schema-check.json")
        schema_request = json.loads(request_path.read_text(encoding="utf-8"))
        schema_request["expected_lifecycle_digest"] = "0" * 64
        write_json(schema_path, schema_request)
        tokens = list(step["argv"]["predecessor"])
        tokens[2] = str(schema_path)
        executable = local_binary if local_binary.is_file() else Path(args.binary)
        code, stdout, stderr, _ = run(
            [str(executable), *expand(tokens, root, linked)], linked, remote_env
        )
        prefix = f"predecessor-{step['id']}-schema-check"
        (logs / f"{prefix}.stdout.json").write_text(stdout, encoding="utf-8")
        (logs / f"{prefix}.stderr.log").write_text(stderr, encoding="utf-8")
        (logs / f"{prefix}.exit-status").write_text(f"{code}\n", encoding="utf-8")
        result = envelope_result(stdout)
        reason = result["envelope"].get("reason_code")
        if reason == "typed_operational_remote_request_invalid_json":
            raise RuntimeError(f"{step['id']} request did not deserialize: {reason}")

    def invoke(step: dict[str, Any], input_value: Any, cwd: Path) -> dict[str, Any]:
        nonlocal digest, generation
        canonical = step["argv"]["predecessor"]
        actual = expand(canonical, root, linked)
        executable = local_binary if local_binary.is_file() else Path(args.binary)
        code, stdout, stderr, elapsed = run(
            [str(executable), *actual], cwd, remote_env
        )
        result = envelope_result(stdout)
        attempt_id = f"predecessor-terminal-{len(attempts)+1}"
        (logs / f"{attempt_id}.stdout.json").write_text(stdout, encoding="utf-8")
        (logs / f"{attempt_id}.stderr.log").write_text(stderr, encoding="utf-8")
        (logs / f"{attempt_id}.exit-status").write_text(f"{code}\n", encoding="utf-8")
        attempts.append({
            "argv": canonical, "topology": "linked" if cwd == linked else "primary",
            "attempt_id": attempt_id, "exit_code": code, "elapsed_millis": elapsed,
            "result": result, "input": input_value,
        })
        digest, generation = current_version(result, digest, generation)
        return result

    for step in scenario["semantic_steps"]:
        step_id = step["id"]
        request_path = private / Path(step["argv"]["predecessor"][2]).name
        request: dict[str, Any]
        if step_id == "prepare":
            request = dict(base)
        elif step_id == "bind":
            request = dict(base)
            request["expected_lifecycle_digest"] = digest
        elif step_id == "proof-input-admission":
            request = dict(base)
            request["expected_lifecycle_digest"] = digest
        elif step_id == "proof":
            if generation is None or digest is None:
                raise RuntimeError("proof requires the completed bind")
            evidence = linked / ".csdlc/evidence/1505"
            evidence.mkdir(parents=True, exist_ok=True)
            shutil.copy2(args.binary, local_binary)
            compile_blake3_helper(
                Path(args.blake3_source), Path(args.target_dir), helper
            )
            doctor_request = dict(base)
            doctor_request["expected_lifecycle_digest"] = digest
            write_json(evidence / "doctor-request.json", doctor_request)
            write_json(evidence / "registrations.json", registrations)
            head = subprocess.check_output(["git", "-C", str(linked), "rev-parse", "HEAD"], text=True).strip()
            branch = subprocess.check_output(["git", "-C", str(linked), "branch", "--show-current"], text=True).strip()
            common = subprocess.check_output(
                ["git", "-C", str(linked), "rev-parse", "--path-format=absolute", "--git-common-dir"],
                text=True,
            ).strip()
            request = {
                "issue": 1505, "repository": "agent-logic/agent-design-language",
                "binding": {
                    "worktree": str(linked), "branch": branch, "exact_head": head,
                    "git_common_dir": common, "generation": generation,
                    "lifecycle_digest": digest,
                },
                "evidence_root": str(linked),
                "proof": {
                    "manifest_id": "issue873-retry-qualification", "lane": "tooling",
                    "deterministic": True,
                    "evidence_ref": ".csdlc/evidence/1505/csdlc-predecessor",
                    "evidence_digest": args.binary_blake3,
                    "observed_digest": args.binary_blake3, "stale": False,
                    "normalization": "doctor_issue_phase_v1",
                    "command": {
                        "generation": "v3",
                        "binary_ref": ".csdlc/evidence/1505/csdlc-predecessor",
                        "argv": [
                            "doctor", "--request", ".csdlc/evidence/1505/doctor-request.json",
                            "--registry", "docs/templates/prompts/current.json",
                            "--registrations", ".csdlc/evidence/1505/registrations.json",
                            "--repo-root", str(linked), "--v3-state-root", f"{common}/csdlc-v3/local",
                        ],
                        "request_ref": ".csdlc/evidence/1505/doctor-request.json",
                        "timeout_millis": 10000,
                        "side_effect_boundary_refs": ["tracked"],
                        "provider_side_effects": False,
                    },
                },
            }
        elif step_id == "review":
            assert head is not None
            evidence = linked / ".csdlc/evidence/1505"
            proof_path = evidence / "v3-proof/issue873-retry-qualification.json"
            proof_digest = blake3_file(helper, proof_path)
            receipt = {
                "schema": "csdlc.v3.typed_review_receipt.v1",
                "repository": "agent-logic/agent-design-language", "issue": 1505,
                "implementer": "synthetic-fixture-author",
                "reviewer": "synthetic-independent-fixture-reviewer",
                "reviewed_revision": head, "expected_head_sha": head,
                "evidence_digest": proof_digest,
            }
            receipt_path = evidence / "review.json"
            write_json(receipt_path, receipt)
            fields = [
                receipt["schema"], receipt["repository"], "1505", receipt["implementer"],
                receipt["reviewer"], head, head, proof_digest,
            ]
            payload = evidence / "review-payload.bin"
            payload.write_bytes(b"".join(value.encode() + b"\0" for value in fields))
            review_digest = blake3_file(helper, payload)
            request = {
                "repository": "agent-logic/agent-design-language", "issue": 1505,
                "pull_request": 639, "implementer": receipt["implementer"],
                "reviewer": receipt["reviewer"], "review_revision": head,
                "expected_head_sha": head, "head_sha": head, "mode": "closing",
                "title": "Installed intent fixture", "body": "Closes #1505",
                "review_present": True,
                "typed_review_receipt_path": ".csdlc/evidence/1505/review.json",
                "typed_review_receipt_digest": review_digest,
            }
        elif step_id in {"publish", "github-pr-create", "merge-internal-input-guard", "merge-remote-apply", "merge-idempotent-observation"}:
            assert head is not None and review_digest is not None
            if selector_digest is None:
                selector_payload = linked / ".csdlc/evidence/1505/selector-payload.bin"
                selector_payload.write_bytes(
                    (linked / "csdlc-v3/operator/authority-selector.json").read_bytes() + b"\0"
                )
                selector_digest = blake3_file(helper, selector_payload)
                remote_env = configure_fake_remote(linked, head)
            if step_id == "publish":
                mutation = {
                    "action": "pull_request_create", "base": "main",
                    "head": base["branch"], "title": "Installed intent fixture",
                    "body": "Closes #1505", "draft": True,
                }
                github_request: dict[str, Any] = {
                    "repository": "agent-logic/agent-design-language", "issue": 1505,
                    "expected_head_sha": head, "credential_names": ["GITHUB_TOKEN"],
                    "mutation": mutation,
                }
            elif step_id == "github-pr-create":
                github_request = {
                    "repository": "agent-logic/agent-design-language", "issue": 1505,
                    "pull_request": 639, "expected_head_sha": head,
                    "credential_names": ["GITHUB_TOKEN"],
                    "mutation": {"action": "pull_request_ready"},
                }
            else:
                mutation = {
                    "action": "pull_request_merge", "base": "main", "method": "merge",
                    "review_receipt_path": str(merge_review_receipt_path(root)),
                    "review_receipt_digest": review_digest,
                }
                github_request = {
                    "repository": "agent-logic/agent-design-language", "issue": 1505,
                    "pull_request": 639, "expected_head_sha": head,
                    "credential_names": ["GITHUB_TOKEN"], "mutation": mutation,
                }
                if step_id != "merge-internal-input-guard":
                    github_request["operator_approval"] = "synthetic operator authorizes fixture PR639 exact merge"
            request = {
                "expected_lifecycle_digest": selector_digest, "exact_review_sha": head,
                "operation": {"kind": "github_mutation", "request": github_request},
            }
        elif step_id in {"finish-preview", "finish-linked", "finish-primary"}:
            assert head is not None
            request = {
                "repository": "agent-logic/agent-design-language", "issue": 1505,
                "pull_request": 639, "expected_head_sha": head, "mode": "closing",
                "credential_names": ["GITHUB_TOKEN"],
            }
            if step_id == "finish-linked":
                request["terminal_state"] = linked_terminal_state(linked, 1505)
            elif step_id == "finish-primary":
                request["terminal_state"] = primary_terminal_state(root, 1505)
        elif step_id == "cleanup-foreign-dirty-guard":
            assert head is not None
            if cleanup_control is None:
                raise RuntimeError("cleanup control was not registered")
            receipt_path = Path(primary_terminal_state(root, 1505)["receipt_path"])
            if not receipt_path.is_file():
                raise RuntimeError("cleanup requires the exact linked terminal receipt")
            foreign_dirty = cleanup_control / "issue-873-foreign-dirty.tmp"
            if foreign_dirty.exists():
                raise RuntimeError("cleanup foreign-dirty fixture already exists")
            foreign_dirty.write_text("synthetic foreign dirty guard\n", encoding="utf-8")
            request = {
                "repository": "agent-logic/agent-design-language", "issue": 1505,
                "pull_request": 639, "expected_head_sha": head, "mode": "closing",
                "cleanup": {
                    "approved_parent": str(fixture_worktree_parent(root)),
                    "repository_root": str(root), "candidate_path": str(cleanup_control),
                    "remove": False, "terminal_receipt": True,
                    "terminal_receipt_path": str(receipt_path),
                    "terminal_receipt_digest": blake3_file(helper, receipt_path),
                },
            }
        elif step_id == "cleanup-tracked-dirty-guard":
            assert head is not None
            if cleanup_control is None:
                raise RuntimeError("cleanup control was not registered")
            foreign_dirty = cleanup_control / "issue-873-foreign-dirty.tmp"
            if not foreign_dirty.is_file():
                raise RuntimeError("foreign dirty guard artifact is missing")
            foreign_dirty.unlink()
            tracked = cleanup_control / "tracked"
            tracked_original = tracked.read_bytes()
            (logs / "tracked-before-dirty.bin").write_bytes(tracked_original)
            tracked.write_text(
                tracked.read_text(encoding="utf-8") + "synthetic tracked dirty guard\n",
                encoding="utf-8",
            )
            receipt_path = Path(primary_terminal_state(root, 1505)["receipt_path"])
            request = {
                "repository": "agent-logic/agent-design-language", "issue": 1505,
                "pull_request": 639, "expected_head_sha": head, "mode": "closing",
                "cleanup": {
                    "approved_parent": str(fixture_worktree_parent(root)),
                    "repository_root": str(root), "candidate_path": str(cleanup_control),
                    "remove": False, "terminal_receipt": True,
                    "terminal_receipt_path": str(receipt_path),
                    "terminal_receipt_digest": blake3_file(helper, receipt_path),
                },
            }
        elif step_id == "cleanup-preview":
            assert head is not None
            if cleanup_control is None:
                raise RuntimeError("cleanup control was not registered")
            if tracked_original is None:
                raise RuntimeError("tracked cleanup fixture bytes were not retained")
            tracked = cleanup_control / "tracked"
            dirty_digest = hashlib.sha256(tracked.read_bytes()).hexdigest()
            tracked.write_bytes(tracked_original)
            restored_digest = hashlib.sha256(tracked.read_bytes()).hexdigest()
            original_digest = hashlib.sha256(tracked_original).hexdigest()
            if restored_digest != original_digest:
                raise RuntimeError("tracked cleanup fixture restoration changed bytes")
            write_json(
                logs / "tracked-restore.json",
                {
                    "path": str(tracked), "dirty_sha256": dirty_digest,
                    "original_sha256": original_digest,
                    "restored_sha256": restored_digest, "byte_equal": True,
                },
            )
            receipt_path = Path(primary_terminal_state(root, 1505)["receipt_path"])
            request = {
                "repository": "agent-logic/agent-design-language", "issue": 1505,
                "pull_request": 639, "expected_head_sha": head, "mode": "closing",
                "cleanup": {
                    "approved_parent": str(fixture_worktree_parent(root)),
                    "repository_root": str(root), "candidate_path": str(cleanup_control),
                    "remove": False, "terminal_receipt": True,
                    "terminal_receipt_path": str(receipt_path),
                    "terminal_receipt_digest": blake3_file(helper, receipt_path),
                },
            }
        elif step_id == "cleanup-stale-preview-guard":
            assert head is not None
            if cleanup_control is None or preview_digest is None:
                raise RuntimeError("stale preview guard requires the exact retained preview")
            receipt_path = Path(primary_terminal_state(root, 1505)["receipt_path"])
            stale_digest = ("0" if preview_digest[0] != "0" else "1") + preview_digest[1:]
            request = {
                "repository": "agent-logic/agent-design-language", "issue": 1505,
                "pull_request": 639, "expected_head_sha": head, "mode": "closing",
                "cleanup": {
                    "approved_parent": str(fixture_worktree_parent(root)),
                    "repository_root": str(root), "candidate_path": str(cleanup_control),
                    "remove": True, "terminal_receipt": True,
                    "terminal_receipt_path": str(receipt_path),
                    "terminal_receipt_digest": blake3_file(helper, receipt_path),
                    "preview_receipt_digest": stale_digest,
                },
            }
        elif step_id in {"cleanup-refresh-preview", "cleanup-execute"}:
            assert head is not None
            if cleanup_control is None or preview_digest is None:
                raise RuntimeError("cleanup completion requires the retained clean preview")
            receipt_path = Path(primary_terminal_state(root, 1505)["receipt_path"])
            request = {
                "repository": "agent-logic/agent-design-language", "issue": 1505,
                "pull_request": 639, "expected_head_sha": head, "mode": "closing",
                "cleanup": {
                    "approved_parent": str(fixture_worktree_parent(root)),
                    "repository_root": str(root), "candidate_path": str(cleanup_control),
                    "remove": step_id == "cleanup-execute", "terminal_receipt": True,
                    "terminal_receipt_path": str(receipt_path),
                    "terminal_receipt_digest": blake3_file(helper, receipt_path),
                },
            }
            if step_id == "cleanup-execute":
                request["cleanup"]["preview_receipt_digest"] = preview_digest
        else:
            raise RuntimeError(f"terminal executor does not yet implement {step_id}")
        write_json(request_path, request)
        if step_id.startswith("merge-"):
            assert_operation_request_deserializes(step, request_path)
        topology = terminal_topology(step_id)
        result = invoke(step, request, linked if topology == "linked" else root)
        reason = result["envelope"].get("reason_code")
        expected = {
            "prepare": {"command_completed"}, "bind": {"command_completed"},
            "proof-input-admission": {"lifecycle_digest_valid"},
            "proof": {"command_completed"}, "review": {"command_completed"},
            "publish": {"command_completed"}, "github-pr-create": {"command_completed"},
            "merge-internal-input-guard": {"github_merge_ineligible"},
            "merge-remote-apply": {"command_completed"},
            "merge-idempotent-observation": {"command_completed"},
            "finish-preview": {"command_completed"}, "finish-linked": {"command_completed"},
            "finish-primary": {"command_completed"},
            "cleanup-foreign-dirty-guard": {"command_completed"},
            "cleanup-tracked-dirty-guard": {"command_completed"},
            "cleanup-preview": {"command_completed"},
            "cleanup-stale-preview-guard": {"preview_receipt_mismatch"},
            "cleanup-refresh-preview": {"command_completed"},
            "cleanup-execute": {"command_completed"},
        }[step_id]
        if reason not in expected:
            raise RuntimeError(f"{step_id} drifted: {reason}")
        if step_id == "prepare":
            registrations = [{"branch": base["branch"], "worktree": str(linked), "primary": False}]
            write_json(private / "registrations.json", registrations)
        elif step_id == "bind":
            cleanup_control = create_cleanup_control(root, linked, logs)
        elif step_id == "review":
            stage_review_receipt_for_primary(linked, root, logs)
        elif step_id in {"cleanup-preview", "cleanup-refresh-preview"}:
            cleanup = result.get("result", {}).get("cleanup", {})
            if cleanup.get("decision") != "removable" or not isinstance(cleanup.get("receipt_digest"), str):
                raise RuntimeError("cleanup preview did not retain a removable receipt")
            preview_digest = cleanup["receipt_digest"]
        if step_id in {"cleanup-foreign-dirty-guard", "cleanup-tracked-dirty-guard"}:
            if not linked.is_dir() or cleanup_control is None or not cleanup_control.is_dir():
                raise RuntimeError("cleanup dirty guard removed a registered worktree")
        if step_id == "cleanup-stale-preview-guard":
            envelope = result.get("envelope", {})
            findings = envelope.get("findings", [])
            if (
                result.get("performed_mutation") is not False
                or envelope.get("effects", {}).get("outcome") != "none"
                or findings != [{
                    "code": "preview_receipt_mismatch",
                    "message": "cleanup removal requires a preview receipt for the same Git registration",
                }]
                or not cleanup_control.is_dir()
            ):
                raise RuntimeError("stale cleanup preview guard changed the cleanup control")
        if step_id == "cleanup-execute":
            cleanup = result.get("result", {}).get("cleanup", {})
            envelope = result.get("envelope", {})
            if (
                result.get("performed_mutation") is not True
                or envelope.get("effects", {}).get("outcome") != "performed"
                or cleanup.get("decision") != "removed"
                or cleanup_control.exists()
                or not linked.is_dir()
            ):
                raise RuntimeError("cleanup execute did not remove only the exact control")

    value = {
        "schema": RAW_SCHEMA, "issue": 873,
        "provenance": {
            "source_head": args.source_revision, "source_has_uncommitted_changes": False,
            "installed_binary_blake3": args.binary_blake3,
            "source_binding": "Exact retained predecessor bytes; isolated fixture generated by the installed candidate harness",
        },
        "attempted": len(attempts), "attempts": attempts,
    }
    topology_path = logs / "cleanup-control-topology.json"
    if not topology_path.is_file():
        raise RuntimeError("terminal journey did not retain cleanup topology")
    value["cleanup_topology"] = json.loads(topology_path.read_text(encoding="utf-8"))
    return value


def configure_fake_remote(
    linked: Path,
    head: str,
    issue: int = 1505,
    branch: str = "codex/1505-installed-intent-fixture",
    transport_root: Path | None = None,
) -> dict[str, str]:
    evidence = transport_root or linked / f".csdlc/evidence/{issue}"
    fake_bin = evidence / "fake-bin"
    fake_bin.mkdir(parents=True, exist_ok=True)
    curl = fake_bin / "curl"
    merge_commit = "e" * 40
    curl.write_text(f'''#!/bin/sh
set -eu
base=$(dirname "$0")/..
method=GET
payload=
url=
while test "$#" -gt 0; do
 case "$1" in
 --request) shift; method=$1 ;;
 --data-binary) shift; payload=${{1#@}} ;;
 --config) shift; if test "$1" = -; then cat >/dev/null; fi ;;
 https://api.github.com/*) url=$1 ;;
 esac
 shift
done
printf '%s:%s\n' "$method" "$url" >> "$base/remote-requests"
case "$method:$url" in
 GET:https://api.github.com/repos/agent-logic/agent-design-language/git/matching-refs/heads/*)
  printf '[{{"ref":"refs/heads/{branch}","object":{{"sha":"{head}"}}}}]' ;;
 POST:https://api.github.com/repos/agent-logic/agent-design-language/pulls)
  data=$(cat "$payload")
  data=$(printf '%s' "$data" | sed 's#"head":"[^"]*"#"head":{{"sha":"{head}","ref":"{branch}"}}#;s#"base":"main"#"base":{{"ref":"main"}}#')
  body=$(printf '%s' "$data" | sed 's/^{{//')
  printf '{{"number":639,"id":639,"node_id":"PR_ready639","state":"open","merged":false,%s' "$body" > "$base/remote-pr.json"
  printf 'pr-create\\n' >> "$base/remote-effects"
  cat "$base/remote-pr.json" ;;
 PATCH:https://api.github.com/repos/agent-logic/agent-design-language/pulls/639)
  data=$(cat "$payload")
  body=$(printf '%s' "$data" | sed 's/^{{//')
  printf '{{"number":639,"id":639,"node_id":"PR_ready639","state":"open","merged":false,"base":{{"ref":"main"}},"head":{{"sha":"{head}","ref":"{branch}"}},"draft":false,%s' "$body" > "$base/remote-pr.next"
  mv "$base/remote-pr.next" "$base/remote-pr.json"
  printf 'pr-update\\n' >> "$base/remote-effects"
  cat "$base/remote-pr.json" ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/pulls[?]*)
  printf '['; if test -f "$base/remote-pr.json"; then cat "$base/remote-pr.json"; fi; printf ']' ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/pulls/639)
  cat "$base/remote-pr.json" ;;
 PUT:https://api.github.com/repos/agent-logic/agent-design-language/pulls/639/merge)
  if ! test -f "$base/remote-pr.json"; then exit 9; fi
  sed 's/"state":"open"/"state":"closed"/;s/"merged":false/"merged":true,"merge_commit_sha":"{merge_commit}"/' "$base/remote-pr.json" > "$base/remote-pr.next"
  mv "$base/remote-pr.next" "$base/remote-pr.json"
  printf 'pr-merge\\n' >> "$base/remote-effects"
  printf '%s' '{{"sha":"{merge_commit}","merged":true,"message":"Pull Request successfully merged"}}' ;;
 POST:https://api.github.com/graphql)
  if ! test -f "$base/remote-pr.json"; then exit 9; fi
  if grep -q 'markPullRequestReadyForReview' "$payload"; then
   sed 's/"draft":true/"draft":false/' "$base/remote-pr.json" > "$base/remote-pr.next"
   mv "$base/remote-pr.next" "$base/remote-pr.json"
   printf 'pr-ready\n' >> "$base/remote-effects"
   printf '%s' '{{"data":{{"markPullRequestReadyForReview":{{"pullRequest":{{"number":639,"headRefOid":"{head}","isDraft":false}}}}}}}}'
  elif grep -q '"merged":true' "$base/remote-pr.json"; then
   cat "$base/graphql-merged.json"
  else
   cat "$base/graphql-open.json"
  fi ;;
 GET:https://api.github.com/graphql)
  if test -f "$base/remote-pr.json" && grep -q '"merged":true' "$base/remote-pr.json"; then
   cat "$base/graphql-merged.json"
  else
   cat "$base/graphql-open.json"
  fi ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/rules/branches/main[?]per_page=100"&"page=1)
  printf '%s' '[]' ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/issues/{issue})
  if test -f "$base/remote-pr.json" && grep -q '"merged":true' "$base/remote-pr.json"; then
   printf '%s' '{{"number":{issue},"title":"Installed intent fixture","body":"Fixture issue","state":"closed","state_reason":"completed","updated_at":"2026-09-16T00:00:00Z","closed_at":"2026-09-16T00:00:00Z","labels":[],"assignees":[],"milestone":null}}'
  else
   printf '%s' '{{"number":{issue},"title":"Installed intent fixture","body":"Fixture issue","state":"open","state_reason":null,"updated_at":"2026-09-16T00:00:00Z","closed_at":null,"labels":[],"assignees":[],"milestone":null}}'
  fi ;;
 *) exit 9 ;;
esac
''', encoding="utf-8")
    def graphql_observation(merged: bool) -> dict[str, Any]:
        issue_state = "CLOSED" if merged else "OPEN"
        pull_state = "MERGED" if merged else "OPEN"
        merge = ({
            "oid": merge_commit,
            "parents": {"nodes": [
                {"oid": "1" * 40}, {"oid": head},
            ], "pageInfo": {"hasNextPage": False}},
        } if merged else None)
        pull = {
            "number": 639,
            "url": "https://github.com/agent-logic/agent-design-language/pull/639",
            "body": f"Closes #{issue}",
            "closingIssuesReferences": {
                "nodes": [{
                    "number": issue,
                    "url": f"https://github.com/agent-logic/agent-design-language/issues/{issue}",
                    "repository": {"nameWithOwner": "agent-logic/agent-design-language"},
                }],
                "pageInfo": {"hasNextPage": False},
            },
            "headRefOid": head,
            "baseRefName": "main",
            "baseRefOid": "1" * 40,
            "state": pull_state,
            "merged": merged,
            "isDraft": False,
            "mergeable": "MERGEABLE",
            "mergeStateStatus": "CLEAN",
            "reviewDecision": "APPROVED",
            "baseRef": {"branchProtectionRule": {
                "requiresStatusChecks": True,
                "requiresApprovingReviews": False,
                "requiresLinearHistory": False,
                "requiredStatusChecks": [{
                    "context": "issue-873-synthetic-check",
                    "app": {"databaseId": 873},
                }],
            }},
            "mergeCommit": merge,
            "reviewThreads": {"nodes": [], "pageInfo": {"hasNextPage": False}},
            "latestReviews": {"nodes": [{"state": "APPROVED"}], "pageInfo": {"hasNextPage": False}},
            "commits": {"nodes": [{"commit": {
                "oid": head,
                "statusCheckRollup": {"state": "SUCCESS", "contexts": {
                    "nodes": [{
                        "__typename": "CheckRun",
                        "name": "issue-873-synthetic-check",
                        "status": "COMPLETED",
                        "conclusion": "SUCCESS",
                        "isRequired": True,
                        "checkSuite": {"app": {"databaseId": 873}},
                    }],
                    "pageInfo": {"hasNextPage": False},
                }},
            }}]},
        }
        return {"data": {
            "repository": {
                "nameWithOwner": "agent-logic/agent-design-language",
                "mergeCommitAllowed": True,
                "pullRequest": pull,
            },
            "linkedRepository": {
                "nameWithOwner": "agent-logic/agent-design-language",
                "issue": {
                    "number": issue,
                    "url": f"https://github.com/agent-logic/agent-design-language/issues/{issue}",
                    "state": issue_state,
                },
            },
        }}
    write_json(evidence / "graphql-open.json", graphql_observation(False))
    write_json(evidence / "graphql-merged.json", graphql_observation(True))
    curl.chmod(0o755)
    token = evidence / "token"
    token.write_text("synthetic-token\n", encoding="utf-8")
    token.chmod(0o600)
    env = os.environ.copy()
    env["PATH"] = f"{fake_bin}:{os.environ.get('PATH', '/usr/bin:/bin')}"
    env["ADL_GITHUB_TOKEN_FILE"] = str(token)
    return env


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
    parser.add_argument(
        "--scenario", choices=["primary-linked-edit", "terminal-journey"], required=True
    )
    args = parser.parse_args()
    scenario_map = json.loads(args.scenario_map.read_text(encoding="utf-8"))
    scenario = next(value for value in scenario_map["scenarios"] if value["id"] == args.scenario)
    fixture_filter = (
        "installed_prepare_bind_edit_and_observations_use_canonical_context"
        if args.scenario == "primary-linked-edit"
        else "installed_merge_finish_and_exact_bound_cleanup_preserve_authority_and_archive_residue"
    )
    capture_fixture(
        args.harness.resolve(), args.slot.resolve(),
        fixture_filter,
        args.fixture.resolve(),
    )
    relocate_fixture(args.fixture.resolve())
    raw = (
        execute_primary(args, args.fixture.resolve(), scenario)
        if args.scenario == "primary-linked-edit"
        else execute_terminal(args, args.fixture.resolve(), scenario)
    )
    write_json(args.output, raw)


if __name__ == "__main__":
    main()
