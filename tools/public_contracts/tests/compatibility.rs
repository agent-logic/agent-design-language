//! PVF fast deterministic synthetic correspondence/refusals, never qualification.
use adl_public_contracts::{archive, compatibility as v};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};
struct Fixture {
    _temp: tempfile::TempDir,
    root: PathBuf,
    receipt: Value,
    lock: Value,
    graph: Value,
    rollback: Value,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(temp.path()).unwrap();
        fs::write(root.join("fixture.json"), "{\"fixture_only\":true}\n").unwrap();
        let receipt = json!({"path":"fixture.json","sha256":archive::digest(&root.join("fixture.json")).unwrap()});
        let doc = adl_public_contracts::root()
            .join("docs/milestones/v0.93.1/repository-decomposition/rd11-1193");
        Self {
            _temp: tempfile::tempdir().unwrap(),
            root,
            receipt,
            lock: v::load(&doc.join("candidate-lockset.json")).unwrap(),
            graph: v::load(&doc.join("interface-graph.json")).unwrap(),
            rollback: v::load(&doc.join("rollback-catalog.json")).unwrap(),
        }
        .keep(temp)
    }
    fn keep(mut self, temp: tempfile::TempDir) -> Self {
        self._temp = temp;
        self
    }
    fn check(&self) -> anyhow::Result<Value> {
        v::verify(
            &self.lock,
            &self.graph,
            &self.rollback,
            Some(&self.root),
            false,
        )
    }
    fn qualified(&mut self) {
        self.lock["state"] = "accepted".into();
        self.lock["rd07_acceptance"] = self.receipt.clone();
        self.lock["independent_review"] = self.receipt.clone();
        self.lock["negative_evidence"]=Value::Array(v::NEGATIVES.iter().map(|name|json!({"case":name,"observed_refusal":"synthetic fixture refusal","executed":1,"evidence":self.receipt})).collect());
        for row in self.lock["roles"].as_array_mut().unwrap() {
            row["selected"] = json!({"source_commit":"a".repeat(40),"asset_identity":{"name":format!("fixture:{}",row["role"].as_str().unwrap()),"path":"fixture.json"},"sha256":self.receipt["sha256"],"package_version":"0.1.0","dependency_lock":self.receipt,"build_provenance":self.receipt,"supported_consumer_versions":["=0.1.0"]});
            row["active_source_owners"] = json!(["fixture-owner"]);
            row["authentication"] = self.receipt.clone();
            if row["role"] == "website" {
                row["website_disposition"] = "preserved_no_deployed_baseline".into();
            } else {
                row["platforms"] = json!([{"target":"fixture-platform","source_commit":"a".repeat(40),"artifact_sha256":self.receipt["sha256"],"independent_clean":true,"distributable_only":true,"operations":[{"operation":"build","argv":["fixture","build"],"exit_code":0,"executed":1,"skipped":0},{"operation":"test","argv":["fixture","test"],"exit_code":0,"executed":1,"skipped":0},{"operation":"install","argv":["fixture","install"],"exit_code":0,"executed":1,"skipped":0}],"evidence":self.receipt}]);
            }
        }
        for e in self.graph["edges"].as_array_mut().unwrap() {
            if e["interface"].is_null() {
                e["interface"] = "fixture-interface".into();
            }
            e["version"] = "1".into();
            e["evidence"] = self.receipt.clone();
        }
        for r in self.rollback["roles"].as_array_mut().unwrap() {
            r["disposition"] = "no_prior_accepted_baseline".into();
            r["prior"] = Value::Null;
            r["restore_argv"] = json!([]);
            r["recoverability"] = self.receipt.clone();
        }
        if let Some(ds) = self.graph["dependency_dispositions"].as_array_mut() {
            for d in ds {
                let owner = self.lock["roles"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["role"] == d["consumer"])
                    .unwrap();
                let selected = &owner["selected"];
                d["selected_source_commit"] = selected["source_commit"].clone();
                d["selected_artifact_sha256"] = selected["sha256"].clone();
                for k in ["dependency_lock", "build_provenance"] {
                    d[k] = selected[k].clone();
                }
                d["authentication"] = owner["authentication"].clone();
                d["evidence"] = self.receipt.clone();
            }
        }
    }
    fn pending(&mut self) {
        for row in self.lock["roles"].as_array_mut().unwrap() {
            for k in ["selected", "authentication"] {
                row[k] = Value::Null;
            }
            row["active_source_owners"] = json!([]);
            row["platforms"] = json!([]);
        }
        self.graph["schema"] = "adl.compatibility_graph.v1".into();
        self.graph
            .as_object_mut()
            .unwrap()
            .remove("dependency_dispositions");
        let edges = self.graph["edges"].as_array_mut().unwrap();
        if !edges
            .iter()
            .any(|e| key(e) == ("codefriend", "runtime", "build", "runtime-source"))
        {
            edges.push(json!({"consumer":"codefriend","producer":"runtime","kind":"build","interface":"runtime-source","version":"0.92.1","optional":false,"evidence":self.receipt}));
        }
    }
    fn independent(&mut self) {
        self.qualified();
        self.graph["schema"] = "adl.compatibility_graph.v2".into();
        self.graph["edges"]
            .as_array_mut()
            .unwrap()
            .retain(|e| key(e) != ("codefriend", "runtime", "build", "runtime-source"));
        let owner = self.lock["roles"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["role"] == "codefriend")
            .unwrap();
        let s = &owner["selected"];
        self.graph["dependency_dispositions"] = json!([{"schema":"adl.compatibility_dependency_disposition.v1","consumer":"codefriend","producer":"runtime","kind":"build","interface":"runtime-source","disposition":"independently_built_no_dependency","selected_source_commit":s["source_commit"],"selected_artifact_sha256":s["sha256"],"dependency_lock":s["dependency_lock"],"build_provenance":s["build_provenance"],"authentication":owner["authentication"],"evidence":self.receipt}]);
    }
    fn bundled(&mut self) {
        self.qualified();
        let files = [
            ("engine.tar".to_owned(), (b"engine.tar".to_vec(), 0o644)),
            ("records.tar".to_owned(), (b"records.tar".to_vec(), 0o644)),
        ]
        .into_iter()
        .collect();
        fs::write(self.root.join("bundle.tar"), archive::pack(&files).unwrap()).unwrap();
        let hash = archive::digest(&self.root.join("bundle.tar")).unwrap();
        for (i, name) in ["engine.tar", "records.tar"].iter().enumerate() {
            let row = &mut self.lock["roles"][i];
            row["selected"]["asset_identity"] = json!({"name":"fixture-bundle:1","path":"bundle.tar","member":{"path":name,"sha256":archive::sha(name.as_bytes())}});
            row["selected"]["sha256"] = hash.clone().into();
            row["platforms"][0]["artifact_sha256"] = hash.clone().into();
        }
    }
    fn refused(&self, label: &str) {
        let error = self.check().expect_err("mutation must refuse").to_string();
        assert!(error.contains(label), "expected {label:?}, got {error:?}");
    }
}
fn key(v: &Value) -> (&str, &str, &str, &str) {
    (
        v["consumer"].as_str().unwrap_or(""),
        v["producer"].as_str().unwrap_or(""),
        v["kind"].as_str().unwrap_or(""),
        v["interface"].as_str().unwrap_or(""),
    )
}
#[test]
fn pending_roles() {
    let mut f = Fixture::new();
    f.pending();
    let r = f.check().unwrap();
    assert_eq!(r["pending_selections"].as_array().unwrap().len(), 7);
    assert_eq!(r["acceptance_authority"], false);
    assert!(v::verify(&f.lock, &f.graph, &f.rollback, Some(&f.root), true).is_err());
}
#[test]
fn synthetic_correspondence_never_grants_authority() {
    let mut f = Fixture::new();
    f.qualified();
    let r = f.check().unwrap();
    assert_eq!(r["status"], "qualified_evidence_correspondence");
    assert_eq!(r["acceptance_authority"], false);
    assert_eq!(r["activation_performed"], false);
}
#[test]
fn independent_codefriend() {
    let mut f = Fixture::new();
    f.independent();
    assert_eq!(f.check().unwrap()["acceptance_authority"], false);
}
#[test]
fn independent_requires_evidence() {
    let mut f = Fixture::new();
    f.independent();
    f.graph["dependency_dispositions"][0]["evidence"] = Value::Null;
    f.refused("dependency disposition evidence missing");
}
#[test]
fn independent_matches_identity() {
    for (field, value, label) in [
        (
            "selected_artifact_sha256",
            json!("0".repeat(64)),
            "selected identity mismatch",
        ),
        (
            "dependency_lock",
            json!({"path":"other.json","sha256":"0".repeat(64)}),
            "source declaration mismatch",
        ),
    ] {
        let mut f = Fixture::new();
        f.independent();
        f.graph["dependency_dispositions"][0][field] = value;
        f.refused(label);
    }
}
#[test]
fn missing_product_proof() {
    let mut f = Fixture::new();
    f.qualified();
    f.lock["roles"][0]["authentication"] = Value::Null;
    f.refused("authentication missing");
}
#[test]
fn mixed_owners() {
    let mut f = Fixture::new();
    f.lock["roles"][0]["active_source_owners"] = json!(["a", "b"]);
    assert!(f.check().is_err());
}
#[test]
fn shared_activation() {
    let mut f = Fixture::new();
    f.lock["roles"][0]["activation"] = "install-shared".into();
    assert!(f.check().is_err());
}
#[test]
fn duplicate_role() {
    let mut f = Fixture::new();
    f.lock["roles"][6] = f.lock["roles"][0].clone();
    f.refused("seven unique roles");
}
#[test]
fn duplicate_assets_ignore_transport_path() {
    let mut f = Fixture::new();
    f.qualified();
    f.lock["roles"][1]["selected"] = f.lock["roles"][0]["selected"].clone();
    f.refused("duplicate selected asset");
    fs::copy(f.root.join("fixture.json"), f.root.join("copy.json")).unwrap();
    f.lock["roles"][1]["selected"]["asset_identity"]["path"] = "copy.json".into();
    f.refused("duplicate selected asset");
}
#[test]
fn distinct_bundle_members() {
    let mut f = Fixture::new();
    f.bundled();
    assert_eq!(f.check().unwrap()["acceptance_authority"], false);
}
#[test]
fn duplicate_bundle_member() {
    let mut f = Fixture::new();
    f.bundled();
    f.lock["roles"][1]["selected"] = f.lock["roles"][0]["selected"].clone();
    f.refused("duplicate selected asset");
}
#[test]
fn invented_bundle_member() {
    for (k, v) in [("path", "missing.tar".into()), ("sha256", "0".repeat(64))] {
        let mut f = Fixture::new();
        f.bundled();
        f.lock["roles"][0]["selected"]["asset_identity"]["member"][k] = v.into();
        f.refused("bundle member");
    }
}
#[test]
fn whole_bundle_overlap() {
    let mut f = Fixture::new();
    f.bundled();
    f.lock["roles"][0]["selected"]["asset_identity"]
        .as_object_mut()
        .unwrap()
        .remove("member");
    f.refused("whole bundle overlaps");
}
#[test]
fn every_concrete_interface_required() {
    for obligation in v::REQUIRED {
        let mut f = Fixture::new();
        f.qualified();
        f.graph["edges"]
            .as_array_mut()
            .unwrap()
            .retain(|e| key(e) != obligation);
        if let Some(ds) = f.graph["dependency_dispositions"].as_array_mut() {
            ds.retain(|e| key(e) != obligation);
        }
        f.refused("interface/kind obligation");
    }
}
#[test]
fn required_kind_and_optionality() {
    for obligation in v::REQUIRED {
        for (k, v) in [("kind", json!("runtime")), ("optional", json!(true))] {
            let mut f = Fixture::new();
            f.qualified();
            let mut count = 0;
            for e in f.graph["edges"].as_array_mut().unwrap() {
                if key(e) == obligation {
                    e[k] = v.clone();
                    count += 1;
                }
            }
            if let Some(ds) = f.graph["dependency_dispositions"].as_array_mut() {
                for d in ds {
                    if key(d) == obligation {
                        d[k] = v.clone();
                        count += 1;
                    }
                }
            }
            assert_eq!(count, 1);
            assert!(f.check().is_err());
        }
    }
}
#[test]
fn artifact_platform_mismatch() {
    let mut f = Fixture::new();
    f.qualified();
    f.lock["roles"][0]["platforms"][0]["artifact_sha256"] = "c".repeat(64).into();
    f.refused("platform artifact/source mismatch");
}
#[test]
fn zero_skipped_failed_operations() {
    for (k, v) in [
        ("executed", json!(0)),
        ("skipped", json!(1)),
        ("exit_code", json!(1)),
    ] {
        let mut f = Fixture::new();
        f.qualified();
        f.lock["roles"][0]["platforms"][0]["operations"][1][k] = v;
        f.refused("failed/zero/skipped operation");
    }
}
#[test]
fn single_adapter_invocation_and_phases() {
    let mut f = Fixture::new();
    f.qualified();
    let mut row = f.lock["roles"][0]["platforms"][0].clone();
    row.as_object_mut().unwrap().remove("operations");
    row["adapter_invocation"] =
        json!({"argv":["fixture","adapter"],"exit_code":0,"executed":1,"skipped":0});
    row["phase_outcomes"] = Value::Array(
        ["build", "test", "install"]
            .iter()
            .map(|p| json!({"phase":p,"executed":1,"failed":0,"skipped":0,"claim":"fixture phase"}))
            .collect(),
    );
    f.lock["roles"][0]["platforms"] = json!([row]);
    assert!(f.check().is_ok());
    for (k, v) in [
        ("executed", json!(2)),
        ("executed", json!(true)),
        ("skipped", json!(1)),
        ("skipped", json!(false)),
        ("exit_code", json!(1)),
        ("exit_code", json!(false)),
    ] {
        f.lock["roles"][0]["platforms"] = json!([row]);
        f.lock["roles"][0]["platforms"][0]["adapter_invocation"][k] = v;
        f.refused("exactly one successful invocation");
    }
    for (k, v) in [
        ("executed", json!(0)),
        ("executed", json!(true)),
        ("failed", json!(1)),
        ("failed", json!(false)),
        ("skipped", json!(1)),
        ("skipped", json!(false)),
    ] {
        f.lock["roles"][0]["platforms"] = json!([row]);
        f.lock["roles"][0]["platforms"][0]["phase_outcomes"][0][k] = v;
        f.refused("adapter phase");
    }
}
#[test]
fn retained_runtime_stays_deferred() {
    let mut f = Fixture::new();
    f.qualified();
    let index = f.lock["roles"]
        .as_array()
        .unwrap()
        .iter()
        .position(|r| r["role"] == "runtime")
        .unwrap();
    let mut row = f.lock["roles"][index]["platforms"][0].clone();
    row.as_object_mut().unwrap().remove("operations");
    row["retained_acceptance"] = json!({"build_provenance":f.receipt,"installation_identity":f.receipt,"scenario_acceptance":f.receipt,"independent_review":f.receipt,"scope_decision":f.receipt,"scenarios_passed":7,"exact_install_invocation_retained":false,"current_qualification_status":"operator_deferred_outside_sprint1","limitations":["historical aggregate install argv was not retained"]});
    f.lock["roles"][index]["platforms"] = json!([row]);
    assert!(f.check().is_ok());
    for (k, v, label) in [
        ("scenarios_passed", json!(0), "scenario denominator"),
        ("scenarios_passed", json!(true), "scenario denominator"),
        (
            "exact_install_invocation_retained",
            json!(true),
            "explicitly unavailable",
        ),
        (
            "current_qualification_status",
            json!("qualified"),
            "qualification disposition",
        ),
    ] {
        f.lock["roles"][index]["platforms"] = json!([row]);
        f.lock["roles"][index]["platforms"][0]["retained_acceptance"][k] = v;
        f.refused(label);
    }
    f.qualified();
    f.lock["roles"][0]["platforms"] = json!([row]);
    f.refused("Runtime-only");
}
#[test]
fn mutated_evidence() {
    let mut f = Fixture::new();
    f.qualified();
    fs::write(f.root.join("fixture.json"), "changed").unwrap();
    f.refused("bytes mismatch");
}
#[test]
fn evidence_escape_and_symlink() {
    for name in ["../outside", "/absolute", "link"] {
        let mut f = Fixture::new();
        f.qualified();
        if name == "link" {
            #[cfg(unix)]
            std::os::unix::fs::symlink(f.root.join("fixture.json"), f.root.join("link")).unwrap();
        }
        f.lock["rd07_acceptance"] = json!({"path":name,"sha256":f.receipt["sha256"]});
        assert!(f.check().is_err());
    }
}
#[test]
fn standalone_csdlc_runtime_forbidden() {
    let mut f = Fixture::new();
    f.pending();
    let mut e = f.graph["edges"][0].clone();
    e["consumer"] = "csdlc".into();
    e["producer"] = "runtime".into();
    f.graph["edges"].as_array_mut().unwrap().push(e);
    f.refused("standalone CSDLC Runtime dependency");
}
#[test]
fn build_cycle_and_missing_graph() {
    let mut f = Fixture::new();
    f.pending();
    let mut e = f.graph["edges"][0].clone();
    e["consumer"] = "public_adl".into();
    e["producer"] = "runtime".into();
    e["kind"] = "build".into();
    f.graph["edges"].as_array_mut().unwrap().push(e);
    f.refused("build-order cycle");
    f.graph["edges"] = json!([]);
    f.refused("interface/kind obligation");
}
#[test]
fn mandatory_enterprise_forbidden() {
    let mut f = Fixture::new();
    f.pending();
    let e = f.graph["edges"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|e| e["consumer"] == "runtime" && e["producer"] == "enterprise_adapter")
        .unwrap();
    e["optional"] = false.into();
    f.refused("mandatory enterprise dependency");
}
#[test]
fn unresolved_rollback_and_negatives() {
    let mut f = Fixture::new();
    f.qualified();
    f.rollback["roles"][0]["disposition"] = "pending".into();
    f.refused("rollback unresolved");
    f.qualified();
    f.lock["negative_evidence"].as_array_mut().unwrap().pop();
    f.refused("original negative proof missing");
}
#[test]
fn retained_generation() {
    let mut f = Fixture::new();
    f.qualified();
    let receipt = json!({"source_revision":"b".repeat(40),"generation":"fixture-generation","platform":"fixture-platform","artifacts":{"kernel":{"sha256":f.receipt["sha256"]},"guardian":{"sha256":f.receipt["sha256"]},"csm":{"sha256":f.receipt["sha256"]}}});
    fs::write(f.root.join("generation.json"), receipt.to_string()).unwrap();
    let mut point = json!({"source_commit":"b".repeat(40),"generation":"fixture-generation","platform":"fixture-platform","artifacts":[{"name":"adl-runtime-kernel","path":"fixture.json","sha256":f.receipt["sha256"]},{"name":"adl-runtime-guardian","path":"fixture.json","sha256":f.receipt["sha256"]},{"name":"csm","path":"fixture.json","sha256":f.receipt["sha256"]}],"receipt":{"path":"generation.json","sha256":archive::digest(&f.root.join("generation.json")).unwrap()},"supported_consumer_versions":["fixture runtime generation"],"replay_authority":false});
    let i = f.rollback["roles"]
        .as_array()
        .unwrap()
        .iter()
        .position(|r| r["role"] == "runtime")
        .unwrap();
    f.rollback["roles"][i]["disposition"] = "retained_installed_generation".into();
    f.rollback["roles"][i]["prior"] = point.clone();
    assert!(f.check().is_ok());
    let mut extra = point["artifacts"].clone();
    extra
        .as_array_mut()
        .unwrap()
        .push(json!({"name":"extra","path":"fixture.json","sha256":f.receipt["sha256"]}));
    for (k, v, label) in [
        (
            "replay_authority",
            json!(true),
            "replay authority forbidden",
        ),
        ("artifacts", json!([]), "artifacts missing"),
        (
            "generation",
            json!("other-generation"),
            "receipt identity mismatch",
        ),
        (
            "artifacts",
            json!([{"name":"kernel","path":"fixture.json","sha256":f.receipt["sha256"]}]),
            "exact installed generation artifacts",
        ),
        ("artifacts", extra, "exact installed generation artifacts"),
    ] {
        f.rollback["roles"][i]["prior"] = point.clone();
        f.rollback["roles"][i]["prior"][k] = v;
        f.refused(label);
    }
    let mut corrupted = receipt.clone();
    corrupted["artifacts"]["kernel"]["sha256"] = "0".repeat(64).into();
    fs::write(f.root.join("generation.json"), corrupted.to_string()).unwrap();
    point["receipt"]["sha256"] = archive::digest(&f.root.join("generation.json"))
        .unwrap()
        .into();
    f.rollback["roles"][i]["prior"] = point.clone();
    f.refused("receipt artifact mismatch");
    fs::write(f.root.join("generation.json"), receipt.to_string()).unwrap();
    point["receipt"]["sha256"] = archive::digest(&f.root.join("generation.json"))
        .unwrap()
        .into();
    f.rollback["roles"][i]["prior"] = point;
    f.rollback["roles"][i]["restore_argv"] = json!(["missing-restorer", "missing-file"]);
    f.refused("cannot imply replay argv");
}
#[test]
fn private_paths_and_mutable_identity() {
    let mut f = Fixture::new();
    f.lock["roles"][0]["available"][0]["description"] = "/Users/private/source".into();
    f.refused("private host content");
    f.lock["roles"][0]["available"][0]["description"] = "fixture".into();
    f.qualified();
    f.lock["roles"][0]["selected"]["asset_identity"]["name"] =
        "https://example.com/releases/latest".into();
    f.refused("mutable artifact identity");
}
#[test]
fn duplicate_json_keys() {
    let f = Fixture::new();
    fs::write(f.root.join("fixture.json"), r#"{"schema":1,"schema":2}"#).unwrap();
    assert!(v::load(&f.root.join("fixture.json")).is_err());
}
