//! Canonical upstream corpus, separate from historical ADL admission fixtures.
//! Callers supply their offline schema engine; this module never executes tools.
use serde::Serialize;
use serde_json::Value;

pub const RESOURCE_PIN_JSON: &str = include_str!("../resources-pin.json");
pub const MANIFEST_JSON: &str = include_str!("../resources/conformance/manifest.json");
pub const FIXTURES: &[(&str, &str)] = &[
    (
        "conformance/valid/safe-read-v1.json",
        include_str!("../resources/conformance/valid/safe-read-v1.json"),
    ),
    (
        "conformance/valid/registry-preview-v1_1.json",
        include_str!("../resources/conformance/valid/registry-preview-v1_1.json"),
    ),
    (
        "conformance/valid/invocation-read-v1_1.json",
        include_str!("../resources/conformance/valid/invocation-read-v1_1.json"),
    ),
    (
        "conformance/invalid/auth-required-none-v1.json",
        include_str!("../resources/conformance/invalid/auth-required-none-v1.json"),
    ),
    (
        "conformance/invalid/auth-required-none-v1_1.json",
        include_str!("../resources/conformance/invalid/auth-required-none-v1_1.json"),
    ),
    (
        "conformance/invalid/high-risk-without-exfiltration-v1_1.json",
        include_str!("../resources/conformance/invalid/high-risk-without-exfiltration-v1_1.json"),
    ),
    (
        "conformance/invalid/none-plus-side-effects-v1_1.json",
        include_str!("../resources/conformance/invalid/none-plus-side-effects-v1_1.json"),
    ),
    (
        "conformance/invalid/none-plus-side-effects-invocation.json",
        include_str!("../resources/conformance/invalid/none-plus-side-effects-invocation.json"),
    ),
    (
        "conformance/dangerous/exfiltration-high-v1.json",
        include_str!("../resources/conformance/dangerous/exfiltration-high-v1.json"),
    ),
    (
        "conformance/reconciliation/lifecycle-stable.json",
        include_str!("../resources/conformance/reconciliation/lifecycle-stable.json"),
    ),
    (
        "conformance/reconciliation/lifecycle-unknown.json",
        include_str!("../resources/conformance/reconciliation/lifecycle-unknown.json"),
    ),
    (
        "conformance/reconciliation/description-short.json",
        include_str!("../resources/conformance/reconciliation/description-short.json"),
    ),
    (
        "conformance/reconciliation/extension-authority.json",
        include_str!("../resources/conformance/reconciliation/extension-authority.json"),
    ),
    (
        "conformance/reconciliation/caller-string.json",
        include_str!("../resources/conformance/reconciliation/caller-string.json"),
    ),
    (
        "conformance/reconciliation/intent-extensible.json",
        include_str!("../resources/conformance/reconciliation/intent-extensible.json"),
    ),
    (
        "conformance/reconciliation/legacy-retry.json",
        include_str!("../resources/conformance/reconciliation/legacy-retry.json"),
    ),
    (
        "conformance/reconciliation/legacy-timeout.json",
        include_str!("../resources/conformance/reconciliation/legacy-timeout.json"),
    ),
    (
        "conformance/reconciliation/retry-negative.json",
        include_str!("../resources/conformance/reconciliation/retry-negative.json"),
    ),
    (
        "conformance/reconciliation/retry-declared-not-authorized.json",
        include_str!("../resources/conformance/reconciliation/retry-declared-not-authorized.json"),
    ),
    (
        "conformance/reconciliation/tool-external-name.json",
        include_str!("../resources/conformance/reconciliation/tool-external-name.json"),
    ),
    (
        "conformance/reconciliation/invocation-extension-permission.json",
        include_str!(
            "../resources/conformance/reconciliation/invocation-extension-permission.json"
        ),
    ),
    (
        "conformance/reconciliation/errors-empty.json",
        include_str!("../resources/conformance/reconciliation/errors-empty.json"),
    ),
    (
        "conformance/reconciliation/extension-required.json",
        include_str!("../resources/conformance/reconciliation/extension-required.json"),
    ),
];

#[derive(Debug, Serialize)]
pub struct CaseResult {
    pub path: String,
    pub schema: String,
    pub expected: String,
    pub schema_valid: bool,
    pub binding_valid: Option<bool>,
    pub normalized_valid: Option<bool>,
    pub execution_granted: bool,
    pub passed: bool,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub schema: &'static str,
    pub resource_pin: Value,
    pub fixture_count: usize,
    pub passed: bool,
    pub cases: Vec<CaseResult>,
}

/// Exercise every canonical case and declaration normalization with an offline
/// schema validator. Invocation cases establish schema-engine parity only;
/// execution/admission evidence belongs to the Runtime owner.
pub fn run(validate: impl Fn(&Value, &Value) -> bool) -> Result<Report, String> {
    let decode = |text: &str| serde_json::from_str::<Value>(text).map_err(|e| e.to_string());
    let manifest = decode(MANIFEST_JSON)?;
    let v1 = decode(crate::V1_SCHEMA_JSON)?;
    let v11 = decode(crate::V1_1_SCHEMA_JSON)?;
    let invocation = decode(crate::V1_1_INVOCATION_SCHEMA_JSON)?;
    let mut cases = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for case in manifest["fixtures"].as_array().ok_or("invalid corpus")? {
        let path = case["path"].as_str().ok_or("missing path")?;
        if !seen.insert(path) {
            return Err("duplicate fixture".into());
        }
        let text = FIXTURES
            .iter()
            .find(|(name, _)| *name == path)
            .ok_or("missing fixture")?
            .1;
        let value = decode(text)?;
        let schema = case["schema"].as_str().ok_or("missing schema")?;
        let expected = case["expected"].as_str().ok_or("missing expectation")?;
        let expected_valid = match expected {
            "valid" | "dangerous_valid" => true,
            "invalid" => false,
            _ => return Err("unknown expectation".into()),
        };
        let source_schema = match schema {
            "uts.v1" => &v1,
            "uts.v1.1" => &v11,
            "tool_invocation.v1.1" => &invocation,
            _ => return Err("unknown schema".into()),
        };
        let schema_valid = validate(source_schema, &value);
        let (binding_valid, normalized_valid) = if schema == "tool_invocation.v1.1" {
            (None, None)
        } else {
            match crate::load_tool_declaration(value) {
                Ok(declaration) => {
                    let normalized =
                        serde_json::to_value(declaration).map_err(|e| e.to_string())?;
                    (Some(true), Some(validate(&v11, &normalized)))
                }
                Err(_) => (Some(false), None),
            }
        };
        let passed = schema_valid == expected_valid
            && binding_valid.is_none_or(|valid| valid == expected_valid)
            && normalized_valid.is_none_or(|valid| valid);
        cases.push(CaseResult {
            path: path.into(),
            schema: schema.into(),
            expected: expected.into(),
            schema_valid,
            binding_valid,
            normalized_valid,
            execution_granted: false,
            passed,
        });
    }
    if cases.len() != FIXTURES.len() {
        return Err("corpus membership mismatch".into());
    }
    Ok(Report {
        schema: "adl.uts.canonical-conformance.v1",
        resource_pin: decode(RESOURCE_PIN_JSON)?,
        fixture_count: cases.len(),
        passed: cases.iter().all(|case| case.passed),
        cases,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // PVF local_contract: deterministic CPU-only, required report-integrity
    // regression. Deliberately broken engines must never yield a passing report.
    #[test]
    fn rejecting_engine_records_failed_valid_cases_and_normalization() {
        let report = run(|_, _| false).unwrap();
        assert_eq!(report.fixture_count, 23);
        assert!(!report.passed);
        assert_eq!(report.resource_pin["resource_version"], "0.1.0-rc.1");
        assert!(report.cases.iter().all(|case| !case.execution_granted));
        let stable = report
            .cases
            .iter()
            .find(|case| case.path.ends_with("lifecycle-stable.json"))
            .unwrap();
        assert_eq!(stable.binding_valid, Some(true));
        assert_eq!(stable.normalized_valid, Some(false));
        assert!(!stable.passed);
        let unknown = report
            .cases
            .iter()
            .find(|case| case.path.ends_with("lifecycle-unknown.json"))
            .unwrap();
        assert_eq!(unknown.binding_valid, Some(false));
        assert_eq!(unknown.normalized_valid, None);
        assert!(unknown.passed);
        let invocation = report
            .cases
            .iter()
            .find(|case| case.path.ends_with("invocation-read-v1_1.json"))
            .unwrap();
        assert_eq!(invocation.binding_valid, None);
        assert_eq!(invocation.normalized_valid, None);
        assert!(!invocation.passed);
    }

    // PVF local_contract: reject vacuous success from a permissive schema engine.
    #[test]
    fn accepting_engine_cannot_hide_invalid_corpus_cases() {
        let report = run(|_, _| true).unwrap();
        assert!(!report.passed);
        let invalid = report
            .cases
            .iter()
            .find(|case| case.path.ends_with("caller-string.json"))
            .unwrap();
        assert!(invalid.schema_valid);
        assert!(!invalid.passed);
        let valid = report
            .cases
            .iter()
            .find(|case| case.path.ends_with("extension-required.json"))
            .unwrap();
        assert!(valid.passed);
        assert_eq!(valid.binding_valid, Some(true));
        assert_eq!(valid.normalized_valid, Some(true));
        let wire = serde_json::to_value(report).unwrap();
        assert_eq!(wire["schema"], "adl.uts.canonical-conformance.v1");
        assert_eq!(wire["cases"].as_array().unwrap().len(), 23);
        assert_eq!(wire["passed"], false);
    }
}
