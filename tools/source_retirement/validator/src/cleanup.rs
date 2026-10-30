use crate::*;
use std::time::{Duration, Instant};
/// The historical cleanup aggregate, with native public and integrity contracts.
/// Each child retains the original 60-second bound; failed children stop the run.
pub fn run(repo: &Repo) -> Result<()> {
    let checks: &[(&str, &[&str])] = &[
        (
            "cargo",
            &[
                "test",
                "--locked",
                "--manifest-path",
                "tools/public_contracts/Cargo.toml",
                "--test",
                "public_helpers",
            ],
        ),
        (
            "cargo",
            &[
                "test",
                "--locked",
                "--manifest-path",
                "tools/source_retirement/validator/Cargo.toml",
                "--test",
                "public_boundary",
            ],
        ),
        (
            "cargo",
            &[
                "test",
                "--locked",
                "--manifest-path",
                "tools/source_retirement/validator/Cargo.toml",
                "--test",
                "contracts",
                "portable::",
            ],
        ),
        (
            "cargo",
            &[
                "test",
                "--locked",
                "--manifest-path",
                "tools/ci_contracts/Cargo.toml",
                "--test",
                "workflow",
            ],
        ),
        (
            "bash",
            &["adl/tools/test_batched_checks_no_codexpr_usage_banner.sh"],
        ),
        (
            "cargo",
            &[
                "run",
                "--locked",
                "--quiet",
                "--manifest-path",
                "tools/ci_contracts/Cargo.toml",
                "--",
                "aggregation-contract",
            ],
        ),
    ];
    for (program, args) in checks {
        let mut child = Command::new(program)
            .args(*args)
            .current_dir(&repo.root)
            .spawn()?;
        let start = Instant::now();
        loop {
            if let Some(status) = child.try_wait()? {
                ensure!(status.success(), "cleanup child failed: {program} {args:?}");
                break;
            }
            if start.elapsed() >= Duration::from_secs(60) {
                child.kill()?;
                child.wait()?;
                bail!("cleanup child timed out: {program} {args:?}");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    println!("POST_SPLIT_CLEANUP_CONTRACTS_PASSED");
    Ok(())
}
