//! PVF local_contract: current committed-tree boundary, small deterministic Git reads.
//! No archived commits, credentials, sibling repositories, or network access.
use crate::*;

pub fn validate(tree: &Tree, policy: &Value, manifests: &BTreeMap<String, Vec<u8>>) -> Result<()> {
    ensure!(
        matches!(
            policy["schema"].as_str(),
            Some("adl.public_history_boundary.v1" | "adl.public_history_boundary.v2")
        ),
        "boundary schema"
    );
    for key in ["retired_paths", "required_paths", "forbidden_prefixes"] {
        let entries = rows(&policy[key])?;
        ensure!(!entries.is_empty(), "empty boundary: {key}");
        let set = strings(&policy[key])?;
        ensure!(set.len() == entries.len(), "duplicate boundary: {key}");
        for path in &set {
            ensure!(
                !path.starts_with('/') && !path.split('/').any(|p| p == ".."),
                "unsafe boundary path"
            );
            if key == "retired_paths" {
                ensure!(!tree.contains_key(path), "retired path restored: {path}");
            } else if key == "required_paths" {
                ensure!(
                    tree.contains_key(path),
                    "required public path missing: {path}"
                );
            } else {
                ensure!(
                    !tree.keys().any(|p| p.starts_with(path))
                        && !tree.contains_key(path.trim_end_matches('/')),
                    "retired prefix restored: {path}"
                );
            }
        }
    }
    ensure!(
        !tree.keys().any(|p| p.ends_with(".rb")),
        "retired Ruby source restored"
    );
    ensure!(
        !tree
            .keys()
            .any(|p| p.starts_with("docs/milestones/") && p.ends_with(".py")),
        "historical Python source restored"
    );
    let pins = policy["manifest_sha256"]
        .as_object()
        .context("manifest pins")?;
    ensure!(
        !pins.is_empty() && manifests.len() == pins.len(),
        "manifest scope mismatch"
    );
    for (path, pin) in pins {
        let expected = string(pin)?;
        ensure!(
            expected.len() == 64 && expected.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid manifest hash"
        );
        let raw = manifests.get(path).context("manifest missing")?;
        ensure!(sha(raw) == expected, "provenance manifest changed: {path}");
        if policy["schema"] == "adl.public_history_boundary.v2"
            && policy["archive_catalog"].as_str() == Some(path.as_str())
        {
            validate_archive_catalog(tree, policy, raw)?;
        } else {
            let _: Value = serde_json::from_slice(raw)?;
        }
    }
    if policy["schema"] == "adl.public_history_boundary.v2" {
        let catalog = policy["archive_catalog"]
            .as_str()
            .context("archive catalog path")?;
        ensure!(pins.contains_key(catalog), "archive catalog is not pinned");
    }
    Ok(())
}

// Current-tree retirement enforcement remains independent from historical audit
// custody. Public CI checks the reviewed catalog and every original digest;
// archive retrieval/restore is proved separately, without private CI access.
fn validate_archive_catalog(tree: &Tree, policy: &Value, raw: &[u8]) -> Result<()> {
    use std::io::Read;
    let mut decoded = Vec::new();
    flate2::read::GzDecoder::new(raw)
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut decoded)?;
    ensure!(
        decoded.len() <= 16 * 1024 * 1024,
        "archive catalog size limit"
    );
    let catalog: Value = serde_json::from_slice(&decoded)?;
    ensure!(
        catalog["schema"] == "adl.historical_file_catalog.v1",
        "archive catalog schema"
    );
    let entries = catalog["files"]
        .as_object()
        .context("archive catalog files")?;
    let archived = policy["archived_manifest_sha256"]
        .as_object()
        .context("archived manifest pins")?;
    ensure!(!archived.is_empty(), "empty archived manifest pins");
    for (path, pin) in archived {
        ensure!(
            path.starts_with("docs/milestones/") && !path.split('/').any(|p| p == ".."),
            "unsafe archived manifest path"
        );
        ensure!(
            !tree.contains_key(path),
            "archived manifest restored without reviewed transition: {path}"
        );
        let entry = entries
            .get(path)
            .context("archived manifest absent from catalog")?;
        let digest = string(pin)?;
        ensure!(
            digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid archived manifest hash"
        );
        ensure!(
            entry["sha256"] == *pin,
            "archived manifest digest changed: {path}"
        );
        let blob = entry["blob"].as_str().context("archived Git blob")?;
        ensure!(
            blob.len() == 40 && blob.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid archived Git blob"
        );
        ensure!(
            matches!(entry["mode"].as_str(), Some("100644" | "100755")),
            "invalid archived file mode"
        );
        ensure!(
            entry["bytes"].as_u64().is_some(),
            "missing archived file size"
        );
    }
    Ok(())
}

pub fn run(repo: &Repo) -> Result<()> {
    let raw = repo.bytes("tools/source_retirement/validator/public-boundary.json")?;
    ensure!(
        sha(&raw) == POLICY_SHA256,
        "public boundary policy changed without reviewed pin"
    );
    let policy: Value = serde_json::from_slice(&raw)?;
    let manifests = policy["manifest_sha256"]
        .as_object()
        .context("manifest pins")?
        .keys()
        .map(|p| Ok((p.clone(), repo.bytes(p)?)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    validate(&repo.tree("HEAD")?, &policy, &manifests)
}

// Updated only with an independently reviewed policy amendment.
const POLICY_SHA256: &str = "594b62f35b190d0310ea25a09c64fef7e12b84665d5b0e1ebb7a760c879d12c8";
