//! Planning draft rendering and structural checks; never review/approval authority.
use anyhow::{bail, ensure, Context, Result};
use regex::Regex;
use serde_json::{json, Map, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
fn object(path: &Path) -> Result<Value> {
    let v: Value = serde_json::from_slice(
        &fs::read(path).with_context(|| format!("read {}", path.display()))?,
    )?;
    ensure!(
        v.is_object(),
        "JSON file must contain an object: {}",
        path.display()
    );
    Ok(v)
}
fn registry(path: &Path) -> Result<Value> {
    let v = object(path)?;
    ensure!(
        v["schema"] == "adl.planning_template_registry.v1",
        "unsupported registry schema"
    );
    Ok(v)
}
fn registered_path(registry: &Path, value: &str) -> Result<PathBuf> {
    let path = Path::new(value);
    ensure!(
        !path.is_absolute(),
        "registered template path must be relative: {value}"
    );
    let resolved = registry.canonicalize()?;
    let root = if resolved.ends_with("docs/templates/planning/current.json") {
        resolved
            .ancestors()
            .nth(4)
            .context("registry root missing")?
    } else {
        resolved.parent().context("registry parent missing")?
    };
    Ok(root.join(path))
}
fn template<'a>(
    registry: &'a Value,
    path: &Path,
    key: &str,
    check_version: bool,
) -> Result<(&'a Value, PathBuf, &'a str)> {
    let template = registry
        .get("templates")
        .and_then(|v| v.get(key))
        .filter(|v| v.is_object())
        .with_context(|| format!("unknown template key: {key}"))?;
    ensure!(
        template["status"] == "active",
        "template is not active: {key}"
    );
    let display = template["path"]
        .as_str()
        .context("template path is missing or invalid")?;
    let target = registered_path(path, display)?;
    let resolved = target.canonicalize().with_context(|| {
        format!(
            "registered template file does not exist: {}",
            target.display()
        )
    })?;
    if let Some(root) = registry
        .get("template_root")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
    {
        let root = registered_path(path, root)?.canonicalize()?;
        ensure!(
            resolved.starts_with(root),
            "template path is outside active template root: {display}"
        );
    }
    ensure!(resolved.is_file(), "registered template must be a file");
    if check_version {
        ensure!(
            template.get("version") == registry.get("planning_template_set"),
            "template version does not match registry set: {key}"
        );
    }
    Ok((template, resolved, display))
}
fn ascii_json_string(s: &str) -> String {
    let quoted = serde_json::to_string(s).expect("string serialization");
    let mut out = String::new();
    for ch in quoted.chars() {
        if ch.is_ascii() {
            out.push(ch)
        } else {
            for unit in ch.encode_utf16(&mut [0; 2]) {
                out.push_str(&format!("\\u{unit:04x}"));
            }
        }
    }
    out
}
fn spaced_json(v: &Value) -> String {
    match v {
        Value::String(s) => ascii_json_string(s),
        Value::Array(a) => format!(
            "[{}]",
            a.iter().map(spaced_json).collect::<Vec<_>>().join(", ")
        ),
        Value::Object(m) => format!(
            "{{{}}}",
            m.iter()
                .map(|(k, v)| format!("{}: {}", ascii_json_string(k), spaced_json(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => v.to_string(),
    }
}
fn quoted_text(s: &str) -> String {
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut result = String::from(quote);
    for c in s.chars() {
        match c {
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\\' => result.push_str("\\\\"),
            c if c == quote => {
                result.push('\\');
                result.push(c)
            }
            c if c.is_control() => {
                let n = c as u32;
                result.push_str(&if n <= 255 {
                    format!("\\x{n:02x}")
                } else {
                    format!("\\u{n:04x}")
                })
            }
            _ => result.push(c),
        }
    }
    result.push(quote);
    result
}
fn representation(v: &Value) -> String {
    match v {
        Value::String(s) => quoted_text(s),
        Value::Null => "None".into(),
        Value::Bool(true) => "True".into(),
        Value::Bool(false) => "False".into(),
        Value::Array(a) => format!(
            "[{}]",
            a.iter().map(representation).collect::<Vec<_>>().join(", ")
        ),
        Value::Object(m) => format!(
            "{{{}}}",
            m.iter()
                .map(|(k, v)| format!("{}: {}", quoted_text(k), representation(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => v.to_string(),
    }
}
fn scalar_text(v: &Value) -> String {
    if let Value::String(s) = v {
        s.clone()
    } else {
        representation(v)
    }
}
fn stringify(v: &Value) -> String {
    match v {
        Value::Array(a) => a.iter().map(scalar_text).collect::<Vec<_>>().join("\n"),
        Value::Object(_) => spaced_json(v),
        _ => scalar_text(v),
    }
}
pub fn fill(text: &str, values: &Map<String, Value>) -> (String, Vec<String>) {
    let pattern = Regex::new(r"<([A-Za-z][A-Za-z0-9_]*)>").unwrap();
    let mut missing = BTreeSet::new();
    let filled = pattern
        .replace_all(text, |captures: &regex::Captures| {
            let key = &captures[1];
            if let Some(value) = values.get(key) {
                stringify(value)
            } else {
                missing.insert(key.to_owned());
                captures[0].to_owned()
            }
        })
        .into_owned();
    (filled, missing.into_iter().collect())
}
pub fn failures(text: &str, template: &Value) -> Vec<String> {
    let mut failures = vec![];
    let placeholder = Regex::new(r"<[A-Za-z][A-Za-z0-9_]*>|\{\{[A-Za-z][A-Za-z0-9_]*\}\}").unwrap();
    let unresolved = placeholder
        .find_iter(text)
        .map(|v| v.as_str())
        .collect::<BTreeSet<_>>();
    if !unresolved.is_empty() {
        failures.push(format!(
            "unresolved placeholders: {}",
            unresolved.into_iter().collect::<Vec<_>>().join(", ")
        ));
    }
    match template.get("required_sections") {
        None => (),
        Some(Value::Array(sections)) => {
            let mut missing = vec![];
            for section in sections {
                let section = scalar_text(section);
                let regex =
                    Regex::new(&format!(r"(?m)^#+\s+{}\s*$", regex::escape(&section))).unwrap();
                if !regex.is_match(text) {
                    missing.push(section)
                }
            }
            if !missing.is_empty() {
                failures.push(format!("missing required sections: {}", missing.join(", ")));
            }
        }
        _ => failures.push("registry required_sections is not a list".into()),
    }
    failures
}
fn generated_header(registry: &Value, key: &str, display: &str) -> String {
    let set = registry
        .get("planning_template_set")
        .map(scalar_text)
        .unwrap_or_else(|| "None".into());
    format!("<!--\nGenerated Planning Draft\nplanning_template_set: {set}\ntemplate: {key}\ntemplate_path: {display}\ngeneration_status: generated_draft\nclaim_boundary: generated draft only; not reviewed or approved\n-->\n\n> Generated planning draft. This file proves only template filling;\n> it is not reviewed, approved, released, merged, or lifecycle-true.\n")
}
pub fn cli(args: &[String], validate: bool) -> i32 {
    if args.iter().any(|s| matches!(s.as_str(), "--help" | "-h")) {
        println!(
            "planning-{} --registry PATH --template KEY {}",
            if validate { "validate" } else { "fill" },
            if validate {
                "--input PATH"
            } else {
                "--values PATH --output PATH [--allow-missing]"
            }
        );
        return 0;
    }
    let options = (|| -> Result<BTreeMap<String, String>> {
        let mut options = BTreeMap::new();
        let mut args = args.iter();
        while let Some(key) = args.next() {
            let allowed = matches!(key.as_str(), "--registry" | "--template")
                || if validate {
                    key == "--input"
                } else {
                    matches!(key.as_str(), "--values" | "--output" | "--allow-missing")
                };
            ensure!(allowed, "unknown option {key}");
            let value = if key == "--allow-missing" {
                "true".into()
            } else {
                args.next().context("option requires a value")?.clone()
            };
            ensure!(
                options.insert(key.clone(), value).is_none(),
                "duplicate option {key}"
            );
        }
        for key in if validate {
            vec!["--template", "--input"]
        } else {
            vec!["--template", "--values", "--output"]
        } {
            ensure!(options.contains_key(key), "missing {key}");
        }
        Ok(options)
    })();
    let options = match options {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:#}");
            return 2;
        }
    };
    let registry_path = Path::new(
        options
            .get("--registry")
            .map(String::as_str)
            .unwrap_or("docs/templates/planning/current.json"),
    );
    let registry = match registry(registry_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:#}");
            return 1;
        }
    };
    let key = &options["--template"];
    if validate {
        let result = (|| -> Result<(Value, Vec<String>)> {
            let (template, _, _) = template(&registry, registry_path, key, true)?;
            let input = &options["--input"];
            let text = fs::read_to_string(input).context("input not found or unreadable")?;
            Ok((
                json!({"status":"PASS","template":key,"input":input,"registry_template_set":registry["planning_template_set"],"claim_boundary":"structural validation only; not review or approval"}),
                failures(&text, template),
            ))
        })();
        match result {
            Err(e) => {
                eprintln!("{e:#}");
                2
            }
            Ok((report, errors)) => {
                if errors.is_empty() {
                    println!("{report}");
                    0
                } else {
                    for error in errors {
                        eprintln!("FAIL: {error}");
                    }
                    1
                }
            }
        }
    } else {
        let result = (|| -> Result<Value> {
            let values = object(Path::new(&options["--values"]))?;
            let (_, path, display) = template(&registry, registry_path, key, false)?;
            let (filled, missing) = fill(&fs::read_to_string(path)?, values.as_object().unwrap());
            if !missing.is_empty() && !options.contains_key("--allow-missing") {
                bail!("missing values: {}", missing.join(", "));
            }
            let output = Path::new(&options["--output"]);
            if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) {
                fs::create_dir_all(parent)?;
            }
            fs::write(output, generated_header(&registry, key, display) + &filled)?;
            Ok(
                json!({"status":if missing.is_empty(){"PASS"}else{"PARTIAL"},"template":key,"template_path":display,"values":options["--values"],"output":options["--output"],"missing_values":missing,"claim_boundary":"generated draft only; not review or approval"}),
            )
        })();
        match result {
            Ok(report) => {
                println!("{report}");
                0
            }
            Err(e) => {
                eprintln!("{e:#}");
                1
            }
        }
    }
}
