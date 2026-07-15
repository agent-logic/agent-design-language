use std::{fs, path::PathBuf};

pub fn resolve(explicit_path: Option<&str>) -> crate::Result<String> {
    for key in ["ADL_GITHUB_TOKEN", "GITHUB_TOKEN", "GH_TOKEN"] {
        if let Ok(value) = std::env::var(key) {
            if !value.trim().is_empty() {
                return Ok(value);
            }
        }
    }
    let path = explicit_path
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("ADL_GITHUB_TOKEN_FILE").map(PathBuf::from))
        .or_else(|| {
            std::env::var_os("HOME").map(|home| PathBuf::from(home).join("keys/github.token"))
        })
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "GitHub token source is unavailable",
            )
        })?;
    let value = fs::read_to_string(path).map_err(|_| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "GitHub token source is unavailable",
        )
    })?;
    if value.trim().is_empty() {
        return Err(crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "GitHub token source is empty",
        ));
    }
    Ok(value.trim().to_owned())
}
