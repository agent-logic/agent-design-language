use anyhow::{Context, Result};
use std::path::PathBuf;

fn resolve_out_path(arg: Option<String>) -> PathBuf {
    arg.map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(
            adl::dspark_speculative_decoding_evaluation::DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH,
        )
    })
}

fn write_report(path: &PathBuf) -> Result<()> {
    adl::dspark_speculative_decoding_evaluation::write_dspark_speculative_decoding_evaluation_report(path)
        .with_context(|| {
            format!(
                "write DSpark speculative decoding evaluation report '{}'",
                path.display()
            )
        })?;
    Ok(())
}

fn main() -> Result<()> {
    let out_path = resolve_out_path(std::env::args().nth(1));
    write_report(&out_path)?;
    println!("{}", out_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{resolve_out_path, write_report};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn demo_v0917_dspark_speculative_decoding_resolve_out_path_uses_explicit_argument() {
        let path = resolve_out_path(Some(
            "tmp/dspark-speculative-decoding-evaluation.json".to_string(),
        ));
        assert_eq!(
            path,
            std::path::PathBuf::from("tmp/dspark-speculative-decoding-evaluation.json")
        );
    }

    #[test]
    fn demo_v0917_dspark_speculative_decoding_write_report_creates_expected_json() {
        let path = unique_temp_path("dspark-speculative-decoding-evaluation-bin");
        write_report(&path).expect("write report");
        let body = fs::read_to_string(&path).expect("read report");
        assert!(body.contains("dspark_speculative_decoding_evaluation.v1"));
        fs::remove_file(&path).expect("remove report");
    }
}
