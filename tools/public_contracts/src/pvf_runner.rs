//! Data helpers for the retained shell runner. Commands execute only in its
//! existing shell dispatch, never during parsing, matching, or report rendering.
use anyhow::{ensure, Result};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::{json, Value};
use std::{fmt, fs, path::Path};
#[derive(Default)]
struct Ordered(Vec<(String, Value)>);
impl<'de> Deserialize<'de> for Ordered {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Ordered;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("lane object")
            }
            fn visit_map<M: MapAccess<'de>>(
                self,
                mut m: M,
            ) -> std::result::Result<Ordered, M::Error> {
                let mut rows: Vec<(String, Value)> = Vec::new();
                while let Some((k, v)) = m.next_entry::<String, Value>()? {
                    if let Some(row) = rows.iter_mut().find(|r| r.0 == k) {
                        row.1 = v;
                    } else {
                        rows.push((k, v));
                    }
                }
                Ok(Ordered(rows))
            }
        }
        d.deserialize_map(V)
    }
}
struct Manifest(Ordered);
impl<'de> Deserialize<'de> for Manifest {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Manifest;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("manifest object")
            }
            fn visit_map<M: MapAccess<'de>>(
                self,
                mut m: M,
            ) -> std::result::Result<Manifest, M::Error> {
                let mut lanes = Ordered::default();
                while let Some(k) = m.next_key::<String>()? {
                    if k == "lanes" {
                        lanes = m.next_value()?;
                    } else {
                        m.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
                Ok(Manifest(lanes))
            }
        }
        d.deserialize_map(V)
    }
}
pub fn plan(source: &str) -> Result<String> {
    crate::pvf_manifest::validate(&serde_json::from_str(source)?)?;
    let Manifest(Ordered(lanes)) = serde_json::from_str(source)?;
    let mut out = String::new();
    for (id, lane) in lanes {
        let hints = serde_json::to_string(&lane["changed_path_hints"])?;
        let row = [
            id.as_str(),
            lane["lane_class"].as_str().unwrap(),
            lane["release_gate_class"].as_str().unwrap(),
            lane["default_trigger"].as_str().unwrap(),
            if lane["requires_worktree"] == true {
                "true"
            } else {
                "false"
            },
            if lane["requires_credentials"] == true {
                "true"
            } else {
                "false"
            },
            hints.as_str(),
            lane["command"].as_str().unwrap(),
        ];
        ensure!(
            row.iter().all(|s| !s.contains(['\t', '\n', '\r'])),
            "lane fields cannot contain TSV control characters"
        );
        out += &row.join("\t");
        out.push('\n');
    }
    Ok(out)
}
#[derive(Debug)]
enum Token {
    Star,
    Any,
    Literal(char),
    Class(bool, Vec<(char, char)>),
}
fn tokens(pattern: &str) -> Vec<Token> {
    let chars: Vec<_> = pattern.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        i += 1;
        match c {
            '*' => out.push(Token::Star),
            '?' => out.push(Token::Any),
            '[' => {
                let start = i;
                let mut end = i;
                if end < chars.len() && chars[end] == '!' {
                    end += 1;
                }
                if end < chars.len() && chars[end] == ']' {
                    end += 1;
                }
                while end < chars.len() && chars[end] != ']' {
                    end += 1;
                }
                if end == chars.len() {
                    out.push(Token::Literal('['));
                    continue;
                }
                let negate = chars[start] == '!';
                let mut j = start + usize::from(negate);
                let mut ranges = Vec::new();
                while j < end {
                    if j + 2 < end && chars[j + 1] == '-' {
                        if chars[j] <= chars[j + 2] {
                            ranges.push((chars[j], chars[j + 2]));
                        }
                        j += 3;
                    } else {
                        ranges.push((chars[j], chars[j]));
                        j += 1;
                    }
                }
                out.push(Token::Class(negate, ranges));
                i = end + 1;
            }
            _ => out.push(Token::Literal(c)),
        }
    }
    out
}
pub fn glob(path: &str, pattern: &str) -> bool {
    let path: Vec<_> = path.chars().collect();
    let mut row = vec![false; path.len() + 1];
    row[0] = true;
    for token in tokens(pattern) {
        let mut next = vec![false; row.len()];
        if matches!(token, Token::Star) {
            next[0] = row[0];
        }
        for j in 1..row.len() {
            next[j] = match &token {
                Token::Star => row[j] || next[j - 1],
                Token::Any => row[j - 1],
                Token::Literal(c) => row[j - 1] && path[j - 1] == *c,
                Token::Class(negate, ranges) => {
                    row[j - 1]
                        && (ranges
                            .iter()
                            .any(|(a, b)| *a <= path[j - 1] && path[j - 1] <= *b)
                            != *negate)
                }
            };
        }
        row = next;
    }
    row[path.len()]
}
pub fn matches(changed: &str, hints: &Value) -> Result<&'static str> {
    let changed: Vec<_> = changed
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    let hints = hints
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("hints must be an array"))?;
    let hints = hints
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or_else(|| anyhow::anyhow!("hint must be a string"))
        })
        .collect::<Result<Vec<_>>>()?;
    if changed.is_empty() {
        return Ok("unknown");
    }
    Ok(
        if changed
            .iter()
            .any(|p| hints.iter().any(|h| p.starts_with(h) || glob(p, h)))
        {
            "match"
        } else {
            "no_match"
        },
    )
}
pub fn report(plan: &str, state: &Path, manifest: &str, mode: &str, status: &str) -> Result<Value> {
    let mut lanes = serde_json::Map::new();
    for line in plan.lines() {
        let id = line.split('\t').next().unwrap();
        ensure!(
            !id.is_empty()
                && id
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'),
            "unsafe lane id"
        );
        let mut lane = serde_json::Map::new();
        for (key, suffix) in [
            ("lane_class", "lane_class"),
            ("status", "status"),
            ("reason", "reason"),
            ("output", "out"),
        ] {
            let path = state.join(format!("{id}.{suffix}"));
            let value = if path.is_file() {
                fs::read_to_string(path)?
            } else {
                String::new()
            };
            lane.insert(key.into(), Value::String(value));
        }
        lanes.insert(id.into(), Value::Object(lane));
    }
    Ok(
        json!({"schema_version":"adl.pvf_run.v1","manifest_path":manifest,"mode":mode,"aggregate_status":status,"lanes":lanes}),
    )
}
pub fn cli(args: &[String]) -> Result<()> {
    match args {
        [action,input,output] if action=="plan"=>fs::write(output,plan(&fs::read_to_string(input)?)?)?,
        [action,changed,hints] if action=="match"=>println!("{}",matches(&fs::read_to_string(changed)?,&serde_json::from_str(hints)?)?),
        [action,plan_path,state,manifest,mode,status,output] if action=="report"=>fs::write(output,format!("{}\n",serde_json::to_string_pretty(&report(&fs::read_to_string(plan_path)?,Path::new(state),manifest,mode,status)?)?))?,
        _=>anyhow::bail!("usage: pvf-data plan INPUT OUTPUT | match CHANGED HINTS_JSON | report PLAN STATE MANIFEST MODE STATUS OUTPUT"),
    }
    Ok(())
}
