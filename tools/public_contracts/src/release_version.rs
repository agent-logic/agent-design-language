//! Release version data checks; live CLI execution remains in the shell entrypoint.
use anyhow::{bail, Context, Result};
use serde_json::Value;

pub fn source(manifest: &str, lock: &str) -> Result<String> {
    let manifest: toml::Value = toml::from_str(manifest)?;
    let lock: toml::Value = toml::from_str(lock)?;
    let version = manifest
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(toml::Value::as_str)
        .filter(|s| !s.is_empty())
        .context("manifest package version missing or not a nonempty string")?;
    let package = lock
        .get("package")
        .and_then(toml::Value::as_array)
        .and_then(|packages| {
            packages
                .iter()
                .find(|p| p.get("name").and_then(toml::Value::as_str) == Some("adl"))
        })
        .context("adl package entry missing from Cargo.lock")?;
    let locked = package
        .get("version")
        .and_then(toml::Value::as_str)
        .context("adl lock package version missing or not a string")?;
    if version != locked {
        bail!("release-version check: adl/Cargo.toml version {version} does not match adl/Cargo.lock package version {locked}");
    }
    Ok(version.to_owned())
}

pub fn metadata(expected: &str, input: &str) -> Result<String> {
    let data: Value = serde_json::from_str(input)?;
    let package = data
        .get("packages")
        .and_then(Value::as_array)
        .and_then(|packages| {
            packages
                .iter()
                .find(|p| p.get("name").and_then(Value::as_str) == Some("adl"))
        })
        .context("adl package missing from cargo metadata")?;
    let resolved = package
        .get("version")
        .and_then(Value::as_str)
        .context("adl metadata version missing or not a string")?;
    if resolved != expected {
        bail!("release-version check: cargo metadata resolved {resolved} but manifest says {expected}");
    }
    Ok(resolved.to_owned())
}
