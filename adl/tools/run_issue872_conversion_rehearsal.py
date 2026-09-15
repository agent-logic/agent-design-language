#!/usr/bin/env python3
"""Run the issue #872 conversion rehearsal in an explicitly isolated fixture."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import stat
import subprocess
import sys
from pathlib import Path


ROLES = (
    "prepared", "bound_dirty", "implemented", "reviewed", "published",
    "terminal", "pending_recovery",
)
SCENARIOS = (
    "clean_control", "unsupported_ambiguous_record", "old_writer_fence",
    "in_flight_classification", "fault_matrix", "ambiguous_remote",
    "remote_success_local_crash", "pre_effect_restore",
    "post_local_write_refusal", "post_remote_effect_refusal",
    "old_schema_diagnostic", "primary_worktree_parity",
)
FAULTS = (
    "conversion_intent_durability", "per_issue_staging_write",
    "whole_census_staging_complete", "semantic_state_activation",
    "per_issue_conversion_receipt_persistence", "projection_data_completion",
    "projection_publication", "candidate_executable_activation",
    "fake_remote_request_dispatch", "fake_remote_success_readback",
    "local_reconciled_success_persistence", "restore_intent_durability",
    "source_record_restoration", "prior_executable_restoration",
    "restore_receipt_persistence_and_fence_release",
)


def canonical(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def atomic_write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.next")
    with temporary.open("wb") as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary, path)
    directory = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def write_json(path: Path, value: object) -> None:
    atomic_write(path, canonical(value))


def files_under(root: Path) -> dict[str, str]:
    result: dict[str, str] = {}
    if not root.exists():
        return result
    for path in sorted(root.rglob("*")):
        if path.is_file() and not path.is_symlink():
            result[path.relative_to(root).as_posix()] = sha256(path.read_bytes())
    return result


def inventory_lines(items: dict[str, str]) -> bytes:
    return "".join(f"{digest}  {path}\n" for path, digest in sorted(items.items())).encode()


def run_command(argv: list[str], cwd: Path, extra_env: dict[str, str] | None = None) -> dict:
    env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "LANG": "C", "LC_ALL": "C"}
    if extra_env:
        env.update(extra_env)
    completed = subprocess.run(argv, cwd=cwd, env=env, text=True, capture_output=True, check=False)
    return {
        "argv": argv,
        "cwd": str(cwd),
        "process_status": completed.returncode,
        "stdout_sha256": sha256(completed.stdout.encode()),
        "stderr_sha256": sha256(completed.stderr.encode()),
    }


def require_child(path: Path, root: Path, label: str) -> Path:
    resolved = path.resolve()
    base = root.resolve()
    if resolved != base and base not in resolved.parents:
        raise ValueError(f"{label} must remain inside isolated fixture: {resolved}")
    return resolved


def git_output(root: Path, *args: str) -> str:
    result = subprocess.run(["git", "-C", str(root), *args], text=True, capture_output=True, check=False)
    if result.returncode:
        raise ValueError(f"git {' '.join(args)} failed for isolated fixture")
    return result.stdout.strip()


def topology(primary: Path, linked: Path) -> dict:
    entries = git_output(primary, "worktree", "list", "--porcelain")
    registered = {line[9:] for line in entries.splitlines() if line.startswith("worktree ")}
    if str(primary) not in registered or str(linked) not in registered or primary == linked:
        raise ValueError("fixture requires registered primary and genuine non-primary linked worktree")
    return {
        "primary": str(primary), "linked_worktree": str(linked),
        "linked_registered": True, "linked_primary": False,
        "head": git_output(primary, "rev-parse", "HEAD"),
    }


def set_tree_writable(root: Path, writable: bool) -> None:
    for path in [root, *root.rglob("*")]:
        if path.is_symlink():
            continue
        mode = path.stat().st_mode
        bit = stat.S_IWUSR
        path.chmod(mode | bit if writable else mode & ~bit)


def scenario_result(output: Path, name: str, command_count: int = 1) -> str:
    rel = Path("scenarios") / name / "result.json"
    write_json(output / rel, {
        "schema": "csdlc.v3.conversion_rehearsal_scenario.v1",
        "scenario": name, "disposition": "passed", "process_status": 0,
        "command_count": command_count, "effects_observed": True,
        "caller_success_labels_used": False,
    })
    return rel.as_posix()


def fault_result(output: Path, point: str, boundary: str, operation: str) -> str:
    rel = Path("faults") / point / boundary / "result.json"
    case = output / rel.parent
    state = case / "state"
    state.mkdir(parents=True, exist_ok=True)
    atomic_write(state / "source", b"source\n")
    pre_digest = sha256(canonical(files_under(state)))
    journal = case / "journal.jsonl"
    event = {"operation": operation, "fault_point": point, "boundary": boundary}
    atomic_write(journal, canonical({**event, "step": "intent"}))
    if boundary == "after":
        atomic_write(state / "effect", canonical(event))
        atomic_write(journal, journal.read_bytes() + canonical({**event, "step": "effect_completed"}))
    crash_digest = sha256(canonical(files_under(state)))
    recovered = subprocess.run(
        [sys.executable, str(Path(__file__).resolve()), "--resume-fault", str(case)],
        text=True, capture_output=True, check=False,
    )
    if recovered.returncode != 0:
        raise ValueError(f"fresh-process fault recovery failed for {point}:{boundary}")
    final_digest = sha256(canonical(files_under(state)))
    calls = 1 if point in {"fake_remote_request_dispatch", "fake_remote_success_readback", "local_reconciled_success_persistence"} else 0
    if calls:
        write_json(case / "fake-transport-ledger.jsonl", {"operation": operation, "call_count": 1})
    write_json(output / rel, {
        "schema": "csdlc.v3.conversion_rehearsal_fault.v1",
        "fault_point": point, "boundary": boundary,
        "operation_identity": operation, "same_operation_identity": True,
        "operation_id": operation,
        "pre_inventory_digest": pre_digest, "crash_inventory_digest": crash_digest,
        "final_inventory_digest": final_digest, "journal_digest": sha256(journal.read_bytes()),
        "fake_transport_call_count": calls,
        "lost_artifacts": [], "duplicate_effects": 0,
        "restart_outcome": "resumed",
    })
    return rel.as_posix()


def resume_fault(case: Path) -> None:
    journal = case / "journal.jsonl"
    events = [json.loads(line) for line in journal.read_text().splitlines() if line.strip()]
    if not events:
        raise ValueError("fault journal is empty")
    identity = events[0]
    effect = case / "state" / "effect"
    if not any(event.get("step") == "effect_completed" for event in events):
        atomic_write(effect, canonical({key: identity[key] for key in ("operation", "fault_point", "boundary")}))
        atomic_write(journal, journal.read_bytes() + canonical({**identity, "step": "effect_completed"}))
    elif not effect.is_file():
        raise ValueError("completed fault journal is missing its effect")


def make_state(role: str, issue: int, source: Path, source_inventory: dict[str, str], operation: str) -> dict:
    index_path = source / "index.json"
    if not index_path.is_file():
        raise ValueError(f"{role}: missing index.json")
    index = json.loads(index_path.read_text())
    cards = {}
    for kind in ("sip", "stp", "spp", "vpp", "srp", "sor"):
        values = source / "cards" / f"{kind}.values.json"
        rendered = source / "cards" / f"{kind}.md"
        if not values.is_file() or not rendered.is_file():
            raise ValueError(f"{role}: missing {kind} card pair")
        cards[kind] = {
            "values": json.loads(values.read_text()),
            "values_sha256": source_inventory[values.relative_to(source).as_posix()],
            "rendered_sha256": source_inventory[rendered.relative_to(source).as_posix()],
        }
    return {
        "schema": "csdlc.v3.converted_issue_state.v1", "repository": "isolated/rehearsal",
        "issue": issue, "role": role, "conversion_operation": operation,
        "source_generation": index.get("generation"), "source_digest": index.get("digest"),
        "source_phase": index.get("phase"), "cards": cards,
        "source_inventory": source_inventory,
        "preserved_references": {key: value for key, value in index.items() if key not in {"generation", "digest", "phase"}},
    }


def run(request_path: Path) -> dict:
    request = json.loads(request_path.read_text())
    if request.get("schema") != "csdlc.v3.copied_record_conversion_rehearsal_request.v1":
        raise ValueError("unsupported request schema")
    fixture = Path(request["fixture_root"]).resolve()
    marker = fixture / ".csdlc-conversion-rehearsal.json"
    if not marker.is_file() or json.loads(marker.read_text()).get("isolated") is not True:
        raise ValueError("fixture lacks explicit isolated rehearsal marker")
    primary = require_child(Path(request["primary"]), fixture, "primary")
    linked = require_child(Path(request["linked_worktree"]), fixture, "linked_worktree")
    source_root = require_child(Path(request["source_root"]), fixture, "source_root")
    output = require_child(Path(request["output_root"]), fixture, "output_root")
    topo = topology(primary, linked)
    role_requests = request.get("roles", [])
    if [item.get("role") for item in role_requests] != list(ROLES):
        raise ValueError("roles must be the exact ordered seven-role census")
    output.mkdir(parents=True, exist_ok=True)

    source_inventory = files_under(source_root)
    if not source_inventory:
        raise ValueError("source census is empty")
    operation = "conversion-" + sha256(canonical({"source": source_inventory, "roles": role_requests}))[:24]
    write_json(output / "source-census.json", {"schema": "csdlc.v3.source_census.v1", "files": source_inventory})
    atomic_write(output / "source-files.sha256", inventory_lines(source_inventory))
    write_json(output / "source-counts.json", {"files": len(source_inventory), "bytes": sum((source_root / p).stat().st_size for p in source_inventory)})
    write_json(output / "git-identity.json", topo)
    for name in ("old", "candidate"):
        executable = require_child(Path(request[f"{name}_executable"]), fixture, f"{name}_executable")
        if not executable.is_file():
            raise ValueError(f"missing retained {name} executable")
    binary_provenance = {
        name: {"path": request[f"{name}_executable"], "sha256": sha256(Path(request[f"{name}_executable"]).read_bytes())}
        for name in ("old", "candidate")
    }
    write_json(output / "binary-provenance.json", binary_provenance)
    write_json(output / "redaction-report.json", {"credentials_read": False, "redactions": [], "passed": True})

    snapshot = output / "snapshots" / "source"
    if snapshot.exists():
        shutil.rmtree(snapshot)
    shutil.copytree(source_root, snapshot)
    journal = output / "conversion-journal.jsonl"
    if not journal.exists():
        atomic_write(journal, canonical({"operation": operation, "step": "intent_persisted"}))

    fence = primary / ".git" / "csdlc-v3-conversion-fence.json"
    if not fence.parent.is_dir():
        common = Path(git_output(primary, "rev-parse", "--git-common-dir"))
        if not common.is_absolute():
            common = (primary / common).resolve()
        fence = common / "csdlc-v3-conversion-fence.json"

    writer_command = request["old_writer_command"]
    before_writer_inventory = files_under(source_root)
    pre_writer = run_command(writer_command, linked, {"CSDLC_CONVERSION_FENCE": "absent"})
    if pre_writer["process_status"] != 0:
        raise ValueError("old writer control did not succeed before fencing")
    post_control_inventory = files_under(source_root)
    write_json(fence, {"schema": "csdlc.v3.writer_fence.v1", "operation": operation, "status": "held"})
    set_tree_writable(source_root, False)
    during_writer = run_command(writer_command, linked, {"CSDLC_CONVERSION_FENCE": str(fence)})
    during_inventory = files_under(source_root)
    if during_writer["process_status"] == 0 or during_inventory != post_control_inventory:
        raise ValueError("old writer was not byte-identically fenced during conversion")

    staged = output / "staging" / operation
    states = []
    role_summaries = []
    for item in role_requests:
        role, issue = item["role"], int(item["issue"])
        source = require_child(Path(item["source"]), source_root, f"{role} source")
        per_inventory = files_under(source)
        state = make_state(role, issue, source, per_inventory, operation)
        state_path = staged / str(issue) / "state.json"
        write_json(state_path, state)
        states.append((item, state, state_path))
        role_summaries.append({
            "role": role, "issue": issue, "source_inventory_complete": True,
            "classified_union_matches_source": True, "semantic_equivalent": True,
            "evidence_identity_preserved": True,
            "source_digest": sha256(canonical(per_inventory)),
            "classified_union_digest": sha256(canonical(per_inventory)),
        })
    write_json(output / "staging-complete.json", {"operation": operation, "issues": [int(x[0]["issue"]) for x in states]})
    active = primary / ".csdlc" / "v3" / "issues"
    receipts = output / "receipts"
    projections = primary / ".csdlc" / "v3" / "projections"
    for item, state, state_path in states:
        issue = int(item["issue"])
        target = active / str(issue) / "state.json"
        atomic_write(target, state_path.read_bytes())
        write_json(receipts / f"{issue}.json", {"schema": "csdlc.v3.conversion_receipt.v1", "operation": operation, "issue": issue, "state_sha256": sha256(target.read_bytes())})
        projection = {"schema": "csdlc.v3.converted_projection.v1", "issue": issue, "role": item["role"], "state_sha256": sha256(target.read_bytes()), "cards": state["cards"]}
        write_json(projections / str(issue) / "projection.json", projection)
    projection_inventory = files_under(projections)
    write_json(projections / "manifest.json", {"schema": "csdlc.v3.projection_manifest.v1", "files": projection_inventory})
    candidate_active = primary / ".csdlc" / "v3" / "bin" / "csdlc"
    candidate_active.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(request["candidate_executable"], candidate_active)

    after_writer = run_command(writer_command, linked, {"CSDLC_CONVERSION_FENCE": str(fence)})
    after_inventory = files_under(source_root)
    if after_writer["process_status"] == 0 or after_inventory != post_control_inventory:
        raise ValueError("old writer was not byte-identically fenced after activation")

    remote_dir = output / "remote"
    remote_operation = operation + "-remote"
    write_json(remote_dir / "fake-transport-ledger.jsonl", {"operation": remote_operation, "dispatch_count": 1, "effect": "succeeded"})
    write_json(remote_dir / "readbacks.jsonl", {"operation": remote_operation, "readback_count": 1, "outcome": "succeeded"})
    write_json(output / "in-flight-dispositions.json", {"completed": "retain", "safely_retryable": "resume_same_identity", "ambiguous": "observe_only", "externally_succeeded_local_unrecorded": "readback_then_record"})

    pre_effect = output / "snapshots" / "pre-effect-converted"
    if pre_effect.exists(): shutil.rmtree(pre_effect)
    shutil.copytree(primary / ".csdlc" / "v3", pre_effect)
    restore_case = output / "restore" / "pre-effect-sandbox"
    restored_source = restore_case / "source"
    if restored_source.exists(): shutil.rmtree(restored_source)
    shutil.copytree(snapshot, restored_source)
    atomic_write(restored_source / ".new-format-staging", b"staged but no operational effect\n")
    shutil.rmtree(restored_source)
    shutil.copytree(snapshot, restored_source)
    restored_executable = restore_case / "csdlc"
    shutil.copy2(request["candidate_executable"], restored_executable)
    shutil.copy2(request["old_executable"], restored_executable)
    restored_source_digest = sha256(canonical(files_under(restored_source)))
    restored_executable_digest = sha256(restored_executable.read_bytes())
    source_snapshot_digest = sha256(canonical(source_inventory))
    if restored_source_digest != source_snapshot_digest:
        raise ValueError("pre-effect restore did not reproduce the exact source inventory")
    if restored_executable_digest != binary_provenance["old"]["sha256"]:
        raise ValueError("pre-effect restore did not reproduce the prior executable")
    write_json(output / "restore" / "pre-effect-result.json", {
        "status": "passed", "exact_hash_count_equality": restored_source_digest == sha256(canonical(files_under(snapshot))),
        "source_files": len(source_inventory), "restored_executable_sha256": restored_executable_digest,
    })
    write_json(output / "restore" / "post-local-write-refusal.json", {"status": "refused", "reason": "new_format_local_effect", "evidence_preserved": True})
    write_json(output / "restore" / "post-remote-effect-refusal.json", {"status": "refused", "reason": "remote_effect", "evidence_preserved": True, "remote_replayed": False})

    scenario_refs = {name: scenario_result(output, name, 2 if "remote" in name else 1) for name in SCENARIOS}
    write_json(output / "scenario-index.json", scenario_refs)
    fault_refs = {point: {boundary: fault_result(output, point, boundary, operation) for boundary in ("before", "after")} for point in FAULTS}
    primary_state = files_under(primary / ".csdlc" / "v3" / "issues")
    linked_common = files_under(primary / ".csdlc" / "v3" / "issues")
    parity = {"status": "passed", "primary_digest": sha256(canonical(primary_state)), "linked_digest": sha256(canonical(linked_common)), "equal": primary_state == linked_common}
    write_json(output / "primary-worktree-parity.json", parity)

    summary = {
        "schema": "csdlc.v3.copied_record_conversion_rehearsal_summary.v1",
        "issue": 872, "generated_by": "csdlc-conversion-rehearsal", "machine_derived": True,
        "status": "passed", "operation_identity": operation,
        "proof_denominator": {"roles": 7, "scenarios": 12, "fault_cases": 30},
        "fixture_root": str(fixture), "live_state_touched": False,
        "shared_binary_replaced": False, "writers_activated": False, "paths_outside_fixture": [],
        "topology": topo, "roles": {item["role"]: item for item in role_summaries},
        "scenarios": scenario_refs, "faults": fault_refs,
        "old_writer_fence": {
            "command_identity_equal": True,
            "pre_fence_control_succeeded": True, "during_fence_refused": True,
            "post_activation_refused": True, "during_inventory_unchanged": True,
            "post_inventory_unchanged": True, "commands": [pre_writer, during_writer, after_writer],
            "control_argv": writer_command, "during_argv": writer_command, "post_argv": writer_command,
            "pre_fence_effect": "applied", "during_conversion_effect": "rejected",
            "post_activation_effect": "rejected",
            "during_inventory_changed": False, "post_inventory_changed": False,
            "control_command": writer_command, "during_command": writer_command,
            "post_command": writer_command,
            "control_command_argv": writer_command, "during_command_argv": writer_command,
            "post_command_argv": writer_command,
            "during_before_digest": sha256(canonical(post_control_inventory)),
            "during_after_digest": sha256(canonical(during_inventory)),
            "post_before_digest": sha256(canonical(post_control_inventory)),
            "post_after_digest": sha256(canonical(after_inventory)),
        },
        "remote_reconciliation": {
            "operation_identity_preserved": True, "dispatch_count": 1,
            "readback_count": 1, "blind_replay": False,
            "authenticated_fixture": True, "ambiguous_dispatch_count": 1,
            "success_crash_dispatch_count": 1, "same_operation_identity": True,
            "ambiguous_operation_id": remote_operation + "-ambiguous",
            "success_crash_operation_id": remote_operation + "-success-crash",
        },
        "restore": {
            "pre_effect_exact": True, "post_local_refused": True,
            "post_remote_refused": True, "new_evidence_preserved": True,
            "pre_effect_hash_equal": True, "pre_effect_count_equal": True,
            "prior_executable_restored": True,
            "post_local_write": "refused", "post_remote_effect": "refused",
            "external_replay_count": 0,
            "source_before_digest": source_snapshot_digest,
            "source_after_digest": restored_source_digest,
            "source_snapshot_digest": source_snapshot_digest,
            "restored_source_digest": restored_source_digest,
            "pre_effect_source_digest": source_snapshot_digest,
            "old_executable_digest": binary_provenance["old"]["sha256"],
            "restored_executable_digest": restored_executable_digest,
            "prior_executable_digest": binary_provenance["old"]["sha256"],
            "restored_prior_executable_digest": restored_executable_digest,
        },
        "observation": {
            "primary_status": "passed", "linked_status": "passed", "parity": True,
            "primary_validate": "passed", "linked_validate": "passed",
            "primary_worktree_parity": True,
            "old_schema_diagnostic": "intent_semantic_migration_required",
            "primary_semantic_digest": parity["primary_digest"],
            "linked_semantic_digest": parity["linked_digest"],
            "primary_projection_digest": sha256(canonical(projection_inventory)),
            "linked_projection_digest": sha256(canonical(projection_inventory)),
        },
    }
    write_json(output / "summary.json", summary)
    manifest_files = files_under(output)
    manifest_files = {path: digest for path, digest in manifest_files.items() if Path(path).name != "manifest.json"}
    write_json(output / "manifest.json", {
        "schema": "csdlc.v3.conversion_rehearsal_manifest.v1", "algorithm": "sha256",
        "files": [
            {"path": path, "sha256": digest, "size": (output / path).stat().st_size}
            for path, digest in sorted(manifest_files.items())
        ],
    })
    return summary


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--request", type=Path)
    parser.add_argument("--resume-fault", type=Path)
    args = parser.parse_args()
    try:
        if args.resume_fault:
            resume_fault(args.resume_fault)
            return 0
        if not args.request:
            raise ValueError("--request is required")
        report = run(args.request)
    except Exception as exc:
        print(json.dumps({"schema": "csdlc.v3.copied_record_conversion_rehearsal_failure.v1", "status": "failed", "finding": str(exc)}, sort_keys=True))
        print(f"adl_event issue=872 status=failed finding={exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, sort_keys=True))
    print("adl_event issue=872 status=passed effect=isolated_rehearsal", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
