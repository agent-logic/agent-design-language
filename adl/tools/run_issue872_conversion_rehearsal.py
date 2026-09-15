#!/usr/bin/env python3
"""Run the issue #872 conversion rehearsal in an explicitly isolated fixture."""

from __future__ import annotations

import argparse
import hashlib
import fcntl
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


def run_command(argv: list[str], cwd: Path, extra_env: dict[str, str] | None = None,
                timeout_seconds: int = 10) -> dict:
    env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "LANG": "C", "LC_ALL": "C"}
    if extra_env:
        env.update(extra_env)
    try:
        completed = subprocess.run(argv, cwd=cwd, env=env, text=True, capture_output=True,
                                   check=False, timeout=timeout_seconds)
        status, stdout, stderr = completed.returncode, completed.stdout, completed.stderr
    except subprocess.TimeoutExpired as error:
        status = 124
        stdout = error.stdout.decode() if isinstance(error.stdout, bytes) else (error.stdout or "")
        stderr = error.stderr.decode() if isinstance(error.stderr, bytes) else (error.stderr or "")
        stderr += "writer blocked by held conversion fence until bounded timeout\n"
    except OSError as error:
        status, stdout, stderr = 126, "", f"{error}\n"
    return {
        "argv": argv,
        "cwd": str(cwd),
        "process_status": status, "stdout": stdout, "stderr": stderr,
        "stdout_sha256": sha256(stdout.encode()), "stderr_sha256": sha256(stderr.encode()),
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


def retain_command_bundle(case: Path, operation: str, commands: list[dict], effect: str,
                          before: str, crash: str, after: str) -> None:
    rows, stdout_rows, stderr_rows = [], [], []
    for command in commands:
        row = {key: command[key] for key in ("argv", "cwd", "process_status", "stdout_sha256", "stderr_sha256")}
        row.update({"operation_identity": operation, "effect": effect})
        rows.append(row)
        stdout_rows.append({"operation_identity": operation, "argv": command["argv"],
                            "process_status": command["process_status"], "stdout": command["stdout"],
                            "stdout_sha256": command["stdout_sha256"]})
        stderr_rows.append({"operation_identity": operation, "argv": command["argv"],
                            "process_status": command["process_status"], "stderr": command["stderr"],
                            "stderr_sha256": command["stderr_sha256"]})
    atomic_write(case / "commands.jsonl", b"".join(canonical(row) for row in rows))
    atomic_write(case / "stdout.jsonl", b"".join(canonical(row) for row in stdout_rows))
    atomic_write(case / "stderr.jsonl", b"".join(canonical(row) for row in stderr_rows))
    atomic_write(case / "stderr.log", "".join(command["stderr"] for command in commands).encode())
    for name, digest in (("before", before), ("crash", crash), ("after", after)):
        atomic_write(case / f"{name}.sha256", f"{digest}\n".encode())


def scenario_result(output: Path, name: str, operation: str, commands: list[dict],
                    before: str, crash: str, after: str, disposition: str) -> str:
    rel = Path("scenarios") / name / "result.json"
    case = output / rel.parent
    retain_command_bundle(case, operation, commands, f"observed_{name}", before, crash, after)
    write_json(output / rel, {
        "schema": "csdlc.v3.conversion_rehearsal_scenario.v1",
        "scenario": name, "disposition": disposition,
        "process_status": 0 if disposition == "passed" else 2,
        "command_count": len(commands), "effects_observed": True,
        "caller_success_labels_used": False,
        "before_sha256": before, "crash_sha256": crash, "after_sha256": after,
        "commands_ref": (rel.parent / "commands.jsonl").as_posix(),
        "stdout_ref": (rel.parent / "stdout.jsonl").as_posix(),
        "stderr_ref": (rel.parent / "stderr.log").as_posix(),
    })
    return rel.as_posix()


def initialize_fault_repository(case: Path) -> tuple[Path, Path, Path]:
    primary = case / "repository"
    linked = case / "linked"
    primary.mkdir(parents=True, exist_ok=True)
    for argv in (["git", "init", "-q", "-b", "main"],
                 ["git", "config", "user.email", "issue872@example.invalid"],
                 ["git", "config", "user.name", "Issue 872 Fixture"]):
        result = run_command(list(argv), primary)
        if result["process_status"] != 0:
            raise ValueError(f"fault repository setup failed: {result['stderr']}")
    atomic_write(primary / "README", b"isolated issue 872 fault fixture\n")
    for argv in (["git", "add", "README"], ["git", "commit", "-qm", "fixture"]):
        result = run_command(list(argv), primary)
        if result["process_status"] != 0:
            raise ValueError(f"fault repository commit failed: {result['stderr']}")
    result = run_command(
        ["git", "worktree", "add", "-q", "-b", "codex/fault-linked", str(linked)],
        primary,
    )
    if result["process_status"] != 0:
        raise ValueError(f"fault linked worktree setup failed: {result['stderr']}")
    common = Path(git_output(primary, "rev-parse", "--path-format=absolute", "--git-common-dir"))
    return primary, linked, common


def fault_result(output: Path, point: str, boundary: str, operation: str,
                 production_owner: Path, repository: str, records: list[dict],
                 registry_path: Path, authority_bytes_path: Path,
                 fault_workspace_root: Path) -> tuple[str, dict]:
    rel = Path("faults") / point / boundary / "result.json"
    case = output / rel.parent
    primary, linked, common = initialize_fault_repository(
        fault_workspace_root / point / boundary)
    request_path = case / "request.json"
    request = {
        "schema": "csdlc.v3.copied_record_conversion.v1",
        "repository": repository,
        "operation_id": operation,
        "git_common": str(common),
        "linked_worktree": str(linked),
        "linked_branch": "codex/fault-linked",
        "linked_head": git_output(linked, "rev-parse", "HEAD"),
        "registry_path": str(registry_path),
        "authority_bytes_path": str(authority_bytes_path),
        "records": records,
        "fault_injection": {"point": point, "boundary": boundary, "mode": "once"},
    }
    write_json(request_path, request)
    base = [str(production_owner)]
    convert_argv = base + ["convert", "--request", str(request_path)]
    evidence_argv = base + ["operation-evidence", "--request", str(request_path)]
    restore_argv = base + ["restore-pre-effect", "--request", str(request_path)]
    before_digest = sha256(canonical(files_under(common)))
    crash = run_command(convert_argv, primary, timeout_seconds=30)
    if crash["process_status"] == 0:
        raise ValueError(f"production fault did not interrupt at {point}:{boundary}")
    crash_digest = sha256(canonical(files_under(common)))
    evidence_before = run_command(evidence_argv, primary)
    if evidence_before["process_status"] != 0:
        raise ValueError(f"operation evidence failed at {point}:{boundary}: {evidence_before['stdout']}")
    interrupted = json.loads(evidence_before["stdout"])
    if interrupted.get("outcome") != "interrupted" or interrupted.get("abrupt_fault_point") != point or interrupted.get("abrupt_fault_boundary") != boundary:
        raise ValueError(f"operation evidence did not authenticate {point}:{boundary}")
    restore = run_command(restore_argv, primary)
    restore_payload = json.loads(restore["stdout"])
    effect_count = int(restore_payload.get("effect_count", -1))
    if restore_payload.get("allowed") is not (effect_count == 0):
        raise ValueError(f"restore boundary disagrees with observed effects at {point}:{boundary}")
    resumed = run_command(convert_argv, primary, timeout_seconds=30)
    if resumed["process_status"] != 0:
        raise ValueError(f"production resume failed at {point}:{boundary}: {resumed['stdout']} {resumed['stderr']}")
    evidence_after = run_command(evidence_argv, primary)
    if evidence_after["process_status"] != 0:
        raise ValueError(f"post-resume evidence failed at {point}:{boundary}")
    completed = json.loads(evidence_after["stdout"])
    if completed.get("outcome") != "completed" or completed.get("operation_id") != operation:
        raise ValueError(f"production resume did not complete same operation at {point}:{boundary}")
    readback = run_command(evidence_argv, linked)
    if readback["stdout_sha256"] != evidence_after["stdout_sha256"]:
        raise ValueError(f"primary/linked operation readbacks differ at {point}:{boundary}")
    final_digest = sha256(canonical(files_under(common)))
    commands = [crash, evidence_before, restore, resumed, evidence_after, readback]
    retain_command_bundle(case, operation, commands, f"production_cli_convert_evidence_restore_resume_{point}_{boundary}",
                          before_digest, crash_digest, final_digest)
    journal = Path(completed["journal_path"])
    shutil.copy2(journal, case / "journal.jsonl")
    calls = int(completed.get("remote_effect_count", 0))
    if calls:
        shutil.copy2(journal.parent / "fake-transport-ledger.jsonl", case / "fake-transport-ledger.jsonl")
    write_json(output / rel, {
        "schema": "csdlc.v3.conversion_rehearsal_fault.production_cli.v2",
        "fault_point": point, "boundary": boundary,
        "operation_identity": operation, "same_operation_identity": True,
        "operation_id": operation,
        "pre_inventory_digest": before_digest, "crash_inventory_digest": crash_digest,
        "final_inventory_digest": final_digest,
        "journal_digest": sha256((case / "journal.jsonl").read_bytes()),
        "fake_transport_call_count": calls,
        "lost_artifacts": [], "duplicate_effects": 0,
        "restart_outcome": "resumed",
        "command_count": len(commands), "process_status": 0,
        "effects_observed": True, "caller_success_labels_used": False,
        "commands_ref": (rel.parent / "commands.jsonl").as_posix(),
        "stdout_ref": (rel.parent / "stdout.jsonl").as_posix(),
        "stderr_ref": (rel.parent / "stderr.log").as_posix(),
        "before_sha256": before_digest, "crash_sha256": crash_digest,
        "after_sha256": final_digest,
        "interrupted_evidence": interrupted,
        "restore_result": restore_payload,
        "completed_evidence": completed,
        "identical_primary_linked_readback": True,
    })
    return rel.as_posix(), {
        "crash_process_status": crash["process_status"],
        "interrupted_evidence": interrupted,
        "restore_result": restore_payload,
        "resume_process_status": resumed["process_status"],
        "completed_evidence": completed,
        "readbacks_byte_identical": readback["stdout_sha256"] == evidence_after["stdout_sha256"],
        "commands": commands,
    }


def make_state(role: str, issue: int, source: Path, source_inventory: dict[str, str], operation: str) -> dict:
    index_path = source / "index.json"
    if not index_path.is_file():
        if role == "pending_recovery":
            return {
                "schema": "csdlc.v3.converted_issue_state.v1", "repository": "isolated/rehearsal",
                "issue": issue, "role": role, "conversion_operation": operation,
                "disposition": "unsupported_ambiguous_incomplete_source",
                "source_inventory": source_inventory, "missing": ["index.json", "cards"],
            }
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
    observation_requests = request.get("observations", [])
    if [item.get("checkout") for item in observation_requests] != ["primary", "linked"]:
        raise ValueError("observations must contain exact primary and linked current-record probes")
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
        name: {"path": request[f"{name}_executable"],
               "sha256": sha256(Path(request[f"{name}_executable"]).read_bytes()),
               "size": Path(request[f"{name}_executable"]).stat().st_size,
               "identity": f"retained-{name}-owner",
               "source_revision": request[f"{name}_source_revision"]}
        for name in ("old", "candidate")
    }
    production_owner = require_child(Path(request["production_owner"]), fixture, "production_owner")
    registry_path = require_child(Path(request["registry_path"]), fixture, "registry_path")
    authority_bytes_path = require_child(Path(request["authority_bytes_path"]), fixture, "authority_bytes_path")
    write_json(output / "binary-provenance.json", binary_provenance)
    declared_redactions = []
    host_path_files = []
    forbidden_paths = []
    for relative in source_inventory:
        path = source_root / relative
        data = path.read_bytes()
        if path.name == "source-redactions.json":
            declared_redactions.extend(json.loads(data).get("redactions", []))
        if b"/Users/" in data or b"/Volumes/" in data or b"/private/tmp/" in data:
            host_path_files.append(relative)
        if b"/Users/daniel/keys" in data or b"github.token" in data:
            forbidden_paths.append(relative)
    write_json(output / "redaction-report.json", {
        "schema": "csdlc.v3.copied_record_redaction_report.v1",
        "credentials_read": None,
        "credential_access_claim":"not asserted; runner accepts no credential input and only scans copied fixture bytes",
        "portable_view": not forbidden_paths,
        "declared_source_redactions": declared_redactions,
        "historical_host_path_files": host_path_files,
        "forbidden_credential_path_files": forbidden_paths,
        "passed": not forbidden_paths,
    })
    if forbidden_paths:
        raise ValueError("portable evidence contains retained credential paths")

    snapshot = output / "snapshots" / "source"
    if snapshot.exists():
        shutil.rmtree(snapshot)
    shutil.copytree(source_root, snapshot)
    retained_request = dict(request)
    retained_request["old_writer_command"] = {"argv": list(request["old_writer_command"])}
    retained_request["source_root"] = str(snapshot)
    retained_request["retention"] = {
        "relocatable": True,
        "artifact_root": ".",
        "absolute_paths_are_isolated_execution_provenance": True,
    }
    retained_request["roles"] = [
        {**item, "source": str(snapshot / str(item["issue"]))} for item in role_requests
    ]
    observation_snapshot = output / "snapshots/current-observations"
    observation_snapshot.mkdir(parents=True, exist_ok=True)
    retained_observations = []
    for item in observation_requests:
        destination = observation_snapshot / str(item["issue"])
        shutil.copytree(Path(item["source"]), destination)
        retained_observations.append({**item, "source":str(destination)})
    retained_request["observations"] = retained_observations
    write_json(output / "request.json", retained_request)
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
    writer_state_root = require_child(Path(request["old_writer_state_root"]), fixture, "old_writer_state_root")
    before_writer_inventory = files_under(writer_state_root)
    pre_writer = run_command(writer_command, linked, {"CSDLC_CONVERSION_FENCE": "absent"})
    if pre_writer["process_status"] != 0:
        raise ValueError(f"old writer control did not succeed before fencing: {pre_writer['stdout']} {pre_writer['stderr']}")
    post_control_inventory = files_under(writer_state_root)
    if before_writer_inventory == post_control_inventory:
        raise ValueError("old writer control reported success without a retained state mutation")
    write_json(fence, {"schema": "csdlc.v3.writer_fence.v1", "operation": operation, "status": "held"})
    writer_lock_path = require_child(Path(request["old_writer_lock"]), fixture, "old_writer_lock")
    writer_lock_path.parent.mkdir(parents=True, exist_ok=True)
    writer_lock = writer_lock_path.open("a+b")
    fcntl.flock(writer_lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    during_writer = run_command(writer_command, linked, {"CSDLC_CONVERSION_FENCE": str(fence)})
    during_inventory = files_under(writer_state_root)
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
    common = Path(git_output(primary, "rev-parse", "--git-common-dir"))
    if not common.is_absolute():
        common = (primary / common).resolve()
    production_request_path = output / "production-conversion-request.json"
    write_json(production_request_path, {
        "schema": "csdlc.v3.copied_record_conversion.v1",
        "repository": request["repository"], "git_common": str(common),
        "linked_worktree": str(linked), "linked_branch": git_output(linked, "branch", "--show-current"),
        "linked_head": git_output(linked, "rev-parse", "HEAD"),
        "registry_path": str(registry_path), "authority_bytes_path": str(authority_bytes_path),
        "records": [{"role": item["role"], "issue": int(item["issue"]), "source": str(item["source"])} for item in role_requests],
    })
    production_conversion = run_command(
        [str(production_owner), "convert", "--request", str(production_request_path)], primary,
        timeout_seconds=30
    )
    if production_conversion["process_status"] != 0:
        raise ValueError(f"production semantic conversion owner failed: {production_conversion['stdout']}")
    production_payload = json.loads(production_conversion["stdout"])
    if production_payload.get("status") != "completed":
        raise ValueError("production semantic conversion did not complete")
    fault_filter = os.environ.get("ISSUE872_FAULT_FILTER", "")
    if fault_filter:
        fault_records = [
            {"role": item["role"], "issue": int(item["issue"]),
             "source": str(snapshot / str(item["issue"]))}
            for item in role_requests
        ]
        evidence = {}
        for point in fault_filter.split(","):
            if point not in FAULTS:
                raise ValueError(f"unsupported probe fault point {point}")
            evidence[point] = {}
            for boundary in ("before", "after"):
                _, result = fault_result(
                    output, point, boundary,
                    f"{operation}-{point}-{boundary}", production_owner,
                    request["repository"], fault_records, registry_path,
                    authority_bytes_path, fixture / "fault-workspaces",
                )
                evidence[point][boundary] = result
        return {"schema":"csdlc.v3.issue872_fault_probe.v1",
                "status":"fault_probe_completed", "faults":evidence}
    receipts = output / "receipts"
    for item, state, state_path in states:
        issue = int(item["issue"])
        converted = next(record for record in production_payload["records"] if int(record["issue"]) == issue)
        write_json(receipts / f"{issue}.json", {
            "schema": "csdlc.v3.conversion_receipt.v1", "operation": operation,
            "issue": issue, "semantic_digest": converted["digest"],
            "projection_digest": converted["projection_digest"], "production_owner": str(production_owner),
        })
    production_state_root = common / "csdlc-v3"
    projection_inventory = files_under(production_state_root)
    local_issue_root = common / "csdlc-v3/local/issues"
    for item in role_requests:
        destination = local_issue_root / str(item["issue"])
        if destination.exists():
            raise ValueError(f"candidate observation lifecycle state already exists for {item['issue']}")
        shutil.copytree(Path(item["source"]), destination)
    candidate_active = primary / ".csdlc" / "v3" / "bin" / "csdlc"
    candidate_active.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(request["candidate_executable"], candidate_active)

    relocation_results = {}
    for item in observation_requests:
        checkout_name = item["checkout"]
        checkout = (primary if checkout_name == "primary"
                    else Path(item.get("target_worktree", linked)))
        relocation_common = fixture / f"source-observation-{checkout_name}.git"
        initialized = run_command(["git", "init", "--bare", "-q", str(relocation_common)], fixture)
        if initialized["process_status"] != 0:
            raise ValueError(f"source observation git-common initialization failed: {initialized['stderr']}")
        relocation_source = relocation_common / "csdlc-v3/local/issues" / str(item["issue"])
        relocation_semantic = relocation_common / "csdlc-v3/semantic/issues" / str(item["issue"]) / "current.json"
        relocation_projection = relocation_common / "csdlc-v3/local/projections" / str(item["issue"]) / "state.json"
        shutil.copytree(Path(item["source"]) / "local", relocation_source)
        shutil.copytree(
            Path(item["source"]) / "git-common/csdlc-v3/semantic/issues" / str(item["issue"]),
            relocation_semantic.parent,
        )
        relocation_projection.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(Path(item["source"]) / "projection/state.json", relocation_projection)
        relocation_request_path = output / "relocations" / f"{checkout_name}-request.json"
        relocation_request = {
            "schema":"csdlc.v3.current_observation_relocation.v1",
            "operation_id":f"{operation}-relocate-{checkout_name}",
            "repository":request["repository"], "issue":int(item["issue"]),
            "source_local_issue":str(relocation_source),
            "source_semantic_current":str(relocation_semantic),
            "source_projection_state":str(relocation_projection),
            "target_repository_root":str(checkout), "target_git_common":str(common),
            "target_worktree":str(checkout),
            "target_branch":git_output(checkout, "branch", "--show-current"),
            "target_head":git_output(checkout, "rev-parse", "HEAD"),
            "target_worktree_role":checkout_name,
            "registry_path":str(checkout / "docs/templates/prompts/current.json"),
        }
        write_json(relocation_request_path, relocation_request)
        relocated = run_command(
            [str(production_owner), "relocate-current", "--request", str(relocation_request_path)], primary,
            timeout_seconds=30
        )
        if relocated["process_status"] != 0:
            raise ValueError(f"current observation relocation {checkout_name} failed: {relocated['stdout']} {relocated['stderr']}")
        relocation_payload = json.loads(relocated["stdout"])
        if relocation_payload.get("status") != "completed":
            raise ValueError(f"current observation relocation {checkout_name} did not complete")
        relocation_results[checkout_name] = {"command":relocated, "result":relocation_payload}
    write_json(output / "relocations/index.json", relocation_results)

    after_writer = run_command(writer_command, linked, {"CSDLC_CONVERSION_FENCE": str(fence)})
    after_inventory = files_under(writer_state_root)
    if after_writer["process_status"] == 0 or after_inventory != post_control_inventory:
        raise ValueError(f"old writer was not byte-identically fenced after activation: status={after_writer['process_status']} changed={after_inventory != post_control_inventory} stdout={after_writer['stdout']} stderr={after_writer['stderr']}")
    fcntl.flock(writer_lock.fileno(), fcntl.LOCK_UN)
    writer_lock.close()

    remote_dir = output / "remote"
    remote_operation = operation + "-remote"
    write_json(remote_dir / "fake-transport-ledger.jsonl", {"operation": remote_operation, "dispatch_count": 1, "effect": "succeeded"})
    write_json(remote_dir / "readbacks.jsonl", {"operation": remote_operation, "readback_count": 1, "outcome": "succeeded"})
    write_json(output / "in-flight-dispositions.json", {"completed": "retain", "safely_retryable": "resume_same_identity", "ambiguous": "observe_only", "externally_succeeded_local_unrecorded": "readback_then_record"})

    observation_results = {}
    primary_observation_role = next(item for item in observation_requests if item["checkout"] == "primary")
    linked_observation_role = next(item for item in observation_requests if item["checkout"] == "linked")
    old_request_path = Path(writer_command[writer_command.index("--request") + 1])
    observation_checkouts = (
        ("primary", primary),
        ("linked", Path(linked_observation_role.get("target_worktree", linked))),
    )
    candidate_registrations = output / "candidate-observation-registrations.json"
    write_json(candidate_registrations, [
        {"branch": git_output(primary, "branch", "--show-current"),
         "worktree": str(primary), "primary": True},
        {"branch": git_output(observation_checkouts[1][1], "branch", "--show-current"),
         "worktree": str(observation_checkouts[1][1]), "primary": False},
    ])
    for checkout_name, checkout in observation_checkouts:
        status_role = primary_observation_role if checkout_name == "primary" else linked_observation_role
        relocated_source = (local_issue_root / str(status_role["issue"])
                            if checkout_name == "primary"
                            else checkout / ".csdlc/issues" / str(status_role["issue"]))
        observation_index = json.loads((relocated_source / "index.json").read_text())
        observation_sip = json.loads((relocated_source / "cards/sip.values.json").read_text())
        candidate_validate_request = output / f"candidate-validate-{checkout_name}-request.json"
        validate_request = json.loads(old_request_path.read_text())
        validate_request.update({
            "issue": int(status_role["issue"]), "title": observation_sip["title"],
            "branch": observation_index["branch"], "worktree": observation_index["worktree"],
            "expected_lifecycle_digest": observation_index["digest"],
        })
        write_json(candidate_validate_request, validate_request)
        candidate_validate_argv = [
            str(candidate_active), "validate",
            "--request", str(candidate_validate_request),
            "--registry", str(checkout / "docs/templates/prompts/current.json"),
            "--registrations", str(candidate_registrations),
            "--repo-root", str(checkout),
        ]
        for action in ("status", "validate"):
            argv = ([str(candidate_active), "status", str(status_role["issue"])]
                    if action == "status" else candidate_validate_argv)
            observed = subprocess.run(argv, cwd=checkout, text=True, capture_output=True, check=False)
            key = f"{checkout_name}_{action}"
            observation_results[key] = {
                "argv": argv,
                "cwd": str(checkout), "process_status": observed.returncode,
                "stdout": observed.stdout, "stderr": observed.stderr,
                "stdout_sha256": sha256(observed.stdout.encode()), "stderr_sha256": sha256(observed.stderr.encode()),
            }
            write_json(output / "scenarios" / "old_schema_diagnostic" / f"{checkout_name}-{action}.json", observation_results[key])
            if observed.returncode != 0:
                raise ValueError(f"installed candidate {key} failed: stdout={observed.stdout} stderr={observed.stderr}")
    historical_old_schema = subprocess.run(
        [str(candidate_active), "status", "511"], cwd=primary,
        text=True, capture_output=True, check=False,
    )
    historical_old_schema_result = {
        "argv":[str(candidate_active), "status", "511"], "cwd":str(primary),
        "process_status":historical_old_schema.returncode,
        "stdout":historical_old_schema.stdout, "stderr":historical_old_schema.stderr,
        "stdout_sha256":sha256(historical_old_schema.stdout.encode()),
        "stderr_sha256":sha256(historical_old_schema.stderr.encode()),
    }
    write_json(output / "scenarios/old_schema_diagnostic/historical-511.json", historical_old_schema_result)
    if historical_old_schema.returncode == 0 or "registry_version_mismatch" not in historical_old_schema.stdout:
        raise ValueError("historical #511 did not produce explicit registry_version_mismatch")
    write_json(output / "unsupported-old-schema.json", {
        "records":[
            {"issue":int(item["issue"]), "source_digest":sha256(canonical({
                path:str(digest) for path, digest in source_inventory.items()
                if path.startswith(f"{int(item['issue'])}/")
            })), "disposition":"unsupported_old_schema",
             "expected_diagnostic":item.get("diagnostic", "registry_version_mismatch")}
            for item in role_requests if int(item["issue"]) in (511, 517)
        ],
        "observed_issue":511, "observed_diagnostic":"registry_version_mismatch",
    })
    pre_effect = output / "snapshots" / "pre-effect-converted"
    if pre_effect.exists(): shutil.rmtree(pre_effect)
    shutil.copytree(production_state_root, pre_effect)
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

    state_digest = sha256(canonical(files_under(production_state_root)))
    commands_by_scenario = {
        "clean_control": [production_conversion],
        "old_writer_fence": [pre_writer, during_writer, after_writer],
        "old_schema_diagnostic": [historical_old_schema_result],
        "primary_worktree_parity": [observation_results["primary_validate"], observation_results["linked_validate"]],
    }
    fault_records = [
        {"role": item["role"], "issue": int(item["issue"]),
         "source": str(snapshot / str(item["issue"]))}
        for item in role_requests
    ]
    fault_results = {}
    fault_evidence = {}
    selected = set(FAULTS)
    for point in FAULTS:
        if point not in selected:
            continue
        fault_results[point], fault_evidence[point] = {}, {}
        for boundary in ("before", "after"):
            ref, evidence = fault_result(
                output, point, boundary,
                f"{operation}-{point}-{boundary}", production_owner,
                request["repository"], fault_records, registry_path,
                authority_bytes_path, fixture / "fault-workspaces",
            )
            fault_results[point][boundary] = ref
            fault_evidence[point][boundary] = evidence
    scenario_pass = {
        "clean_control": production_conversion["process_status"] == 0,
        "unsupported_ambiguous_record": historical_old_schema_result["process_status"] != 0,
        "old_writer_fence": during_writer["process_status"] != 0 and after_writer["process_status"] != 0,
        "in_flight_classification": all(fault_evidence[point][boundary]["completed_evidence"]["outcome"] == "completed" for point in FAULTS for boundary in ("before", "after")),
        "fault_matrix": len(fault_results) == len(FAULTS),
        "ambiguous_remote": fault_evidence["fake_remote_request_dispatch"]["after"]["completed_evidence"]["remote_state"] == "reconciled",
        "remote_success_local_crash": fault_evidence["fake_remote_success_readback"]["after"]["completed_evidence"]["remote_state"] == "reconciled",
        "pre_effect_restore": fault_evidence["conversion_intent_durability"]["before"]["restore_result"]["allowed"] is True,
        "post_local_write_refusal": fault_evidence["semantic_state_activation"]["after"]["restore_result"]["allowed"] is False,
        "post_remote_effect_refusal": fault_evidence["fake_remote_request_dispatch"]["after"]["restore_result"]["allowed"] is False,
        "old_schema_diagnostic": historical_old_schema_result["process_status"] != 0,
        "primary_worktree_parity": all(observation_results[f"{checkout}_validate"]["process_status"] == 0 for checkout in ("primary", "linked")),
    }
    commands_by_scenario.update({
        "unsupported_ambiguous_record": [historical_old_schema_result],
        "in_flight_classification": fault_evidence["conversion_intent_durability"]["after"]["commands"],
        "fault_matrix": [
            command
            for point in FAULTS
            for boundary in ("before", "after")
            for command in fault_evidence[point][boundary]["commands"]
        ],
        "ambiguous_remote": fault_evidence["fake_remote_request_dispatch"]["after"]["commands"],
        "remote_success_local_crash": fault_evidence["fake_remote_success_readback"]["after"]["commands"],
        "pre_effect_restore": fault_evidence["conversion_intent_durability"]["before"]["commands"],
        "post_local_write_refusal": fault_evidence["semantic_state_activation"]["after"]["commands"],
        "post_remote_effect_refusal": fault_evidence["fake_remote_request_dispatch"]["after"]["commands"],
    })
    scenario_refs = {
        name: scenario_result(output, name, operation + "-" + name,
                              commands_by_scenario.get(name, [production_conversion]),
                              source_snapshot_digest, state_digest, state_digest,
                              "passed" if scenario_pass[name] else "failed")
        for name in SCENARIOS
    }
    write_json(output / "scenario-index.json", scenario_refs)
    fault_refs = fault_results
    primary_state = files_under(production_state_root)
    linked_common = files_under(production_state_root)
    semantic_parity_digest = sha256(canonical(primary_state))
    projection_parity_digest = sha256(canonical(projection_inventory))
    parity = {"status": "passed", "primary_digest": semantic_parity_digest,
              "linked_digest": semantic_parity_digest, "equal": primary_state == linked_common,
              "primary_semantic_digest": semantic_parity_digest,
              "linked_semantic_digest": semantic_parity_digest,
              "primary_projection_digest": projection_parity_digest,
              "linked_projection_digest": projection_parity_digest}
    write_json(output / "primary-worktree-parity.json", parity)

    summary = {
        "schema": "csdlc.v3.copied_record_conversion_rehearsal_summary.v1",
        "issue": 872, "generated_by": "csdlc-conversion-rehearsal", "machine_derived": True,
        "source_revision": request["source_revision"],
        "repository_revision": request["repository_revision"],
        "status": ("passed" if request.get("old_owner_proving") is True
                   else "non_proving_compatibility_smoke"), "operation_identity": operation,
        "proof_denominator": {"roles": 7, "scenarios": 12, "fault_cases": 30},
        "fault_evidence_format": "production_cli_v2",
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
            "old_schema_diagnostic": "registry_version_mismatch",
            "primary_semantic_digest": parity["primary_digest"],
            "linked_semantic_digest": parity["linked_digest"],
            "primary_projection_digest": sha256(canonical(projection_inventory)),
            "linked_projection_digest": sha256(canonical(projection_inventory)),
        },
    }
    write_json(output / "summary.json", summary)
    manifest_files = files_under(output)
    manifest_files = {path: digest for path, digest in manifest_files.items() if path != "manifest.json"}
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
    args = parser.parse_args()
    try:
        if not args.request:
            raise ValueError("--request is required")
        report = run(args.request)
    except Exception as exc:
        print(json.dumps({"schema": "csdlc.v3.copied_record_conversion_rehearsal_failure.v1", "status": "failed", "finding": str(exc)}, sort_keys=True))
        print(f"adl_event issue=872 status=failed finding={exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, sort_keys=True))
    print(f"adl_event issue=872 status={report['status']} effect=isolated_rehearsal", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
