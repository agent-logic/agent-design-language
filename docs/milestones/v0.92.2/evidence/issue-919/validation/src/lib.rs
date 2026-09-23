//! Issue-local adapter for the existing internal-review packet validator.
#[cfg(test)]
mod tests {
    use std::{path::Path, process::Command};

    #[test]
    fn review_contract_and_negative_fixtures() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../..");
        let result = Command::new("python3")
            .current_dir(root)
            .args([
                "docs/milestones/v0.92.2/evidence/issue-919/validate_review.py",
                "--self-test",
            ])
            .output()
            .expect("start internal-review packet validator");
        assert!(
            result.status.success(),
            "review proof failed:\n{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        let stdout = String::from_utf8_lossy(&result.stdout);
        assert!(stdout.contains("\"status\": \"pass\""));
        assert!(stdout.contains("\"negative_fixtures\": 20"));
        assert!(stdout.contains("\"accepted_inputs\": 0"));
    }
}
