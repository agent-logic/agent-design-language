//! PVF focused_contract: deterministic local documentation/identity consistency.
//! Ordinary docs gate only; no historical-object fetch, product qualification or release authority.
use serde_json::Value;
use std::path::{Component, PathBuf};

#[test]
fn milestone_records_preserve_reviewed_sources_and_acceptance_boundaries() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let value: Value = serde_json::from_slice(
        &std::fs::read(root.join("docs/releases/milestones/REGISTRY.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(value["schema"], "adl.milestone_release_registry.v1");
    assert_eq!(value["canonical_cutover"], "v0.92.0");
    assert_eq!(
        value["legacy_repository"],
        "danielbaustin/agent-design-language"
    );
    assert_eq!(value["legacy_releases"], "unchanged");
    let records = value["records"].as_array().unwrap();
    assert_eq!(records.len(), 4);
    for (record, (version, source, tree, package, outcome)) in records.iter().zip([
        (
            "v0.92.0",
            "f0a011a5c59d46c763d669f69a10308b3f870ba4",
            "e36b23606b9cf3f0a8eca8e3dcae9e0c201306f6",
            "0.92.0",
            "engineering_complete",
        ),
        (
            "v0.92.1",
            "c9cdb2f13ff77a06dcc79ad4e4c332f444b2e14f",
            "653f87a7601466c8a4f52c0433ae0cc4c9650b00",
            "0.92.1",
            "published_software_release",
        ),
        (
            "v0.92.2",
            "248f00e359e412f6bc0061ac88be9ff374e3a212",
            "4c794391969133fdb39e48dfcf08067175e2ab4c",
            "0.92.1",
            "administrative_close_with_deferred_qualification",
        ),
        (
            "v0.93.1",
            "69fae86cde65861cdace95183c2c373dcb09ca01",
            "cd3f06cf3b067bb2fbccde030d0a7602582a2822",
            "0.92.1",
            "ready_to_deploy_undeployed",
        ),
    ]) {
        assert_eq!(record["milestone"], version);
        assert_eq!(record["original_source_commit"], source);
        assert_eq!(record["original_source_tree"], tree);
        assert_eq!(record["package_version"], package);
        assert_eq!(record["outcome"], outcome);
        assert_eq!(record["deployment_claimed"], false);
        assert_eq!(
            record["historical_source_archive"],
            "agent-design-language/1382/approved-cli-20261027-74976e8b"
        );
        let path = PathBuf::from(record["notes"].as_str().unwrap());
        assert!(path.components().all(|p| matches!(p, Component::Normal(_))));
        let notes = std::fs::read_to_string(root.join(path)).unwrap();
        assert!(notes.contains(source));
        assert!(notes.contains(tree));
        if version == "v0.92.1" {
            assert_eq!(
                record["public_tag_commit"],
                "12a6c9f971284b42ebdcd588eef1499c7717cf4d"
            );
            assert_eq!(record["release_id"], 387308409_u64);
        } else {
            assert_eq!(record["release_role"], "historical_milestone_record");
            assert!(notes.contains("not the original milestone source"));
            assert!(notes.contains("No binary or registry release is claimed"));
        }
        if version == "v0.92.2" {
            assert!(notes.contains("external review attempt failed"));
            assert!(notes.contains("qualification incomplete/deferred"));
        }
        if version == "v0.93.1" {
            assert!(notes.contains("ready to deploy, undeployed"));
            assert_eq!(
                record["qualified_source_commit"],
                "ac2b596915538f3ed9ce418aec5aa7b917c18fd4"
            );
        }
        assert_eq!(
            record["approved_open_backlog_carryovers"],
            serde_json::json!([84, 251, 269, 1226])
        );
    }
    assert_eq!(
        value["active_or_planned_not_complete"],
        serde_json::json!(["v0.93", "v0.93.2", "v0.93.3"])
    );
}
