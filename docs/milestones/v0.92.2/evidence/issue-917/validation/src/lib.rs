//! Issue-local adapter for the existing documentation validators.
#[cfg(test)]
mod tests {
    use std::{path::Path, process::Command};

    fn check(script: &str, kind: &str) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../..");
        let result = Command::new("python3")
            .current_dir(root)
            .args(["-c", include_str!("../check.py"), script, kind])
            .output()
            .expect("start existing documentation validator");
        assert!(result.status.success(), "documentation proof failed:\n{}\n{}",
            String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
    }

    #[test]
    fn handoff_contract_and_negative_fixtures() {
        check("docs/milestones/v0.92.2/evidence/issue-917/validate_handoff.py", "handoff");
    }

    #[test]
    fn planning_contract_and_negative_fixtures() {
        check("docs/milestones/v0.92.2/validate_planning.py", "planning");
    }
}
