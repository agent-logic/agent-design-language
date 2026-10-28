//! PVF: deterministic local Git/document contract; small CPU/memory, no network.
//! Proof role: issue 1345 inventory/custody consistency, not retirement approval.
//! Gate: focused prepublication check; full delivery is separately verified in #1352.
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn inventory() -> Value {
    serde_json::from_slice(
        &std::fs::read(root().join("docs/language-cleanup/inventory.json")).unwrap(),
    )
    .unwrap()
}
fn git(args: &[&str]) -> Vec<u8> {
    let o = Command::new("git")
        .args(args)
        .current_dir(root())
        .output()
        .unwrap();
    assert!(
        o.status.success(),
        "git failed: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    o.stdout
}
fn identities(v: &Value) -> Result<BTreeMap<String, String>, String> {
    let rows = v["scripts"].as_array().ok_or("missing scripts")?;
    let mut result = BTreeMap::new();
    for row in rows {
        let p = row["path"].as_str().ok_or("missing path")?;
        let contract_id = row["replacement_contract"]
            .as_str()
            .ok_or("missing contract")?;
        let contract = &v["replacement_contracts"][contract_id];
        for key in [
            "outputs",
            "failure_cases",
            "platform_scope",
            "caller_switch",
            "sequencing",
        ] {
            if contract[key].as_str().is_none_or(|s| s.is_empty()) {
                return Err("incomplete replacement contract".into());
            }
        }
        if contract["baseline_tests"]
            .as_array()
            .is_none_or(|a| a.is_empty())
        {
            return Err("missing proving surfaces".into());
        }

        let blob = row["git_blob"].as_str().ok_or("missing blob")?;
        if !p.ends_with(".rb") && !p.ends_with(".py") {
            return Err("unexpected extension".into());
        }
        if result.insert(p.to_owned(), blob.to_owned()).is_some() {
            return Err("duplicate path".into());
        }
        if !(1346..=1351).contains(&row["delivery_issue"].as_u64().ok_or("missing owner")?) {
            return Err("unassigned delivery".into());
        }
        if row["preservation"]["commit"] != v["baseline_commit"]
            || row["preservation"]["path"] != row["path"]
        {
            return Err("custody mismatch".into());
        }
        if row["retirement_authorized"] != false
            || row["gates"].as_array().is_none_or(|x| x.is_empty())
        {
            return Err("unsupported retirement permission".into());
        }
    }
    Ok(result)
}
#[test]
fn every_baseline_script_has_exact_identity_and_custody() {
    let v = inventory();
    let rows = identities(&v).unwrap();
    let base = v["baseline_commit"].as_str().unwrap();
    let tree = git(&["ls-tree", "-rz", base]);
    let mut expected = BTreeMap::new();
    let mut all = BTreeMap::new();
    for e in tree.split(|b| *b == 0).filter(|e| !e.is_empty()) {
        let s = std::str::from_utf8(e).unwrap();
        let (meta, path) = s.split_once('\t').unwrap();
        let parts: Vec<_> = meta.split_whitespace().collect();
        all.insert(path.to_owned(), (parts[0].to_owned(), parts[2].to_owned()));
        if path.ends_with(".rb") || path.ends_with(".py") {
            expected.insert(path.to_owned(), parts[2].to_owned());
        }
    }
    assert_eq!(
        rows, expected,
        "inventory must cover exact baseline, without omissions or extras"
    );
    assert_eq!(rows.keys().filter(|p| p.ends_with(".rb")).count(), 408);
    assert_eq!(rows.keys().filter(|p| p.ends_with(".py")).count(), 234);
    let mut child = Command::new("git")
        .args(["cat-file", "--batch"])
        .current_dir(root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap());
    for row in v["scripts"].as_array().unwrap() {
        let p = row["path"].as_str().unwrap();
        let (mode, blob) = &all[p];
        assert_eq!(row["git_mode"], mode.as_str());
        writeln!(input, "{blob}").unwrap();
        input.flush().unwrap();
        let mut header = String::new();
        output.read_line(&mut header).unwrap();
        let size: usize = header.split_whitespace().nth(2).unwrap().parse().unwrap();
        let mut bytes = vec![0; size];
        output.read_exact(&mut bytes).unwrap();
        let mut newline = [0];
        output.read_exact(&mut newline).unwrap();
        assert_eq!(row["bytes"], size);
        assert_eq!(row["sha256"], format!("{:x}", Sha256::digest(&bytes)));
        assert_eq!(
            row["language"],
            if p.ends_with(".rb") { "Ruby" } else { "Python" }
        );
        for caller in row["caller_candidates"].as_array().unwrap() {
            assert!(all.contains_key(caller.as_str().unwrap()));
        }
    }
    drop(input);
    assert!(child.wait().unwrap().success());
    let mut seen = BTreeSet::new();
    for row in v["interpreter_reference_candidates"].as_array().unwrap() {
        let p = row["path"].as_str().unwrap();
        assert!(seen.insert(p));
        assert_eq!(row["git_blob"], all[p].1);
        assert_eq!(row["git_mode"], all[p].0);
        assert!(!rows.contains_key(p));
        assert!(!row["matching_lines"].as_array().unwrap().is_empty());
    }
    for contract in v["replacement_contracts"].as_object().unwrap().values() {
        for path in contract["baseline_tests"].as_array().unwrap() {
            assert!(
                all.contains_key(path.as_str().unwrap()),
                "missing contract proving surface: {path}"
            );
        }
    }
    for row in v["archives"].as_array().unwrap() {
        assert_eq!(row["git_blob"], all[row["path"].as_str().unwrap()].1);
    }
    assert_eq!(v["python_exception_budget"]["maximum_files"], 0);
    assert!(v["python_exception_budget"]["approved_paths"]
        .as_array()
        .unwrap()
        .is_empty());
}
#[test]
fn duplicate_unowned_and_invented_custody_are_rejected() {
    let v = inventory();
    let mut duplicate = v.clone();
    let first = duplicate["scripts"][0].clone();
    duplicate["scripts"].as_array_mut().unwrap().push(first);
    assert_eq!(identities(&duplicate).unwrap_err(), "duplicate path");
    let mut unowned = v.clone();
    unowned["scripts"][0]["delivery_issue"] = 0.into();
    assert_eq!(identities(&unowned).unwrap_err(), "unassigned delivery");
    let mut custody = v.clone();
    custody["scripts"][0]["preservation"]["commit"] = "fabricated".into();
    assert_eq!(identities(&custody).unwrap_err(), "custody mismatch");
    let mut missing_contract = v.clone();
    missing_contract["scripts"][0]["replacement_contract"] = "absent".into();
    assert_eq!(
        identities(&missing_contract).unwrap_err(),
        "incomplete replacement contract"
    );
    let mut permission = v;
    permission["scripts"][0]["retirement_authorized"] = true.into();
    assert_eq!(
        identities(&permission).unwrap_err(),
        "unsupported retirement permission"
    );
}
