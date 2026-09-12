//! Shared SIM-01/02/03 installed-command measurement support, not a lifecycle
//! implementation. Callers own request validity and semantic assertions.
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::Instant,
};

#[derive(Clone, Copy)]
pub struct ClockSample {
    pub wall_unix_millis: u64,
    pub monotonic_millis: u64,
}

/// Inject fixture clocks so neither sequencing nor verdicts depend on host time.
pub trait Clock {
    fn sample(&mut self) -> ClockSample;
}

pub struct FixtureClock(pub u64);
impl Clock for FixtureClock {
    fn sample(&mut self) -> ClockSample {
        let sample = ClockSample {
            wall_unix_millis: 1_789_084_800_000 + self.0,
            monotonic_millis: self.0,
        };
        self.0 += 10;
        sample
    }
}

pub struct Corpus<C> {
    pub attempts: Vec<Value>,
    clock: C,
    root: PathBuf,
    provenance: Value,
    output_path: PathBuf,
    pub transport_log: PathBuf,
}

impl<C: Clock> Corpus<C> {
    pub fn new(root: &Path, binary: &Path, source: &Path, clock: C, output_path: &Path) -> Self {
        let git = |args: &[&str]| {
            let output = Command::new("git")
                .arg("-C")
                .arg(source)
                .args(args)
                .output()
                .unwrap();
            assert!(output.status.success());
            output.stdout
        };
        let source_head = String::from_utf8(git(&["rev-parse", "HEAD"])).unwrap();
        let tracked_diff = git(&["diff", "HEAD", "--", "csdlc-v3"]);
        let source_inventory = ["src", "tests"].map(|directory| {
            (
                directory,
                super::observation::inventory(&source.join("csdlc-v3").join(directory)),
            )
        });
        Self {
            attempts: Vec::new(),
            clock,
            root: root.to_path_buf(),
            output_path: output_path.to_path_buf(),
            transport_log: output_path.with_extension("transport-calls"),
            provenance: json!({
                "source_head": source_head.trim(),
                "tracked_diff_blake3": blake3::hash(&tracked_diff).to_hex().to_string(),
                "source_and_tests_inventory_blake3": blake3::hash(&serde_json::to_vec(&source_inventory).unwrap()).to_hex().to_string(),
                "binary_blake3": blake3::hash(&fs::read(binary).unwrap()).to_hex().to_string(),
                "os": std::env::consts::OS, "architecture": std::env::consts::ARCH,
                "rustc": tool_version("rustc"), "git": tool_version("git"),
                "transport": "isolated fake remote; no live credentials or network",
                "clock": "injected synthetic fixture clock; actual subprocess duration recorded separately",
                "scope": "installed fixture attempts, not production reliability or causal timing savings"
            }),
        }
    }

    /// Start and outcome are both retained; unsuccessful processes are never
    /// reclassified as completed lifecycle work just because a test expects denial.
    pub fn run(
        &mut self,
        id: &str,
        route: &str,
        scenario: &str,
        cwd: &Path,
        issue_indexes: (&Path, &Path),
        request: &Value,
        command: &mut Command,
    ) -> Output {
        let before = super::observation::inventory(&self.root);
        let version_before = issue_version(issue_indexes.0);
        let start = self.clock.sample();
        let mut fields = Vec::new();
        field_paths(request, "", &mut fields);
        self.attempts.push(json!({
            "correlation_id":id,"route":route,"scenario":scenario,
            "eligible":true,"attempted":true,"completed":false,"abandoned":false,"censored":true,
            "outcome":"censored","reason":"started; subprocess outcome not yet retained",
            "manual_input_fields":fields,"operator_invocation_count":1,
            "issue_version_before":version_before,"wall_start_millis":start.wall_unix_millis,
            "monotonic_start_millis":start.monotonic_millis,"wait_owner":"installed subprocess"
        }));
        self.write(&self.output_path);
        let transport_before = line_count(&self.transport_log);
        let measured = Instant::now();
        let output = match command.output() {
            Ok(output) => output,
            Err(_) => {
                self.attempts.last_mut().unwrap()["censored"] = json!(false);
                self.classify(
                    "failed",
                    "could not start or collect installed subprocess",
                    false,
                    None,
                );
                panic!("installed subprocess unavailable; failed attempt retained");
            }
        };
        let measured_micros = measured.elapsed().as_micros();
        let end = self.clock.sample();
        let after = super::observation::inventory(&self.root);
        let effects: Vec<_> = before
            .keys()
            .chain(after.keys())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .filter(|path| before.get(*path) != after.get(*path))
            .map(|path| json!({"path":path,"before":before.get(path),"after":after.get(path)}))
            .collect();
        let stdout = String::from_utf8_lossy(&output.stdout)
            .replace(self.root.to_str().unwrap(), "$FIXTURE");
        let stderr = String::from_utf8_lossy(&output.stderr)
            .replace(self.root.to_str().unwrap(), "$FIXTURE");
        let leaked =
            stdout.contains("SIM01_SECRET_SENTINEL") || stderr.contains("SIM01_SECRET_SENTINEL");
        let stdout = stdout.replace("SIM01_SECRET_SENTINEL", "[REDACTED]");
        let stderr = stderr.replace("SIM01_SECRET_SENTINEL", "[REDACTED]");
        let parsed: Value = serde_json::from_str(&stdout).unwrap_or(Value::Null);
        *self.attempts.last_mut().unwrap() = json!({
            "correlation_id": id, "route":route, "scenario":scenario,
            "checkout": cwd.strip_prefix(&self.root).unwrap_or(cwd),
            "eligible":true, "attempted":true, "completed":false,
            "abandoned":false, "censored":false,
            "process_exit":output.status.code(), "outcome":"failed",
            "reason":"raw result retained; semantic classification not yet completed",
            "redaction_violation":leaked,
            "invalidation_cause":Value::Null, "wait_owner":"installed subprocess",
            "manual_input_field_count":fields.len(), "manual_input_fields":fields, "manual_input_source":"fixture supplied request fields, not measured human typing",
            "operator_invocation_count":1, "nested_invocation_count":line_count(&self.transport_log)-transport_before,
            "nested_transport_count":line_count(&self.transport_log)-transport_before,"nested_cli_count":0,
            "invocation_count_scope":"installed CLI entrypoints and fake remote transport; internal Git utilities excluded",
            "issue_version_before":version_before, "issue_version_after":issue_version(issue_indexes.1),
            "effects":effects, "stdout":parsed, "stdout_text":stdout, "stderr":stderr,
            "wall_start_millis":start.wall_unix_millis, "wall_end_millis":end.wall_unix_millis,
            "monotonic_start_millis":start.monotonic_millis,"monotonic_end_millis":end.monotonic_millis,
            "fixture_duration_millis":end.monotonic_millis-start.monotonic_millis,
            "wall_duration_millis":end.wall_unix_millis-start.wall_unix_millis,
            "observed_subprocess_duration_micros":measured_micros
        });
        self.write(&self.output_path);
        assert!(!leaked, "redaction violation retained without secret bytes");
        output
    }

    pub fn classify(
        &mut self,
        outcome: &str,
        reason: &str,
        completed: bool,
        invalidation: Option<&str>,
    ) {
        let record = self.attempts.last_mut().expect("attempt exists");
        record["outcome"] = json!(outcome);
        record["reason"] = json!(reason);
        record["completed"] = json!(completed);
        record["invalidation_cause"] = json!(invalidation);
        self.write(&self.output_path);
    }

    pub fn record_nested_cli(&mut self, count: usize) {
        let record = self.attempts.last_mut().unwrap();
        record["nested_cli_count"] = json!(count);
        record["nested_invocation_count"] =
            json!(count + record["nested_transport_count"].as_u64().unwrap() as usize);
        self.write(&self.output_path);
    }

    pub fn write(&self, output: &Path) {
        assert!(
            !output.starts_with(&self.root),
            "measurement artifacts must be outside inspected fixture storage"
        );
        let mut denominator = BTreeMap::new();
        for name in [
            "eligible",
            "attempted",
            "completed",
            "abandoned",
            "censored",
        ] {
            denominator.insert(
                name,
                self.attempts.iter().filter(|a| a[name] == true).count(),
            );
        }
        let mut outcomes = BTreeMap::new();
        for record in &self.attempts {
            *outcomes
                .entry(record["outcome"].as_str().unwrap())
                .or_insert(0usize) += 1;
        }
        fs::create_dir_all(output.parent().unwrap()).unwrap();
        let pending = output.with_extension("next");
        fs::write(
            &pending,
            serde_json::to_vec_pretty(&json!({
                "schema":"csdlc.sim01.attempt_corpus.v1", "provenance":self.provenance,
                "denominators":denominator, "outcomes":outcomes, "attempts":self.attempts
            }))
            .unwrap(),
        )
        .unwrap();
        fs::File::open(&pending).unwrap().sync_all().unwrap();
        fs::rename(&pending, output).unwrap();
    }
}

fn line_count(path: &Path) -> usize {
    match fs::read(path) {
        Ok(bytes) => bytes.iter().filter(|byte| **byte == b'\n').count(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
        Err(error) => panic!("measurement log unreadable: {error}"),
    }
}

fn tool_version(program: &str) -> String {
    let output = Command::new(program)
        .arg("--version")
        .output()
        .expect("required fixture tool available");
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn issue_version(path: &Path) -> Value {
    match fs::read(path) {
        Ok(bytes) => match serde_json::from_slice::<Value>(&bytes) {
            Ok(value) => {
                json!({"generation":value["generation"],"digest":value["digest"],"phase":value["phase"]})
            }
            Err(_) => {
                json!({"state":"corrupt","bytes_blake3":blake3::hash(&bytes).to_hex().to_string()})
            }
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => json!({"state":"missing"}),
        Err(_) => json!({"state":"unreadable"}),
    }
}

fn field_paths(value: &Value, prefix: &str, fields: &mut Vec<String>) {
    if let Some(object) = value.as_object() {
        for (name, value) in object {
            let path = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{prefix}.{name}")
            };
            field_paths(value, &path, fields);
        }
    } else {
        fields.push(prefix.to_owned());
    }
}
