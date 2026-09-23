//! Issue-local adapter; success means a valid private draft, never release approval.
#[cfg(test)]
mod tests {
    use std::{path::Path, process::Command};

    fn check(mode: &str) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../..");
        let result = Command::new("python3")
            .current_dir(root)
            .args(["-c", include_str!("../check.py"), mode])
            .output()
            .expect("start publication packet validator");
        assert!(result.status.success(), "publication packet proof failed:\n{}\n{}",
            String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
    }

    #[test]
    fn publication_packet_contract() { check("default"); }

    #[test]
    fn publication_packet_negative_fixtures() { check("self-test"); }
}
