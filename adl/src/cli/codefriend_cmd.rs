use adl::codefriend::ingestion::{local, AdmissionInput, Scope};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, path::Path};
const USAGE: &str = "Usage: adl codefriend ingest local --checkout <directory> --repository <https://host/owner/repo> --revision <full-commit-id> --scope <scope.json> --out <new-packet.json>\n       adl codefriend packet read --input <packet.json>";
pub(super) fn real_codefriend(args: &[String]) -> Result<()> {
    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        println!("{USAGE}");
        return Ok(());
    }
    ensure!(args.len() >= 2, "{USAGE}");
    let read = args[0] == "packet" && args[1] == "read";
    ensure!(
        read || (args[0] == "ingest" && args[1] == "local"),
        "unsupported_codefriend_command"
    );
    let expected: &[&str] = if read {
        &["--input"]
    } else {
        &[
            "--checkout",
            "--repository",
            "--revision",
            "--scope",
            "--out",
        ]
    };
    let mut flags = BTreeMap::new();
    for pair in args[2..].chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_codefriend_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_codefriend_argument"
        );
    }
    ensure!(flags.len() == expected.len(), "missing_codefriend_argument");
    let packet = if read {
        AdmissionInput::read(Path::new(flags["--input"]))?
            .packet()
            .clone()
    } else {
        use std::io::Read;
        let mut bytes = Vec::new();
        std::fs::File::open(flags["--scope"])
            .map_err(|_| anyhow::anyhow!("scope_open_failed"))?
            .take(128 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("scope_read_failed"))?;
        ensure!(bytes.len() <= 128 * 1024, "scope_byte_limit_exceeded");
        let scope: Scope =
            serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_scope_json"))?;
        let root = Path::new(flags["--checkout"]);
        let packet = local::acquire(root, flags["--repository"], flags["--revision"], scope)?;
        local::write_packet(root, Path::new(flags["--out"]), &packet)?;
        // Actual production reader validates the just-published acquisition artifact.
        AdmissionInput::read(Path::new(flags["--out"]))?
            .packet()
            .clone()
    };
    println!(
        "{}",
        serde_json::to_string(
            &serde_json::json!({"schema":"codefriend.acquisition_result.v1","packet_id":packet.packet_id,"revision":packet.revision,"objects":packet.objects.len(),"completeness":packet.completeness,"review_state":packet.review_state})
        )?
    );
    Ok(())
}
