//! Native-proof bridge for the issue #920 preparation validator.

#[cfg(test)]
mod tests {
    use std::{path::Path, process::Command};

    #[test]
    fn preparation_packet_is_truthful_and_not_a_completed_external_review() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
        let result = Command::new("python3")
            .current_dir(root)
            .arg(".csdlc/evidence/920/validate_external_review_packet.py")
            .output()
            .expect("start issue #920 packet validator");
        assert!(
            result.status.success(),
            "packet validation failed:\n{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        let stdout = String::from_utf8(result.stdout).expect("validator stdout is UTF-8");
        assert!(stdout.contains("\"status\": \"pass\""));
        assert!(stdout.contains("\"packet_status\": \"preparation_only\""));
        assert!(stdout.contains("\"external_review_complete\": false"));
    }
}
