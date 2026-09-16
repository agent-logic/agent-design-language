use adl::codefriend::ingestion::{
    github::{self, Input, Transport},
    Scope,
};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, io::Read, path::Path};
pub(super) const USAGE: &str = "       adl codefriend ingest github --repository <https://github.com/owner/repo> <--revision full-sha1 | --pr number> --scope <scope.json> --out <new-directory> [--fixture-api http://127.0.0.1:port]";
pub(super) fn run(args: &[String]) -> Result<()> {
    let mut flags = BTreeMap::new();
    for pair in args.chunks(2) {
        ensure!(
            pair.len() == 2
                && [
                    "--repository",
                    "--revision",
                    "--pr",
                    "--scope",
                    "--out",
                    "--fixture-api"
                ]
                .contains(&pair[0].as_str())
                && !pair[1].starts_with("--"),
            "invalid_github_ingestion_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_github_ingestion_argument"
        );
    }
    ensure!(
        ["--repository", "--scope", "--out"]
            .iter()
            .all(|k| flags.contains_key(k))
            && flags.contains_key("--revision") != flags.contains_key("--pr"),
        "missing_or_conflicting_github_ingestion_arguments"
    );
    let input = if let Some(revision) = flags.get("--revision") {
        Input::Commit((*revision).into())
    } else {
        Input::PullRequest(
            flags["--pr"]
                .parse()
                .map_err(|_| anyhow::anyhow!("invalid_pull_request_number"))?,
        )
    };
    let mut bytes = Vec::new();
    std::fs::File::open(flags["--scope"])
        .map_err(|_| anyhow::anyhow!("scope_open_failed"))?
        .take(128 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| anyhow::anyhow!("scope_read_failed"))?;
    ensure!(bytes.len() <= 128 * 1024, "scope_byte_limit_exceeded");
    let scope: Scope =
        serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_scope_json"))?;
    scope.validate()?;
    // Controlled loopback proof never resolves or forwards real credentials.
    let mut transport = if let Some(base) = flags.get("--fixture-api") {
        Transport::fixture(base)?
    } else {
        let token = super::super::github_token::resolve_github_token()?;
        Transport::github(token.as_ref().map(|t| t.value()))?
    };
    let result = github::acquire(&mut transport, flags["--repository"], input, scope)?;
    result.write(Path::new(flags["--out"]))?;
    println!(
        "{}",
        serde_json::to_string(
            &serde_json::json!({"schema":"codefriend.acquisition_result.v1","packet_id":result.packet.packet_id,"revision":result.packet.revision,"objects":result.packet.objects.len(),"completeness":result.packet.completeness,"review_state":result.packet.review_state,"provenance":result.provenance})
        )?
    );
    Ok(())
}
