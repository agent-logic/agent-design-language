//! Session persistence and fail-closed mutation contracts through the public CLI.
use super::*;
use anyhow::ensure;
struct Session<'a> {
    binary: &'a Path,
    root: &'a Path,
    ledger: PathBuf,
    count: usize,
    env: BTreeMap<String, String>,
}
impl Session<'_> {
    fn call(&mut self, args: &[&str], failure: Option<&str>, structured: bool) -> Result<Value> {
        let mut a = vec!["session"];
        a.extend(args);
        a.extend(["--ledger", self.ledger.to_str().context("ledger path")?]);
        if structured {
            a.push("--json");
        }
        let o = invoke(self.binary, self.root, &a, &self.env)?;
        self.count += 1;
        if let Some(message) = failure {
            ensure!(
                o.status.code() == Some(1) && o.stdout.is_empty() && text(&o).1.contains(message),
                "expected {message}: {o:?}"
            );
            return Ok(Value::Null);
        }
        ensure!(o.status.success() && o.stderr.is_empty(), "session: {o:?}");
        if structured {
            Ok(serde_json::from_slice(&o.stdout)?)
        } else {
            Ok(json!(String::from_utf8(o.stdout)?))
        }
    }
}
pub fn run(binary: &Path) -> Result<Value> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();
    let ledger = root.join("isolated/ledger.json");
    let mut e = env();
    e.insert("ADL_OBSERVABILITY".into(), "0".into());
    let mut s = Session {
        binary,
        root,
        ledger: ledger.clone(),
        count: 0,
        env: e,
    };
    ensure!(s.call(&["status"], None, true)?["active_claims"] == 0);
    ensure!(!ledger.exists());
    for command in ["", "status", "claim", "heartbeat", "release"] {
        let args = if command.is_empty() {
            vec!["--help"]
        } else {
            vec![command, "--help"]
        };
        ensure!(s
            .call(&args, None, false)?
            .as_str()
            .unwrap()
            .contains("Usage:"));
        ensure!(!ledger.exists());
    }
    let claim_args = [
        "claim",
        "--session-id",
        "fixture-session",
        "--owner",
        "fixture",
        "--resource",
        "test:portable",
        "--purpose",
        "isolated contract",
        "--ttl-secs",
        "3600",
    ];
    let claim = s.call(&claim_args, None, true)?;
    let id = claim["claim_id"].as_str().context("claim id")?.to_owned();
    ensure!(
        claim["resource"] == json!({"kind":"test","id":"portable"}) && claim["mode"] == "active"
    );
    ensure!(s.call(&["status"], None, true)?["active_claims"] == 1);
    ensure!(s
        .call(&["status"], None, false)?
        .as_str()
        .unwrap()
        .contains(&id));
    let saved = fs::read(&ledger)?;
    for (extra, message) in [
        (vec!["--unknown"], "unknown session claim option"),
        (vec!["--owner", ""], "owner cannot be empty"),
        (
            vec!["--ttl-secs", "0"],
            "ttl seconds must be greater than zero",
        ),
        (vec!["--ttl-secs", "9223372036854775807"], "ttl seconds"),
        (vec!["--ttl-secs", "9223372036854775"], "ttl seconds"),
        (vec![], "already has active claim"),
    ] {
        let mut args = claim_args.to_vec();
        args.extend(extra);
        s.call(&args, Some(message), true)?;
        ensure!(fs::read(&ledger)? == saved);
    }
    for (ttl, message) in [
        ("0", "ttl seconds must be greater than zero"),
        ("9223372036854775807", "ttl seconds"),
        ("9223372036854775", "ttl seconds"),
    ] {
        s.call(
            &["heartbeat", "--claim-id", &id, "--ttl-secs", ttl],
            Some(message),
            true,
        )?;
        ensure!(fs::read(&ledger)? == saved);
    }
    let heartbeat = s.call(
        &["heartbeat", "--claim-id", &id, "--ttl-secs", "7200"],
        None,
        true,
    )?;
    ensure!(heartbeat["claim_id"] == id && heartbeat["mode"] == "active");
    ensure!(heartbeat["expires_at"].as_str() > claim["expires_at"].as_str());
    ensure!(s
        .call(&["heartbeat", "--claim-id", &id], None, false)?
        .as_str()
        .unwrap()
        .starts_with("heartbeat "));
    let release = s.call(
        &["release", "--claim-id", &id, "--reason", "fixture complete"],
        None,
        true,
    )?;
    ensure!(release["mode"] == "released" && release["release_reason"] == "fixture complete");
    let saved = fs::read(&ledger)?;
    for (args, message) in [
        (
            vec!["heartbeat", "--claim-id", id.as_str()],
            "already released",
        ),
        (vec!["release", "--claim-id", "absent"], "not found"),
    ] {
        s.call(&args, Some(message), true)?;
        ensure!(fs::read(&ledger)? == saved);
    }
    let status = s.call(&["status"], None, true)?;
    ensure!(status["active_claims"] == 0 && status["released_claims"] == 1);
    ensure!(s
        .call(&claim_args, None, false)?
        .as_str()
        .unwrap()
        .starts_with("claimed "));
    let status = s.call(&["status"], None, true)?;
    let active = status["claims"]
        .as_array()
        .context("claims")?
        .iter()
        .find(|r| r["classification"] == "active")
        .context("active claim")?;
    ensure!(s
        .call(
            &[
                "release",
                "--claim-id",
                active["claim_id"].as_str().context("id")?
            ],
            None,
            false
        )?
        .as_str()
        .unwrap()
        .starts_with("released "));
    ensure!(
        fs::read(ledger.with_file_name("ledger.json.lock"))? == b"adl.session-ledger.os-lock.v1\n"
    );
    ensure!(!ledger.with_extension("json.tmp").exists());
    let mut malformed: Value = serde_json::from_slice(&fs::read(&ledger)?)?;
    malformed["schema"] = json!("unsupported-fixture-schema");
    fs::write(&ledger, malformed.to_string())?;
    let saved = fs::read(&ledger)?;
    s.call(&["status"], Some("unsupported session ledger schema"), true)?;
    ensure!(fs::read(&ledger)? == saved);
    fs::write(&ledger, "{invalid")?;
    s.call(&["status"], Some("parse session ledger"), true)?;
    ensure!(fs::read_to_string(&ledger)? == "{invalid");
    fs::remove_file(&ledger)?;
    fs::create_dir(&ledger)?;
    s.call(&["status"], Some("read session ledger"), true)?;
    ensure!(ledger.is_dir());
    Ok(
        json!({"status":"passed","commands":s.count,"scope":"isolated occupancy contract; no lifecycle authority"}),
    )
}
