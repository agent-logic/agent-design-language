use anyhow::{bail, Context, Result};
use serde_json::{Map, Value};

use crate::model::{
    CommandObservation, NormalizationRule, NormalizedObservation, RawObservation, Stream,
    NORMALIZED_SCHEMA,
};

pub fn normalize(
    raw: &RawObservation,
    rules: &[NormalizationRule],
) -> Result<NormalizedObservation> {
    let mut commands = raw.commands.clone();
    for command in &mut commands {
        command.expanded_args = command
            .declared_args
            .iter()
            .map(|arg| arg.replace("{ROOT}", "<ROOT>").replace("{WORK}", "<WORK>"))
            .collect();
    }
    for rule in rules {
        apply_rule(&mut commands, rule)?;
    }
    Ok(NormalizedObservation {
        schema: NORMALIZED_SCHEMA.into(),
        case_id: raw.case_id.clone(),
        repetition: raw.repetition,
        incumbent_revision: raw.incumbent_revision.clone(),
        binary_sha256: raw.binary_sha256.clone(),
        commands,
    })
}

fn apply_rule(commands: &mut [CommandObservation], rule: &NormalizationRule) -> Result<()> {
    let (step, stream) = match rule {
        NormalizationRule::CanonicalJson { step, stream }
        | NormalizationRule::ReplaceJsonFields { step, stream, .. }
        | NormalizationRule::RemoveExactLine { step, stream, .. } => (step, *stream),
    };
    let command = commands
        .iter_mut()
        .find(|command| command.step_id == *step)
        .ok_or_else(|| anyhow::anyhow!("normalizer names unknown step {step}"))?;
    let text = stream_mut(command, stream);
    let before = text.clone();
    match rule {
        NormalizationRule::CanonicalJson { .. } => {
            let value: Value =
                serde_json::from_str(text).context("canonical_json requires a JSON stream")?;
            *text = serde_json::to_string_pretty(&sort_value(value))? + "\n";
        }
        NormalizationRule::ReplaceJsonFields { fields, .. } => {
            if fields.is_empty() {
                bail!("replace_json_fields requires named fields");
            }
            let mut value: Value =
                serde_json::from_str(text).context("replace_json_fields requires a JSON stream")?;
            let mut replacements = 0;
            replace_fields(&mut value, fields, &mut replacements);
            if replacements == 0 {
                bail!("replace_json_fields matched no declared fields");
            }
            *text = serde_json::to_string_pretty(&sort_value(value))? + "\n";
        }
        NormalizationRule::RemoveExactLine { line, .. } => {
            let mut removed = false;
            let retained = text
                .lines()
                .filter(|candidate| {
                    if !removed && *candidate == line {
                        removed = true;
                        false
                    } else {
                        true
                    }
                })
                .collect::<Vec<_>>();
            *text = if retained.is_empty() {
                String::new()
            } else {
                retained.join("\n") + "\n"
            };
        }
    }
    if *text == before {
        bail!("normalization rule for {step} was a no-op");
    }
    Ok(())
}

fn stream_mut(command: &mut CommandObservation, stream: Stream) -> &mut String {
    match stream {
        Stream::Stdout => &mut command.stdout,
        Stream::Stderr => &mut command.stderr,
    }
}

fn sort_value(value: Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut entries = object.into_iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            Value::Object(
                entries
                    .into_iter()
                    .map(|(key, value)| (key, sort_value(value)))
                    .collect::<Map<_, _>>(),
            )
        }
        Value::Array(values) => Value::Array(values.into_iter().map(sort_value).collect()),
        other => other,
    }
}

fn replace_fields(value: &mut Value, fields: &[String], replacements: &mut usize) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                if fields.contains(key) {
                    *value = Value::String(format!("<{key}>"));
                    *replacements += 1;
                } else {
                    replace_fields(value, fields, replacements);
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                replace_fields(value, fields, replacements);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CommandObservation, RawObservation, OBSERVATION_SCHEMA};

    fn observation(stdout: &str) -> RawObservation {
        RawObservation {
            schema: OBSERVATION_SCHEMA.into(),
            case_id: "case".into(),
            repetition: 1,
            incumbent_revision: "a".repeat(40),
            binary_sha256: "b".repeat(64),
            commands: vec![CommandObservation {
                step_id: "step".into(),
                declared_args: vec![],
                expanded_args: vec![],
                exit_code: 0,
                stdout_sha256: "a".repeat(64),
                stderr_sha256: "b".repeat(64),
                stdout: stdout.into(),
                stderr: String::new(),
            }],
        }
    }

    #[test]
    fn canonical_json_sorts_objects_but_preserves_arrays() {
        let raw = observation(r#"{"z":1,"a":[{"b":2,"a":1},0]}"#);
        let normalized = normalize(
            &raw,
            &[NormalizationRule::CanonicalJson {
                step: "step".into(),
                stream: Stream::Stdout,
            }],
        )
        .unwrap();
        assert!(normalized.commands[0].stdout.starts_with("{\n  \"a\": ["));
        assert!(normalized.commands[0]
            .stdout
            .contains("{\n      \"a\": 1,\n      \"b\": 2"));
    }

    #[test]
    fn no_op_normalizer_is_rejected() {
        let error = normalize(
            &observation("stable\n"),
            &[NormalizationRule::RemoveExactLine {
                step: "step".into(),
                stream: Stream::Stdout,
                line: "absent".into(),
            }],
        )
        .unwrap_err();
        assert!(error.to_string().contains("no-op"));
    }
}
