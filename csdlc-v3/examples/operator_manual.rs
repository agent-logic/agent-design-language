//! Optional read-only documentation helper, not a lifecycle command owner.
use std::{env, fs, path::Path, process::Command};

use csdlc_v3::commands::{
    local::WorktreeRegistration,
    remote::{
        canonical_authority_selector_digest, typed_review_receipt_payload_digest,
        TypedReviewReceipt,
    },
};

fn run(args: &[String]) -> Result<String, String> {
    let [action, path] = args else {
        return Err(
            "usage: operator_manual <file-digest|review-digest|selector-digest|registrations> PATH"
                .into(),
        );
    };
    match action.as_str() {
        "file-digest" => fs::read(path)
            .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
            .map_err(|_| "artifact unreadable".into()),
        "review-digest" => {
            let bytes = fs::read(path).map_err(|_| "review receipt unreadable")?;
            let receipt: TypedReviewReceipt =
                serde_json::from_slice(&bytes).map_err(|_| "review receipt schema invalid")?;
            Ok(typed_review_receipt_payload_digest(&receipt))
        }
        "selector-digest" => {
            canonical_authority_selector_digest(Path::new(path)).map_err(|finding| finding.code)
        }
        "registrations" => {
            let output = Command::new("git")
                .args(["-C", path, "worktree", "list", "--porcelain", "-z"])
                .env("GIT_OPTIONAL_LOCKS", "0")
                .output()
                .map_err(|_| "git unavailable")?;
            if !output.status.success() {
                return Err("Git registration observation failed".into());
            }
            let text = String::from_utf8(output.stdout).map_err(|_| "non-UTF-8 registration")?;
            let mut registrations = Vec::new();
            let mut worktree = None;
            let mut branch = None;
            let mut record_number = 0;
            for token in text.split('\0') {
                if let Some(value) = token.strip_prefix("worktree ") {
                    worktree = Some(value.to_owned());
                } else if let Some(value) = token.strip_prefix("branch refs/heads/") {
                    branch = Some(value.to_owned());
                } else if token.is_empty() {
                    if let Some(worktree) = worktree.take() {
                        if let Some(branch) = branch.take() {
                            registrations.push(WorktreeRegistration {
                                branch,
                                worktree,
                                primary: record_number == 0,
                            });
                        }
                        record_number += 1;
                    }
                }
            }
            serde_json::to_string_pretty(&registrations)
                .map_err(|_| "registration encoding failed".into())
        }
        _ => Err("unsupported helper action".into()),
    }
}

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("operator_manual: {error}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PVF: deterministic local tooling, fixture parsing and Git readback; no network.
    #[test]
    fn helper_digests_use_native_contracts() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let path = root.join("docs/csdlc-v3/man/examples/typed-review.json");
        let bytes = fs::read(&path).unwrap();
        let receipt: TypedReviewReceipt = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            run(&["review-digest".into(), path.display().to_string()]).unwrap(),
            typed_review_receipt_payload_digest(&receipt)
        );
        assert_eq!(
            run(&["file-digest".into(), path.display().to_string()]).unwrap(),
            blake3::hash(&bytes).to_hex().to_string()
        );
        assert!(run(&["unknown".into(), path.display().to_string()]).is_err());
    }

    #[test]
    fn registration_helper_observes_invoking_checkout_without_claiming_it_is_primary() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .canonicalize()
            .unwrap();
        let output = run(&["registrations".into(), root.display().to_string()]).unwrap();
        let rows: Vec<WorktreeRegistration> = serde_json::from_str(&output).unwrap();
        let current = rows
            .iter()
            .find(|row| Path::new(&row.worktree) == root)
            .unwrap();
        assert_eq!(current.primary, root.join(".git").is_dir());
        assert_eq!(rows.iter().filter(|row| row.primary).count(), 1);
        assert!(!current.branch.starts_with("refs/heads/"));
    }
}
