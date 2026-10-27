//! PVF local_contract: deterministic identity/custody regression, small local
//! fixtures and bounded Git/archive reads; no provider/network. Required issue gate.
use serde_json::{json, Value};
use source_retirement_validator::*;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
fn repo() -> Repo {
    Repo::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.."))
}
fn reject<T>(r: anyhow::Result<T>) {
    assert!(r.is_err(), "expected refusal");
}
fn fixture() -> (Value, Tree, Tree, Paths, Paths) {
    let a = json!(["100644", "a".repeat(40)]);
    let b = json!(["100755", "b".repeat(40)]);
    let base = Tree::from([
        ("retained".into(), a.clone()),
        ("removed".into(), a.clone()),
        ("held".into(), a.clone()),
    ]);
    let candidate = Tree::from([("retained".into(), b.clone()), ("held".into(), a.clone())]);
    let manifest = json!({"schema":"adl.source_retirement.execution_overlay.v1","issue":1223,"status":"complete","missing_entries":[],"entries":[{"path":"retained","action":"modify","before":a,"after":b,"review_reference":"reviewed fixture","implementation_commit":"fixture","approval_reference":"fixture"}]});
    (
        manifest,
        base.clone(),
        candidate,
        Paths::from(["removed".into()]),
        base.keys().cloned().collect(),
    )
}
#[test]
fn execution_overlay_adversarial_cases() {
    let d = identities().unwrap();
    let (m, b, c, r, s) = fixture();
    assert_eq!(overlay::check(&m, &b, &c, &r, &s, &d).unwrap(), r);
    for field in ["status", "missing_entries", "schema", "issue"] {
        let mut x = m.clone();
        x[field] = match field {
            "missing_entries" => json!(["missing"]),
            "issue" => json!(0),
            _ => json!("bad"),
        };
        reject(overlay::check(&x, &b, &c, &r, &s, &d));
    }
    for field in [
        "review_reference",
        "implementation_commit",
        "approval_reference",
    ] {
        let mut x = m.clone();
        x["entries"][0][field] = json!("");
        reject(overlay::check(&x, &b, &c, &r, &s, &d));
    }
    let mut x = m.clone();
    x["entries"][0]["before"] = x["entries"][0]["after"].clone();
    reject(overlay::check(&x, &b, &c, &r, &s, &d));
    let mut x = m.clone();
    x["entries"]
        .as_array_mut()
        .unwrap()
        .push(m["entries"][0].clone());
    reject(overlay::check(&x, &b, &c, &r, &s, &d));
    for value in [b["retained"].clone(), json!(["100644", "b".repeat(40)])] {
        let mut x = c.clone();
        x.insert("retained".into(), value);
        reject(overlay::check(&m, &b, &x, &r, &s, &d));
    }
    let mut x = c.clone();
    x.remove("held");
    reject(overlay::check(&m, &b, &x, &r, &s, &d));
    let mut x = b.clone();
    x.insert("outside".into(), b["held"].clone());
    reject(overlay::check(&m, &x, &c, &r, &s, &d));
    let mut x = c.clone();
    x.insert("removed".into(), b["removed"].clone());
    reject(overlay::check(&m, &b, &x, &r, &s, &d));
    for (path, custody) in [("held", json!("proposal")), ("removed", Value::Null)] {
        let mut x = m.clone();
        x["entries"].as_array_mut().unwrap().push(json!({"path":path,"action":"remove","before":b[path],"after":null,"review_reference":"review","implementation_commit":"fixture","approval_reference":"proposal","rollback_custody":custody}));
        reject(overlay::check(&x, &b, &c, &r, &s, &d));
    }
    for identity in [
        json!(["100777", "b".repeat(40)]),
        json!(["100644", "z".repeat(40)]),
        json!(["100644", "short"]),
    ] {
        let mut x = m.clone();
        x["entries"][0]["after"] = identity;
        reject(overlay::check(&x, &b, &c, &r, &s, &d));
    }
}
#[test]
fn exact_transient_chains_and_refusals() {
    let d = identities().unwrap();
    for (path_key, id_key, add, remove) in [
        (
            "TRANSIENT_CONVERSION_BRIDGE",
            "TRANSIENT_BRIDGE_IDENTITY",
            "f598c0711d9b270a5b304c7cde64dcdf3068619e",
            "d293b17fe05f70633826017eb581f5315359f80f",
        ),
        (
            "TRANSIENT_RUNTIME_RESOURCES",
            "TRANSIENT_RUNTIME_IDENTITY",
            "4df3ff232116341f2dfecb4dfd4e5448157aed63",
            "7783ee5303651ace42d5d3fecd4055aa83a0a3a5",
        ),
    ] {
        let path = string(&d[path_key]).unwrap();
        let id = &d[id_key];
        let entry = json!({"path":path,"action":"remove_added","before":null,"after":null,"review_reference":"reviewed","implementation_commit":"fixture","approval_reference":"pinned packet","rollback_custody":{"mode":id[0],"blob":id[1]},"source_chain":[{"commit":add,"before":null,"after":id}],"supplemental_source_chain":[{"commit":remove,"before":id,"after":null}]});
        let m = json!({"schema":"adl.source_retirement.execution_overlay.v1","issue":1223,"status":"complete","missing_entries":[],"entries":[entry]});
        let r = Paths::from([path.into()]);
        let t = Tree::new();
        let s = Paths::new();
        assert_eq!(overlay::check(&m, &t, &t, &r, &s, &d).unwrap(), r);
        for field in ["source_chain", "supplemental_source_chain"] {
            let mut x = m.clone();
            x["entries"][0][field] = json!([]);
            reject(overlay::check(&x, &t, &t, &r, &s, &d));
        }
        let mut x = m.clone();
        x["entries"][0]["path"] = json!("unapproved");
        reject(overlay::check(
            &x,
            &t,
            &t,
            &Paths::from(["unapproved".into()]),
            &s,
            &d,
        ));
        let mut x = m.clone();
        x["entries"][0]["rollback_custody"]["blob"] = json!("0".repeat(40));
        reject(overlay::check(&x, &t, &t, &r, &s, &d));
    }
}
#[test]
fn combined_inventory_and_holds() {
    let repo = repo();
    let d = identities().unwrap();
    let o = repo.packet("DISPOSITION_REGISTER.json").unwrap();
    let c = repo.packet("COMBINED_REMAINING_DELTA.json").unwrap();
    let g = repo.packet("GROUPED_REMAINING_HOLDS.json").unwrap();
    let f = repo.tree(string(&o["original_revision"]).unwrap()).unwrap();
    let b = repo.tree(string(&o["observed_main"]).unwrap()).unwrap();
    inventory::combined(&o, &c, &g, &f, &b, &d).unwrap();
    let mut x = c.clone();
    x["rows"].as_array_mut().unwrap().swap(0, 1);
    reject(inventory::combined(&o, &x, &g, &f, &b, &d));
    let mut x = c.clone();
    x["additional_paths"][0]["path"] = c["rows"][0]["path"].clone();
    reject(inventory::combined(&o, &x, &g, &f, &b, &d));
    for (field, value) in [
        ("new_removals_authorized", false),
        ("all_holds_resolved", true),
    ] {
        let mut x = o.clone();
        x[field] = json!(value);
        reject(inventory::combined(&x, &c, &g, &f, &b, &d));
    }
    for group in ["rows", "additional_paths"] {
        for (field, value) in [
            ("mode", "100777"),
            ("blob", "0000000000000000000000000000000000000000"),
        ] {
            let mut x = c.clone();
            x[group][0]["current_identity"][field] = json!(value);
            reject(inventory::combined(&o, &x, &g, &f, &b, &d));
        }
    }
    let mut x = c.clone();
    x["rows"][0]["proposed_action"] = json!("hold_test_fixture");
    let mut counts = BTreeMap::<String, usize>::new();
    for r in rows(&x["rows"]).unwrap() {
        *counts
            .entry(string(&r["proposed_action"]).unwrap().into())
            .or_default() += 1;
    }
    x["counts"] = json!(counts);
    let mut groups = g.clone();
    groups["held_count"] = json!(1);
    groups["groups"] = json!([{"count":1,"rows":[x["rows"][0]]}]);
    inventory::combined(&o, &x, &groups, &f, &b, &d).unwrap();
    groups["groups"][0]["rows"] = json!([]);
    groups["groups"][0]["count"] = json!(0);
    reject(inventory::combined(&o, &x, &groups, &f, &b, &d));
    let mut changed = f.clone();
    changed.remove(string(&o["rows"][0]["path"]).unwrap());
    reject(inventory::combined(&o, &c, &g, &changed, &b, &d));
}
#[test]
fn separately_observed_scope() {
    let d = identities().unwrap();
    let records = &d["SEPARATELY_OBSERVED_RETIREMENTS"];
    let c = json!({"rows":[],"additional_paths":[],"separately_observed_retirements":records});
    let observed = rows(records)
        .unwrap()
        .iter()
        .map(|r| {
            (
                string(&r["path"]).unwrap().to_owned(),
                identity(&r["execution_baseline_identity"]),
            )
        })
        .collect::<Tree>();
    let removed = observed.keys().cloned().collect::<Paths>();
    inventory::separately_observed(&c, &Tree::new(), &observed, &removed, &d).unwrap();
    let mut x = c.clone();
    x["separately_observed_retirements"][0]["path"] = json!("foreign");
    reject(inventory::separately_observed(
        &x,
        &Tree::new(),
        &observed,
        &removed,
        &d,
    ));
    reject(inventory::separately_observed(
        &c, &observed, &observed, &removed, &d,
    ));
    reject(inventory::separately_observed(
        &c,
        &Tree::new(),
        &observed,
        &Paths::new(),
        &d,
    ));
    let mut x = c.clone();
    x["additional_paths"] = json!([{"path":records[0]["path"]}]);
    reject(inventory::separately_observed(
        &x,
        &Tree::new(),
        &observed,
        &removed,
        &d,
    ));
}
#[test]
fn committed_inputs_ignore_working_files() {
    let t = tempfile::tempdir().unwrap();
    let r = Repo::new(t.path());
    r.git(&["init", "-q"]).unwrap();
    std::fs::write(t.path().join("input.json"), b"{\"allowed\":[]}").unwrap();
    r.git(&["add", "input.json"]).unwrap();
    r.git(&[
        "-c",
        "user.name=Fixture",
        "-c",
        "user.email=fixture@example.invalid",
        "-c",
        "commit.gpgsign=false",
        "commit",
        "-qm",
        "fixture",
    ])
    .unwrap();
    std::fs::write(t.path().join("input.json"), b"{\"allowed\":[\"live\"]}").unwrap();
    assert_eq!(r.json("input.json").unwrap(), json!({"allowed":[]}));
    reject(r.pinned("input.json", &"0".repeat(64)));
}
#[test]
fn rollback_and_reference_refusals() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("archive");
    reject(check_rollback(&p, "pin"));
    std::fs::write(&p, b"wrong").unwrap();
    reject(check_rollback(&p, "pin"));
    reject(check_references(
        &Paths::from(["adl/src/live.rs".into()]),
        &Paths::from([".csdlc/old.md".into()]),
    ));
}
struct FamilyCase {
    packet: &'static str,
    commit: &'static str,
    count: usize,
    kind: usize,
}
fn cases() -> Vec<FamilyCase> {
    vec![
        FamilyCase {
            packet: "CODEFRIEND_SIX_HELPER_APPLIED.json",
            commit: "CODEFRIEND_IMPLEMENTATION",
            count: 6,
            kind: 0,
        },
        FamilyCase {
            packet: "CODEFRIEND_FAMILY_APPLIED.json",
            commit: "CODEFRIEND_FAMILY_IMPLEMENTATION",
            count: 198,
            kind: 1,
        },
        FamilyCase {
            packet: "CSDLC_SOURCE_FAMILY_APPLIED.json",
            commit: "CSDLC_FAMILY_IMPLEMENTATION",
            count: 815,
            kind: 2,
        },
        FamilyCase {
            packet: "RUNTIME_SOURCE_FAMILY_APPLIED.json",
            commit: "RUNTIME_FAMILY_IMPLEMENTATION",
            count: 896,
            kind: 3,
        },
        FamilyCase {
            packet: "INFRASTRUCTURE_SOURCE_FAMILY_APPLIED.json",
            commit: "INFRASTRUCTURE_FAMILY_IMPLEMENTATION",
            count: 212,
            kind: 4,
        },
    ]
}
fn validate(
    k: usize,
    b: &Value,
    bt: &Tree,
    at: &Tree,
    p: &Paths,
    d: &Value,
) -> anyhow::Result<Paths> {
    match k {
        0 => family::codefriend_batch(b, bt, at, p, d),
        1 => family::codefriend(b, bt, at, p),
        2 => family::csdlc(b, bt, at, p, d),
        3 => family::runtime(b, bt, at, p, d),
        _ => family::infrastructure(b, bt, at, p, d),
    }
}
#[test]
fn family_scopes_preimages_custody_and_protected_paths() {
    let repo = repo();
    let d = identities().unwrap();
    for case in cases() {
        let batch = repo.packet(case.packet).unwrap();
        let rev = string(&d[case.commit]).unwrap();
        let before = repo.tree(&format!("{rev}^")).unwrap();
        let after = repo.tree(rev).unwrap();
        let p = validate(case.kind, &batch, &before, &after, &Paths::new(), &d).unwrap();
        assert_eq!(p.len(), case.count);
        reject(validate(case.kind, &batch, &before, &after, &p, &d));
        let key = if case.kind == 0 { "removed" } else { "files" };
        let index = rows(&batch[key])
            .unwrap()
            .iter()
            .position(|r| case.kind == 0 || r["action"] == "remove")
            .unwrap();
        let row = &batch[key][index];
        let path = string(&row["path"]).unwrap();
        let mut changed = before.clone();
        changed.insert(path.into(), json!(["100755", "0".repeat(40)]));
        reject(validate(
            case.kind,
            &batch,
            &changed,
            &after,
            &Paths::new(),
            &d,
        ));
        let mut changed = after.clone();
        changed.insert(path.into(), before[path].clone());
        reject(validate(
            case.kind,
            &batch,
            &before,
            &changed,
            &Paths::new(),
            &d,
        ));
        let mut b = batch.clone();
        match case.kind {
            0 => b[key][index]["rollback"]["archive_member_sha256"] = json!("wrong"),
            1 => b["rollback_custody"]["rows"][0]["preimage_sha256"] = json!("wrong"),
            2 => b["rollback"]["rows"][0]["sha256"] = json!("wrong"),
            3 => b["prepared_patch"]["deletions"][0]["current_sha256"] = json!("wrong"),
            _ => b["files"][0]["rollback"]["sha256"] = json!("wrong"),
        };
        reject(validate(case.kind, &b, &before, &after, &Paths::new(), &d));
        if case.kind >= 2 {
            for path in [
                "csdlc-v3/operator/authority-selector.json",
                "csdlc-v2/operator/retained.md",
                ".csdlc/evidence/old",
                "docs/templates/prompts/current.json",
            ] {
                let mut b = batch.clone();
                b[key][index]["path"] = json!(path);
                reject(validate(case.kind, &b, &before, &after, &Paths::new(), &d));
            }
        }
        if case.kind == 3 {
            for p in strings(&d["RUNTIME_PUBLIC_RETAINS"]).unwrap() {
                let mut a = after.clone();
                a.insert(p, json!(["100644", "0".repeat(40)]));
                reject(validate(case.kind, &batch, &before, &a, &Paths::new(), &d));
            }
        }
        for field in ["id"] {
            let mut b = batch.clone();
            if case.kind == 0 || case.kind == 4 {
                b[key][index][field] = json!("RD07-9999");
            } else if case.kind == 2 {
                b["rollback"]["rows"][0][field] = json!("RD07-9999");
            } else if case.kind == 3 {
                b["prepared_patch"]["deletions"][0][field] = json!("RD07-9999");
            } else {
                continue;
            }
            reject(validate(case.kind, &b, &before, &after, &Paths::new(), &d));
        }
    }
}
#[test]
fn recent_families_exact_scope_and_negative_matrix() {
    let repo = repo();
    let d = identities().unwrap();
    let mut total = 0;
    for key in ["CONVERSION_SKILLS_BATCHES", "RECENT_OWNER_BATCHES"] {
        for spec in rows(&d[key]).unwrap() {
            let batch = repo.packet(string(&spec["packet"]).unwrap()).unwrap();
            let rev = string(&spec["commit"]).unwrap();
            let before = repo.tree(&format!("{rev}^")).unwrap();
            let after = repo.tree(rev).unwrap();
            let p = family::recent(&batch, &before, &after, &Paths::new(), spec, &d).unwrap();
            assert_eq!(p.len(), number(&spec["removals"]).unwrap());
            if key == "RECENT_OWNER_BATCHES" {
                total += p.len();
            }
            if !p.is_empty() {
                reject(family::recent(&batch, &before, &after, &p, spec, &d));
            }
            let mut b = batch.clone();
            b["files"]
                .as_array_mut()
                .unwrap()
                .push(batch["files"][0].clone());
            reject(family::recent(&b, &before, &after, &Paths::new(), spec, &d));
            for r in rows(&batch["files"]).unwrap().iter().take(1).chain(
                rows(&batch["files"])
                    .unwrap()
                    .iter()
                    .filter(|r| r["action"] == "add")
                    .take(1),
            ) {
                let path = string(&r["path"]).unwrap();
                let mut a = after.clone();
                a.insert(path.into(), json!(["100644", "0".repeat(40)]));
                reject(family::recent(&batch, &before, &a, &Paths::new(), spec, &d));
                let mut b = before.clone();
                b.insert(path.into(), json!(["100644", "0".repeat(40)]));
                reject(family::recent(&batch, &b, &after, &Paths::new(), spec, &d));
            }
            if p.is_empty() {
                assert!(!rows(&batch["files"]).unwrap().is_empty());
                let mut b = batch.clone();
                b["files"][0]["action"] = json!("remove");
                b["files"][0]["after"] = Value::Null;
                reject(family::recent(&b, &before, &after, &Paths::new(), spec, &d));
            }
        }
    }
    assert_eq!(total, 1076);
}
#[test]
fn all_reviewed_custody_and_substitution_refusals() {
    let repo = repo();
    let d = identities().unwrap();
    let original = PathBuf::from(
        repo.text(&["rev-parse", "--path-format=absolute", "--git-common-dir"])
            .unwrap(),
    )
    .join("csdlc-v3/local/evidence/1223/retirement/baseline.tar.gz");
    let mut a = custody::Archives::default();
    check_rollback(&original, string(&d["ARCHIVE_SHA256"]).unwrap()).unwrap();
    for case in cases() {
        let b = repo.packet(case.packet).unwrap();
        match case.kind {
            0 => custody::codefriend_six(&mut a, &b, &original, &d),
            1 => custody::codefriend(&mut a, &b, &original, &d),
            2 => custody::csdlc(&mut a, &b, &original, &d),
            3 => custody::runtime(&mut a, &b, &original, &d),
            _ => custody::infrastructure(&mut a, &b),
        }
        .unwrap();
    }
    for key in ["CONVERSION_SKILLS_BATCHES", "RECENT_OWNER_BATCHES"] {
        for spec in rows(&d[key]).unwrap() {
            let b = repo.packet(string(&spec["packet"]).unwrap()).unwrap();
            if key == "CONVERSION_SKILLS_BATCHES" {
                custody::conversion(&mut a, &b, spec).unwrap();
            } else {
                custody::recent(&mut a, &b, spec).unwrap();
            }
        }
    }
    let spec = &d["RECENT_OWNER_BATCHES"][0];
    let mut b = repo.packet(string(&spec["packet"]).unwrap()).unwrap();
    b["files"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["action"] == "remove")
        .unwrap()["before"]["sha256"] = json!("wrong");
    reject(custody::recent(&mut a, &b, spec));
    let tmp = tempfile::tempdir().unwrap();
    let fake = tmp.path().join("bad.tar");
    std::fs::write(&fake, b"not the reviewed archive").unwrap();
    let mut s = spec.clone();
    s["archives"][0]["path"] = json!(fake);
    reject(custody::recent(&mut a, &b, &s));
}
#[test]
fn archive_member_mode_bytes_and_scope() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("fixture.tar");
    let mut tar = tar::Builder::new(std::fs::File::create(&path).unwrap());
    let mut header = tar::Header::new_ustar();
    header.set_size(4);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append_data(&mut header, "source/a", &b"good"[..])
        .unwrap();
    tar.finish().unwrap();
    drop(tar);
    let pin = sha(&std::fs::read(&path).unwrap());
    let mut a = custody::Archives::default();
    a.member(&path, &pin, "source/a", 0o644, &sha(b"good"))
        .unwrap();
    reject(a.member(&path, &pin, "source/a", 0o755, &sha(b"good")));
    reject(a.member(&path, &pin, "source/a", 0o644, &sha(b"bad")));
    reject(a.member(&path, &pin, "source/missing", 0o644, &sha(b"good")));
    reject(a.exact(&path, &pin, &Paths::new()));
    reject(a.load(&path, &"0".repeat(64)));
}

#[test]
fn historical_runtime_retirement_preserves_custody_and_rejects_drift() {
    let repo = repo();
    let candidate = repo.tree("HEAD").unwrap();
    let historical = successors::runtime_remnants(&repo, &candidate).unwrap();
    let path = "infra/runtime-v3/runtime-init.toml";
    assert!(!candidate.contains_key(path));
    assert!(historical.contains_key(path));
    let mut restored = candidate.clone();
    restored.insert(path.into(), historical[path].clone());
    reject(successors::runtime_remnants(&repo, &restored));
    let mut new_source = candidate.clone();
    new_source.insert("csdlc-v2/unreviewed.rs".into(), historical[path].clone());
    reject(successors::runtime_remnants(&repo, &new_source));
    let mut changed_consumer = candidate.clone();
    changed_consumer.remove("adl-characterization/corpus/v2/shadow.yaml");
    reject(successors::runtime_remnants(&repo, &changed_consumer));
    for kept in ["README.md", "adl/src/cli/runtime_owner.rs", "csdlc-v3/operator/authority-selector.json", "csdlc-v3/operator/native-authority-receipt.json", "csdlc-v3/operator/native-authority-pr-observation.json"] {
        assert_eq!(candidate[kept], historical[kept]);
    }
}
