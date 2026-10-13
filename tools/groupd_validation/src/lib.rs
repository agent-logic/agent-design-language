//! PVF: deterministic local contract regression adapter for issue #1160.
//! Executes real shell/Python suites; no cloud, provider, or container execution.
#[cfg(test)]
mod tests {
    use std::{path::Path, process::Command};
    fn run(program: &str, script: &str) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let result = Command::new(program)
            .arg(script)
            .current_dir(root)
            .env("PYTHONDONTWRITEBYTECODE", "1")
            .env(
                "ADL_BUILD_ACTION_LOG_DIR",
                root.join(".csdlc/evidence/1160/adapter-build-actions"),
            )
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{script}\n{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
    }
    #[test]
    fn dependency_inventory() {
        run(
            "python3",
            "tools/groupd_validation/run_dependency_inventory.py",
        );
    }
    #[test]
    fn release_documentation() {
        run("python3", "tools/groupd_validation/test_documentation.py");
    }
    #[test]
    fn builder_inputs() {
        run("python3", "tools/groupd_validation/run_builder_contracts.py");
    }
    #[test]
    fn ci_routing() {
        run("bash", "adl/tools/test_ci_path_policy.sh");
    }
    // Bootstrap safety tests moved with the Infrastructure package (#1223).
    // Their 24 original assertions are not counted as ADL adapter executions.
    #[test]
    fn lane_selector() {
        run("python3", "tools/public_adl/test_validation_interfaces.py");
    }
    #[test]
    fn coverage_test_only() {
        run("python3", "adl/tools/test_coverage_test_only.py");
    }
}
