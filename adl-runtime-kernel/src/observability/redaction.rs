pub(super) fn validate_otlp_endpoint(endpoint: Option<&str>) -> Result<(), String> {
    let Some(endpoint) = endpoint else {
        return Ok(());
    };
    if endpoint.starts_with("https://")
        || endpoint.starts_with("http://127.0.0.1:")
        || endpoint.starts_with("http://localhost:")
        || endpoint.starts_with("http://[::1]:")
    {
        return Ok(());
    }
    Err("otlp_endpoint_requires_https_or_loopback".to_owned())
}

pub(super) fn redact_field(key: &str, value: &str) -> String {
    if sensitive_key(key) || sensitive_text(value) {
        "<redacted>".to_owned()
    } else {
        value.to_owned()
    }
}

fn sensitive_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    [
        "secret",
        "token",
        "authorization",
        "password",
        "api_key",
        "access_key",
        "private_key",
        "credential",
    ]
    .iter()
    .any(|sensitive| normalized.contains(sensitive))
}

fn sensitive_text(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
    value.contains("-----BEGIN PRIVATE KEY-----")
        || [
            "authorization:",
            "api_key=",
            "password=",
            "token=",
            "secret=",
        ]
        .iter()
        .any(|needle| lowered.contains(needle))
}
