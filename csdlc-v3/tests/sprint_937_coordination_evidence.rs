use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("csdlc-v3 package must live under the repository root")
        .to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn json(path: &Path) -> Value {
    serde_json::from_str(&read(path))
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
}

#[test]
fn sprint_937_coordination_packet_preserves_roster_owners_and_gates() {
    let root = repo_root();
    let evidence = root.join(".csdlc/evidence/937");
    let packet = read(&evidence.join("SPRINT_EXECUTION_PACKET.md"));

    for section in [
        "## Child Issue Wave",
        "## Recommended Execution Order",
        "## Watcher Policy",
        "## Budget And Goal Accounting",
        "## Watcher Plan",
        "## Safe Parallel Lanes",
        "## Candidate Parallel Lanes",
        "## Serial Gates",
        "## Parallelism Outcome Plan",
        "## Sprint Closeout Rollup Expectations",
        "## Non-Claims",
    ] {
        assert!(
            packet.contains(section),
            "missing packet section: {section}"
        );
    }

    let wave = packet
        .split_once("## Child Issue Wave")
        .expect("child wave section")
        .1
        .split_once("## Dependency Graph")
        .expect("dependency graph section")
        .0;
    let rows: Vec<Vec<&str>> = wave
        .lines()
        .filter(|line| line.starts_with("| #"))
        .map(|line| line.trim_matches('|').split('|').map(str::trim).collect())
        .collect();
    let observed: Vec<u64> = rows
        .iter()
        .map(|columns| {
            columns[0]
                .trim_start_matches('#')
                .parse()
                .expect("numeric issue")
        })
        .collect();
    assert_eq!(observed, (916..=925).collect::<Vec<_>>());
    assert_eq!(rows[0][6], "Planning #11; Worker #9 read-only audit");
    assert_eq!(rows[1][6], "Planning #4.5");
    assert_eq!(rows[2][6], "Worker #10");
    assert_eq!(rows[6][6], "Planning #11");
    assert!(packet.contains("- Owner: `Planning #5` for umbrella coordination"));
    assert!(packet.contains("Sprint 10 qualification did not pass"));
    assert!(packet.contains(
        "This packet does not authorize v0.93 execution, Beta 1 launch, release, provider calls, product repair, deployment, publication, or merge."
    ));

    let state = json(&evidence.join("sprint-state.json"));
    assert_eq!(state["sprint_issue_number"], 937);
    let ordered: Vec<u64> = state["ordered_issue_numbers"]
        .as_array()
        .expect("ordered issue numbers")
        .iter()
        .map(|issue| issue.as_u64().expect("numeric ordered issue"))
        .collect();
    assert_eq!(ordered, (916..=925).collect::<Vec<_>>());
    let records: Vec<u64> = state["issue_records"]
        .as_array()
        .expect("issue records")
        .iter()
        .map(|record| record["issue_number"].as_u64().expect("record issue"))
        .collect();
    assert_eq!(records, ordered);

    let events: Vec<Value> = read(&evidence.join("activity.jsonl"))
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("valid activity event"))
        .collect();
    assert!(!events.is_empty());
    assert!(events.iter().all(|event| event["sprint_issue"] == 937));
    let event = |kind: &str| {
        events
            .iter()
            .find(|event| event["event"] == kind)
            .unwrap_or_else(|| panic!("missing activity event: {kind}"))
    };
    let wave_event = event("coordination_wave_recorded");
    assert_eq!(wave_event["actor"], "Planning #5");
    let details = &wave_event["details"];
    assert_eq!(
        details["active_decision_owner"],
        serde_json::json!({"issue": 916, "owner": "Planning #11"})
    );
    assert_eq!(
        details["read_only_support"],
        serde_json::json!({"issue": 916, "owner": "Worker #9"})
    );
    assert_eq!(
        details["preparation_lanes"],
        serde_json::json!([
            {"issue": 917, "owner": "Planning #4.5"},
            {"issue": 918, "owner": "Worker #10"},
            {"issue": 922, "owner": "Planning #11"}
        ])
    );
    assert_eq!(details["sprint10_disposition"]["issue"], 915);
    assert_eq!(details["sprint10_disposition"]["owner"], "Planning #7.3");
    assert_eq!(
        details["sprint10_closeout"],
        serde_json::json!({"issue": 936, "owner": "Planning #11"})
    );
    assert_eq!(details["successors"], serde_json::json!([1148, 1149, 1150]));
    assert!(details["acceptance_rule"]
        .as_str()
        .expect("acceptance rule")
        .contains("preparation overlap grants no early acceptance"));

    let closed = &event("sprint10_disposition_closed")["details"];
    assert_eq!(closed["issue_915"]["state"], "closed");
    assert_eq!(closed["issue_915"]["disposition"], "NOT_PLANNED");
    assert_eq!(closed["issue_936"]["state"], "closed");
    assert_eq!(closed["issue_936"]["disposition"], "NOT_PLANNED");
    assert_eq!(closed["successors"], serde_json::json!([1148, 1149, 1150]));
    assert_eq!(
        closed["qualification_claim"],
        "incomplete_deferred_not_pass"
    );

    let spp = json(&root.join(".csdlc/v3/issues/937/cards/spp.values.json"));
    let dependencies = spp["dependencies_inline"]
        .as_str()
        .expect("SPP dependencies");
    for phrase in [
        "#916 remains not_proven",
        "#916 through #925",
        "#910 and #911",
        "Final acceptance remains strictly serial",
    ] {
        assert!(
            dependencies.contains(phrase),
            "missing dependency gate: {phrase}"
        );
    }
    assert_eq!(spp["step_3_status"], "in_progress");
    assert_eq!(spp["step_4_status"], "pending");
    assert_eq!(spp["step_5_status"], "pending");
    let acceptance = spp["acceptance_criteria_inline"]
        .as_str()
        .expect("SPP acceptance");
    for phrase in [
        "#917, #918, and #922 preparation does not become early acceptance",
        "no product repair",
        "provider call",
        "deployment",
        "publication",
        "merge",
        "release",
        "public launch",
    ] {
        assert!(
            acceptance.contains(phrase),
            "missing acceptance boundary: {phrase}"
        );
    }
}
