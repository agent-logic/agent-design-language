//! Full SHA pinning and canonical hosted-action policy.
use regex::Regex;
use std::{fs, path::Path};
const CANONICAL: [(&str, &str); 5] = [
    (
        "dtolnay/rust-toolchain",
        "631a55b12751854ce901bb631d5902ceb48146f7",
    ),
    (
        "actions/checkout",
        "9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0",
    ),
    (
        "actions/download-artifact",
        "37930b1c2abaa49bbe596cd826c3c89aef350131",
    ),
    (
        "actions/upload-artifact",
        "043fb46d1a93c77aae656e7c1c64a875d1fc6a0a",
    ),
    (
        "Swatinem/rust-cache",
        "c19371144df3bb44fab255c43d04cbc2ab54d1c4",
    ),
];
pub fn parse_uses(line: &str) -> Result<Option<String>, String> {
    let line = line.trim();
    let re = Regex::new(r"^(?:-\s+)?uses:\s+(.+?)\s*(?:#.*)?$").unwrap();
    let Some(c) = re.captures(line) else {
        if !line.starts_with('#') && Regex::new(r#"["']?uses["']?\s*:"#).unwrap().is_match(line) {
            return Err("unsupported workflow uses syntax".into());
        }
        return Ok(None);
    };
    let value = c[1].trim();
    if value.starts_with(['\'', '"']) {
        if value.len() < 2 || value.as_bytes()[0] != *value.as_bytes().last().unwrap() {
            return Err("invalid quoted workflow uses scalar".into());
        }
        return Ok(Some(value[1..value.len() - 1].into()));
    }
    Ok(Some(value.into()))
}
pub fn require_pin(value: &str) -> Result<(), String> {
    if value.starts_with("./") {
        if Path::new(value)
            .components()
            .any(|p| p == std::path::Component::ParentDir)
        {
            return Err("local action escapes repository".into());
        }
        return Ok(());
    }
    if !Regex::new(r"^[A-Za-z0-9_.-]+/[A-Za-z0-9_./-]+@[0-9a-f]{40}$")
        .unwrap()
        .is_match(value)
    {
        return Err("external workflow uses must pin a full commit SHA".into());
    }
    for (action, sha) in CANONICAL {
        if value.starts_with(&format!("{action}@")) && value != format!("{action}@{sha}") {
            return Err(format!(
                "workflow must pin {action} to canonical Node 24 SHA"
            ));
        }
    }
    Ok(())
}
pub fn check(root: &Path) -> Result<(), String> {
    for e in fs::read_dir(root.join(".github/workflows")).map_err(|e| e.to_string())? {
        let path = e.map_err(|e| e.to_string())?.path();
        if !path.extension().is_some_and(|x| x == "yml" || x == "yaml") {
            continue;
        }
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        for sha in [
            "34e114876b0b11c390a56381ad16ebd13914f8d5",
            "ea165f8d65b6e75b540449e92b4886f43607fa02",
            "779680da715d629ac1d338a641029a2f4372abb5",
        ] {
            if text.contains(sha) {
                return Err("deprecated Node 20 action SHA".into());
            }
        }
        for line in text.lines() {
            if let Some(value) = parse_uses(line)? {
                require_pin(&value)?;
            }
        }
    }
    Ok(())
}
