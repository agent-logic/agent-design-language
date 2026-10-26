//! PVF local_contract: deterministic offline report-consistency regression;
//! small resources, no network/interpreters, not lifecycle or release acceptance.
use adl_public_contracts::adapter_evidence::{load, verify, Documents};
use serde_json::{json, Value};
fn documents() -> Documents {
    load(&adl_public_contracts::root().join("docs/repository-split/adapter-admission-1275"))
        .unwrap()
}
fn reject(mut docs: Documents, document: &str, pointer: &str, value: Value) {
    *docs
        .get_mut(document)
        .unwrap()
        .pointer_mut(pointer)
        .unwrap() = value;
    assert!(
        verify(&docs).is_err(),
        "accepted mutation {document}{pointer}"
    );
}
#[test]
fn current_report_is_consistent_without_claiming_acceptance() {
    verify(&documents()).unwrap();
}
#[test]
fn identities_authority_and_aggregate_cannot_be_forged() {
    let docs = documents();
    let admitted_repo = &docs["ADMISSION"]["repositories"][0]["repository"];
    let contract_index = docs["CONTRACTS"]["repositories"]
        .as_array()
        .unwrap()
        .iter()
        .position(|row| &row["repository"] == admitted_repo)
        .unwrap();
    reject(
        docs.clone(),
        "CONTRACTS",
        &format!("/repositories/{contract_index}/source_revision"),
        json!("wrong"),
    );
    for (doc, pointer, value) in [
        ("ADMISSION", "/repositories/0/head", json!("wrong")),
        (
            "ADMISSION",
            "/repositories/0/operational_authority",
            json!(false),
        ),
        ("ADMISSION", "/repositories/0/read_only", json!(false)),
        (
            "DELIVERY",
            "/repositories/0/publication_receipt/authenticated",
            json!(false),
        ),
        (
            "DELIVERY",
            "/repositories/0/publication_receipt/issue",
            json!(0),
        ),
        (
            "PACKAGE",
            "/lifecycle",
            json!(["SIP", "STP", "SPP", "SRP", "VPP", "SOR"]),
        ),
        (
            "RETAINED_ACCEPTANCE",
            "/repositories/0/proof/object_digest_verified",
            json!(false),
        ),
        (
            "RETAINED_ACCEPTANCE",
            "/repositories/0/proof/head",
            json!("wrong"),
        ),
        (
            "RETAINED_ACCEPTANCE",
            "/repositories/0/preparation/package_artifact",
            json!("unbound"),
        ),
        (
            "RETAINED_ACCEPTANCE",
            "/repositories/0/current_admission/performed_mutation",
            json!(true),
        ),
        (
            "RETAINED_ACCEPTANCE",
            "/repositories/0/cleanup/status",
            json!("invented"),
        ),
        ("STATUS", "/repositories/0/acceptance", json!("incomplete")),
        ("STATUS", "/all_requirements_satisfied", json!("true")),
    ] {
        reject(docs.clone(), doc, pointer, value);
    }
}
#[test]
fn missing_documents_duplicate_repositories_and_missing_identity_fail_closed() {
    let original = documents();
    for name in [
        "BASELINE",
        "STATUS",
        "CONTRACTS",
        "ADMISSION",
        "DELIVERY",
        "PACKAGE",
        "RETAINED_ACCEPTANCE",
        "PACKAGE_BINDINGS",
    ] {
        let mut docs = original.clone();
        docs.remove(name);
        assert!(verify(&docs).is_err(), "accepted missing {name}");
    }
    for name in [
        "BASELINE",
        "STATUS",
        "CONTRACTS",
        "ADMISSION",
        "DELIVERY",
        "RETAINED_ACCEPTANCE",
        "PACKAGE_BINDINGS",
    ] {
        let mut docs = original.clone();
        let rows = docs.get_mut(name).unwrap()["repositories"]
            .as_array_mut()
            .unwrap();
        rows.push(rows[0].clone());
        assert!(verify(&docs).is_err(), "accepted duplicate {name}");
    }
    let mut docs = original.clone();
    for name in ["ADMISSION", "DELIVERY"] {
        docs.get_mut(name).unwrap()["repositories"][0]
            .as_object_mut()
            .unwrap()
            .remove("head");
    }
    docs.get_mut("DELIVERY").unwrap()["repositories"][0]["publication_receipt"]
        .as_object_mut()
        .unwrap()
        .remove("expected_head_sha");
    assert!(
        verify(&docs).is_err(),
        "missing identities must not compare equal"
    );
}
#[test]
fn incomplete_report_can_remain_truthfully_incomplete() {
    let mut docs = documents();
    docs.get_mut("STATUS").unwrap()["all_requirements_satisfied"] = json!(false);
    docs.get_mut("STATUS").unwrap()["repositories"][0]["acceptance"] = json!("incomplete");
    verify(&docs).unwrap();
}
