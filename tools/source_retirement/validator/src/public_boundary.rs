//! PVF local_contract: current committed-tree boundary, small deterministic Git reads.
//! No archived commits, credentials, sibling repositories, or network access.
use crate::*;

pub fn validate(tree: &Tree, policy: &Value, manifests: &BTreeMap<String, Vec<u8>>) -> Result<()> {
    ensure!(
        policy["schema"] == "adl.public_history_boundary.v1",
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
        let _: Value = serde_json::from_slice(raw)?;
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
const POLICY_SHA256: &str = "c1b256b34932bbb26ba1afe03316eaa3c04a7dba6b97722ef4a511b3d2b6a63d";
