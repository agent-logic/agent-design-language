use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct Terminal {
    schema: String,
    issue: u64,
    repository: String,
    initialization_digest: String,
    canonical_generation: u64,
    canonical_digest: String,
    pull_request: Option<u64>,
    disposition: String,
    head_sha: Option<String>,
    merge_sha: Option<String>,
    issue_state: String,
    pr_state: Option<String>,
    approved_reason: Option<String>,
    observed_unix_seconds: u64,
    mutable_fresh_until_unix_seconds: Option<u64>,
    source: String,
    digest: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct RecordlessReceipt {
    schema: String,
    issue: u64,
    repository: String,
    receipt_ref: String,
    terminal: Terminal,
    actor: String,
    approved_reason: String,
    source_projection_at_pr_head: bool,
    local_projection_present: bool,
    existing_closeout_receipt_present: bool,
    digest: String,
}

fn digest<T: Clone + Serialize>(value: &T) -> String {
    blake3::hash(&serde_json::to_vec(value).expect("serialize canonical value"))
        .to_hex()
        .to_string()
}

fn validate_terminal(mut terminal: Terminal) -> Result<Terminal, String> {
    let claimed = terminal.digest.clone();
    terminal.digest.clear();
    if claimed != digest(&terminal) {
        return Err("terminal self-digest mismatch".into());
    }
    terminal.digest = claimed;
    if terminal.schema != "csdlc.derived_terminal.v1"
        || terminal.issue == 0
        || !terminal.repository.contains('/')
        || terminal.observed_unix_seconds == 0
        || terminal.disposition != "merged"
        || terminal.issue_state != "closed_by_merged_pr"
        || terminal.pr_state.as_deref() != Some("closed")
    {
        return Err("terminal semantic identity invalid".into());
    }
    Ok(terminal)
}

fn main() {
    let path = std::env::args().nth(1).expect("terminal path argument");
    let bytes = std::fs::read(&path).expect("read terminal file");
    let value: serde_json::Value = serde_json::from_slice(&bytes).expect("parse terminal JSON");
    let result = (|| -> Result<(&str, Terminal, Option<String>), String> { if value.get("schema").and_then(|v| v.as_str())
        == Some("csdlc.recordless_terminal_receipt.v1")
    {
        let mut receipt: RecordlessReceipt = serde_json::from_value(value).expect("parse recordless receipt");
        let claimed = receipt.digest.clone();
        receipt.digest.clear();
        if claimed != digest(&receipt) {
            Err("recordless receipt self-digest mismatch".to_string())
        } else {
            receipt.digest = claimed;
            let terminal = validate_terminal(receipt.terminal.clone())?;
            if receipt.issue != terminal.issue
                || receipt.repository != terminal.repository
                || receipt.receipt_ref != format!("csdlc-v2/closeout/{}.json", receipt.issue)
                || receipt.source_projection_at_pr_head
                || receipt.local_projection_present
                || receipt.existing_closeout_receipt_present
            {
                Err("recordless receipt identity invalid".to_string())
            } else {
                Ok(("recordless_closeout", terminal, Some(receipt.digest)))
            }
        }
    } else {
        let terminal: Terminal = serde_json::from_value(value).expect("parse derived terminal");
        validate_terminal(terminal).map(|terminal| ("derived_terminal", terminal, None))
    } })();

    match result {
        Ok((kind, terminal, receipt_digest)) => println!(
            "{}",
            serde_json::json!({
                "valid": true,
                "kind": kind,
                "issue": terminal.issue,
                "repository": terminal.repository,
                "pull_request": terminal.pull_request,
                "head_sha": terminal.head_sha,
                "merge_sha": terminal.merge_sha,
                "terminal_digest": terminal.digest,
                "receipt_digest": receipt_digest
            })
        ),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
