//! PVF contract-fast: deterministic offline schema/reference/distribution proof;
//! small local CPU/storage, no providers/services. Required for GHB WP02 source,
//! separate from actual isolated installed Runtime consumer acceptance.
use adl_public_contracts::{archive, ghb_contract as c, ghb_distribution as d, root};
use serde_json::{json, Value};
use std::{fs, process::Command};
fn input(name: &str) -> (Value, Value, Value) {
    let s = c::parse(&fs::read(root().join("adl-spec/schemas/ghb/v1/packet.schema.json")).unwrap())
        .unwrap();
    let p = c::parse(
        &fs::read(root().join(format!("adl-spec/examples/ghb/v1/{name}.proposed.json"))).unwrap(),
    )
    .unwrap();
    let mut context = serde_json::Map::new();
    for k in [
        "resident_identity_ref",
        "body_ref",
        "runtime_incarnation",
        "objective_ref",
        "policy_ref",
        "memory_head_ref",
        "continuity_head_ref",
        "archive_head_ref",
    ] {
        context.insert(k.into(), p["cycle"][k].clone());
    }
    (s, p, context.into())
}
#[test]
fn examples_roundtrip_and_current_context() {
    for name in ["task", "improvement"] {
        let (s, p, e) = input(name);
        let r = c::validate(&s, &p, &e).unwrap();
        assert_eq!(r["execution_performed"], false);
        let bytes = c::canonical(&p).unwrap();
        assert_eq!(c::parse(&bytes).unwrap(), p);
        assert_eq!(c::digest(&p).unwrap(), archive::sha(&bytes));
        let mut stale = e.clone();
        stale["body_ref"]["sha256"] = json!("0".repeat(64));
        assert!(c::validate(&s, &p, &stale).is_err());
        let mut stale = e.clone();
        stale["runtime_incarnation"] = json!("another-incarnation");
        assert!(c::validate(&s, &p, &stale).is_err());
    }
}
#[test]
fn malformed_json_and_schema_refusal() {
    for raw in [r#"{"x":1,"x":2}"#, r#"{"x":1.0}"#, r#"{"é":1}"#] {
        assert!(c::parse(raw.as_bytes()).is_err());
    }
    let (s, p, e) = input("task");
    for field in ["schema", "contract_version", "profile_id"] {
        let mut q = p.clone();
        q[field] = Value::Null;
        assert!(c::validate(&s, &q, &e).is_err());
    }
    let mut q = p.clone();
    q["unexpected"] = json!(true);
    assert!(c::validate(&s, &q, &e).is_err());
    let mut remote = s.clone();
    remote["definitions"]["forbidden"] = json!({"$ref":"https://example.invalid/no-fetch"});
    assert!(c::validate(&remote, &p, &e).is_err());
}
#[test]
fn graph_digest_and_budget_refusals() {
    let (s, p, e) = input("improvement");
    let mut q = p.clone();
    q["artifacts"][0]["owner"] = json!("changed/owner");
    assert!(c::validate(&s, &q, &e).is_err());
    let mut q = p.clone();
    q["artifacts"].as_array_mut().unwrap().remove(0);
    assert!(c::validate(&s, &q, &e).is_err());
    let mut q = p.clone();
    let a = q["artifacts"][0].clone();
    q["artifacts"].as_array_mut().unwrap().push(a);
    assert!(c::validate(&s, &q, &e).is_err());
    let mut q = p.clone();
    q["cycle"]["budget"]["consumed"]["calls"] = json!(999);
    assert!(c::validate(&s, &q, &e).is_err());
    let mut q = p.clone();
    q["artifacts"][0]["payload"]["posterior"] = json!(99);
    assert!(c::validate(&s, &q, &e).is_err());
}
fn git(path: &std::path::Path, args: &[&str]) {
    assert!(Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .unwrap()
        .status
        .success());
}
#[test]
fn export_exact_committed_data_and_install_refusals() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = fs::canonicalize(tmp.path()).unwrap().join("producer");
    fs::create_dir(&repo).unwrap();
    for rel in [
        "LICENSE",
        "adl-spec/schemas/ghb/v1/packet.schema.json",
        "adl-spec/examples/ghb/v1/task.proposed.json",
    ] {
        let dest = repo.join(rel);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(root().join(rel), dest).unwrap();
    }
    git(&repo, &["init", "-q"]);
    git(&repo, &["add", "."]);
    git(
        &repo,
        &[
            "-c",
            "user.name=ContractTest",
            "-c",
            "user.email=contract@example.invalid",
            "commit",
            "-qm",
            "canonical input",
        ],
    );
    let a = tmp.path().join("a.tar.gz");
    let b = tmp.path().join("b.tar.gz");
    let r = d::export(&repo, &a).unwrap();
    d::export(&repo, &b).unwrap();
    assert_eq!(fs::read(&a).unwrap(), fs::read(&b).unwrap());
    let pin = r["manifest_sha256"].as_str().unwrap();
    let installed = fs::canonicalize(tmp.path()).unwrap().join("installed");
    d::verify(&a, pin, Some(&installed)).unwrap();
    d::verify_root(&installed, pin).unwrap();
    assert!(d::verify(&a, &"0".repeat(64), None).is_err());
    assert!(d::verify(&a, pin, Some(&installed)).is_err());
    #[cfg(unix)]
    {
        let alias = installed.join("adl-spec\\schemas\\ghb\\v1\\packet.schema.json");
        fs::copy(
            installed.join("adl-spec/schemas/ghb/v1/packet.schema.json"),
            &alias,
        )
        .unwrap();
        assert!(d::verify_root(&installed, pin).is_err());
        fs::remove_file(alias).unwrap();
        let target = installed.join("adl-spec/examples/ghb/v1/task.proposed.json");
        let backup = fs::read(&target).unwrap();
        fs::remove_file(&target).unwrap();
        std::os::unix::fs::symlink(installed.join("LICENSE"), &target).unwrap();
        assert!(d::verify_root(&installed, pin).is_err());
        fs::remove_file(&target).unwrap();
        fs::write(&target, &backup).unwrap();
        let link = tmp.path().join("outside-hardlink");
        fs::hard_link(&target, &link).unwrap();
        assert!(d::verify_root(&installed, pin).is_err());
        fs::remove_file(link).unwrap();
    }
    // A staged deletion must not omit committed canonical inputs.
    git(
        &repo,
        &[
            "rm",
            "--cached",
            "adl-spec/examples/ghb/v1/task.proposed.json",
        ],
    );
    let staged = d::export(&repo, &tmp.path().join("staged.tar.gz")).unwrap();
    assert_eq!(staged["manifest_sha256"], r["manifest_sha256"]);
    fs::write(installed.join("unexpected.json"), b"{}").unwrap();
    assert!(d::verify_root(&installed, pin).is_err());
    fs::remove_file(installed.join("unexpected.json")).unwrap();
    fs::write(
        installed.join("adl-spec/examples/ghb/v1/task.proposed.json"),
        b"{}",
    )
    .unwrap();
    assert!(d::verify_root(&installed, pin).is_err());
    fs::write(
        repo.join("adl-spec/examples/ghb/v1/task.proposed.json"),
        b"{}",
    )
    .unwrap();
    assert!(d::export(&repo, &tmp.path().join("dirty.tar.gz")).is_err());
}
#[cfg(unix)]
#[test]
fn aliased_installed_root_refused() {
    let tmp = tempfile::tempdir().unwrap();
    let real = tmp.path().join("real");
    fs::create_dir(&real).unwrap();
    let alias = tmp.path().join("alias");
    std::os::unix::fs::symlink(real, &alias).unwrap();
    assert!(d::verify_root(&alias, &"0".repeat(64)).is_err());
}
fn observation(p: &Value) -> Value {
    json!({"transition":"observe","source_state":"initial","target_state":"observed","inputs":[p["cycle"]["body_ref"]],"outputs":[],"owner":"example.invalid/observer","authority_ref":null,"effects":["artifact_retention"],"remaining_budget":p["cycle"]["budget"]["ceilings"],"stop_reason":null,"correlation_ids":{"cycle_id":p["cycle"]["cycle_id"],"candidate_id":null,"trial_id":null,"invocation_ids":[],"evaluator_id":null,"lineage_id":null},"operation_id":"example.observe","experiment_id":null,"outcome":"completed","reason":{"code":"retained","detail":"Synthetic replay observation record; no invoked operation"},"known_partial_effects":[],"recovery_owner_ref":null})
}
#[test]
fn transition_order_replay_and_task_guards() {
    let (s, mut p, e) = input("task");
    p["cycle"]["transitions"] = json!([observation(&p)]);
    c::validate(&s, &p, &e).unwrap();
    for mode in [
        "replay-effect",
        "skip-admission",
        "task-activation",
        "unknown-recovery",
        "budget-increase",
    ] {
        let mut q = p.clone();
        let t = &mut q["cycle"]["transitions"][0];
        match mode {
            "replay-effect" => t["effects"] = json!(["provider_call"]),
            "skip-admission" => {
                t["transition"] = json!("execute_task_action");
                t["target_state"] = json!("task_executed");
            }
            "task-activation" => {
                t["transition"] = json!("authorize_activation");
                t["source_state"] = json!("disposed");
                t["target_state"] = json!("activation_authorized");
            }
            "unknown-recovery" => {
                t["outcome"] = json!("recovery_required");
                t["target_state"] = json!("recovery_required");
            }
            _ => t["remaining_budget"]["calls"] = json!(999),
        }
        assert!(c::validate(&s, &q, &e).is_err(), "accepted {mode}");
    }
    let mut q = p.clone();
    let t = q["cycle"]["transitions"][0].clone();
    q["cycle"]["transitions"].as_array_mut().unwrap().push(t);
    assert!(c::validate(&s, &q, &e).is_err());
}
#[test]
fn protected_candidate_and_fabricated_posterior_refused() {
    let (s, p, e) = input("improvement");
    let mut q = p.clone();
    let a = q["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["artifact_type"] == "candidate")
        .unwrap();
    a["payload"]["mutation"]["provider_model_bindings"] = json!("override");
    assert!(c::validate(&s, &q, &e).is_err());
    let mut q = p.clone();
    let a = q["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["artifact_type"] == "belief_update")
        .unwrap();
    a["payload"]["mode"] = json!("quantitative");
    assert!(c::validate(&s, &q, &e).is_err());
}
#[test]
fn noncompletion_never_grants_admission_and_partial_history_refused() {
    let (s, p, e) = input("task");
    for outcome in ["denied", "deferred", "failed", "interrupted"] {
        let mut q = p.clone();
        let mut t = observation(&q);
        t["outcome"] = json!(outcome);
        t["target_state"] = json!("task_admitted");
        q["cycle"]["transitions"] = json!([t]);
        assert!(c::validate(&s, &q, &e).is_err());
    }
    let mut q = p.clone();
    let mut t = observation(&q);
    t["transition"] = json!("execute_task_action");
    t["source_state"] = json!("task_admitted");
    t["target_state"] = json!("task_executed");
    t["authority_ref"] = q["cycle"]["body_ref"].clone();
    q["cycle"]["transitions"] = json!([t]);
    assert!(c::validate(&s, &q, &e).is_err());
    let mut q = p.clone();
    let mut t = observation(&q);
    t["outcome"] = json!("denied");
    t["target_state"] = json!("stopped");
    q["cycle"]["transitions"] = json!([t]);
    c::validate(&s, &q, &e).unwrap();
    let mut q = p.clone();
    let mut t = observation(&q);
    t["correlation_ids"]["cycle_id"] = json!("other-cycle");
    q["cycle"]["transitions"] = json!([t]);
    assert!(c::validate(&s, &q, &e).is_err());
}
