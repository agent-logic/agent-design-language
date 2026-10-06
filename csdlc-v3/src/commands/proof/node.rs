//! Bounded Node built-in test-runner adapter. No shell or caller-supplied reporter.
use super::*;

pub(super) fn admit(root: &Path, validator: &Validator) -> Result<(), String> {
    if validator.success_marker != "node:test"
        || validator.args.first().map(String::as_str) != Some("--test")
        || !(2..=33).contains(&validator.args.len())
    {
        return Err("intent_node_test_arguments_invalid".into());
    }
    let mut seen = std::collections::BTreeSet::new();
    for name in validator.args.iter().skip(1) {
        if !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_./-".contains(&b))
            || name.starts_with('-')
            || name.starts_with('.')
            || !seen.insert(name)
            || ![".js", ".mjs", ".cjs"]
                .iter()
                .any(|suffix| name.ends_with(suffix))
            || name
                .split('/')
                .any(|part| part.is_empty() || part == ".." || part == ".")
        {
            return Err("intent_node_test_arguments_invalid".into());
        }
        let path = resolve_repo_path(root, name, true).map_err(|finding| finding.code)?;
        if !path.is_file() || git_read(root, &["ls-files", "--error-unmatch", "--", name]).is_err()
        {
            return Err("intent_validator_input_not_tracked".into());
        }
    }
    Ok(())
}

// Node's TAP reporter prefixes test stdout with '# ', so forged footer lines
// printed by a test become '# # tests ...', not a runner summary. Require one
// complete root summary and real successful cases, not a caller marker.
pub(super) fn counts(root: &Path, validator: &Validator, text: &str) -> Option<(u64, u64)> {
    if !text.starts_with("TAP version 13\n") {
        return None;
    }
    // A file without any node:test cases receives an implicit passing file test.
    // That is not nonempty test evidence, even if it printed a fake summary.
    if text
        .lines()
        .filter_map(|line| line.strip_prefix("# Subtest: "))
        .any(|name| {
            validator
                .args
                .iter()
                .skip(1)
                .any(|file| name == file || name == root.join(file).to_string_lossy())
        })
    {
        return None;
    }
    let field = |prefix: &str| {
        let values = text
            .lines()
            .filter_map(|line| line.strip_prefix(prefix))
            .collect::<Vec<_>>();
        if values.len() != 1 {
            return None;
        }
        values[0].parse::<u64>().ok()
    };
    let tests = field("# tests ")?;
    let pass = field("# pass ")?;
    let fail = field("# fail ")?;
    let cancelled = field("# cancelled ")?;
    let skipped = field("# skipped ")?;
    let todo = field("# todo ")?;
    let total = pass
        .checked_add(fail)?
        .checked_add(cancelled)?
        .checked_add(skipped)?
        .checked_add(todo)?;
    if tests == 0 || tests != total || cancelled != 0 || todo != 0 {
        return None;
    }
    Some((pass, fail))
}
