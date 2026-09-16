use std::mem::discriminant;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Check {
    Pid(u32),
    PidFile(PathBuf),
    Port { host: String, port: u16 },
    Name(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ParsedStatus {
    pub(super) check: Check,
    pub(super) json: bool,
}

pub(super) fn parse_status_args(args: &[String]) -> Result<ParsedStatus> {
    let mut json = false;
    let mut host = String::from("127.0.0.1");
    let mut target = None;
    let mut conflicting_targets = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                json = true;
                i += 1;
            }
            "--pid" => {
                let value = take_value(args, i, "--pid")?;
                set_target(
                    &mut target,
                    &mut conflicting_targets,
                    Check::Pid(parse_pid(value)?),
                );
                i += 2;
            }
            "--pid-file" => {
                let value = take_value(args, i, "--pid-file")?;
                set_target(
                    &mut target,
                    &mut conflicting_targets,
                    Check::PidFile(PathBuf::from(value)),
                );
                i += 2;
            }
            "--host" => {
                host = take_value(args, i, "--host")?.to_string();
                validate_loopback_host(&host)?;
                i += 2;
            }
            "--port" => {
                let value = take_value(args, i, "--port")?;
                set_target(
                    &mut target,
                    &mut conflicting_targets,
                    Check::Port {
                        host: host.clone(),
                        port: parse_port(value)?,
                    },
                );
                i += 2;
            }
            "--name" => {
                let value = take_value(args, i, "--name")?;
                if value.trim().is_empty() {
                    return Err(anyhow!("--name cannot be empty"));
                }
                set_target(
                    &mut target,
                    &mut conflicting_targets,
                    Check::Name(value.to_string()),
                );
                i += 2;
            }
            other => {
                return Err(anyhow!(
                    "unknown process status option '{other}'\n\n{}",
                    status_usage()
                ));
            }
        }
    }

    if conflicting_targets {
        return Err(anyhow!(
            "process status requires exactly one of --pid, --pid-file, --port, or --name\n\n{}",
            status_usage()
        ));
    }

    let mut check = target.ok_or_else(|| {
        anyhow!(
            "process status requires exactly one of --pid, --pid-file, --port, or --name\n\n{}",
            status_usage()
        )
    })?;
    if let Check::Port {
        host: target_host, ..
    } = &mut check
    {
        *target_host = host;
    }

    Ok(ParsedStatus { check, json })
}

fn set_target(target: &mut Option<Check>, conflict: &mut bool, next: Check) {
    if target
        .as_ref()
        .is_some_and(|current| discriminant(current) != discriminant(&next))
    {
        *conflict = true;
    }
    *target = Some(next);
}

fn take_value<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .filter(|value| !value.starts_with("--"))
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

pub(super) fn parse_pid(value: &str) -> Result<u32> {
    let pid: u32 = value
        .parse()
        .with_context(|| format!("invalid --pid value '{value}'"))?;
    if pid == 0 {
        return Err(anyhow!("--pid must be greater than zero"));
    }
    Ok(pid)
}

fn parse_port(value: &str) -> Result<u16> {
    let port: u16 = value
        .parse()
        .with_context(|| format!("invalid --port value '{value}'"))?;
    if port == 0 {
        return Err(anyhow!("--port must be greater than zero"));
    }
    Ok(port)
}

fn validate_loopback_host(host: &str) -> Result<()> {
    match host {
        "127.0.0.1" | "::1" | "localhost" => Ok(()),
        "" => Err(anyhow!("--host cannot be empty")),
        other => Err(anyhow!(
            "--host must be a loopback target (127.0.0.1, ::1, or localhost); got '{other}'"
        )),
    }
}

pub(super) fn status_usage() -> &'static str {
    "Usage:
  adl-process status --pid <pid> [--json]
  adl-process status --pid-file <path> [--json]
  adl-process status --port <port> [--host <host>] [--json]
  adl-process status --name <label> [--json]

Compatibility:
  adl process status --pid <pid> [--json]
  adl process status --pid-file <path> [--json]
  adl process status --port <port> [--host <host>] [--json]
  adl process status --name <label> [--json]

Notes:
  The helper classifies exact metadata targets only. It does not run ps, pgrep, lsof, or broad process scans."
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    fn error(values: &[&str]) -> String {
        parse_status_args(&args(values))
            .expect_err("arguments should be rejected")
            .to_string()
    }

    #[test]
    fn accepts_each_target_and_preserves_port_host_order() {
        assert_eq!(
            parse_status_args(&args(&["--pid", "7", "--json"])).unwrap(),
            ParsedStatus {
                check: Check::Pid(7),
                json: true,
            }
        );
        assert_eq!(
            parse_status_args(&args(&["--pid-file", "server.pid"]))
                .unwrap()
                .check,
            Check::PidFile(PathBuf::from("server.pid"))
        );
        assert_eq!(
            parse_status_args(&args(&["--host", "localhost", "--port", "8787"]))
                .unwrap()
                .check,
            Check::Port {
                host: "localhost".to_string(),
                port: 8787,
            }
        );
        assert_eq!(
            parse_status_args(&args(&["--port", "8787", "--host", "::1"]))
                .unwrap()
                .check,
            Check::Port {
                host: "::1".to_string(),
                port: 8787,
            }
        );
        assert_eq!(
            parse_status_args(&args(&["--name", "demo"])).unwrap().check,
            Check::Name("demo".to_string())
        );
    }

    #[test]
    fn repeated_options_keep_the_last_value_without_creating_a_conflict() {
        assert_eq!(
            parse_status_args(&args(&["--pid", "7", "--pid", "9", "--json", "--json"])).unwrap(),
            ParsedStatus {
                check: Check::Pid(9),
                json: true,
            }
        );
        assert_eq!(
            parse_status_args(&args(&[
                "--host",
                "localhost",
                "--port",
                "80",
                "--port",
                "443",
                "--host",
                "127.0.0.1",
            ]))
            .unwrap()
            .check,
            Check::Port {
                host: "127.0.0.1".to_string(),
                port: 443,
            }
        );
    }

    #[test]
    fn rejects_missing_unknown_and_conflicting_targets_with_existing_messages() {
        assert_eq!(error(&["--pid"]), "--pid requires a value");
        assert_eq!(error(&["--pid", "--json"]), "--pid requires a value");
        assert!(error(&[]).starts_with("process status requires exactly one of --pid"));
        assert!(error(&["--pid", "7", "--port", "8787"])
            .starts_with("process status requires exactly one of --pid"));
        assert!(error(&["--wat", "now"]).starts_with("unknown process status option '--wat'"));
    }

    #[test]
    fn reports_value_errors_before_a_preceding_target_conflict() {
        assert_eq!(
            error(&["--pid", "7", "--port", "0"]),
            "--port must be greater than zero"
        );
        assert_eq!(error(&["--pid", "7", "--port"]), "--port requires a value");
        assert!(error(&["--pid", "7", "--port", "8", "--wat"])
            .starts_with("unknown process status option '--wat'"));
    }

    #[test]
    fn validates_numeric_boundaries_names_and_loopback_hosts() {
        assert_eq!(
            parse_status_args(&args(&["--pid", &u32::MAX.to_string()]))
                .unwrap()
                .check,
            Check::Pid(u32::MAX)
        );
        assert_eq!(
            parse_status_args(&args(&["--port", &u16::MAX.to_string()]))
                .unwrap()
                .check,
            Check::Port {
                host: "127.0.0.1".to_string(),
                port: u16::MAX,
            }
        );
        assert_eq!(error(&["--pid", "0"]), "--pid must be greater than zero");
        assert_eq!(error(&["--port", "0"]), "--port must be greater than zero");
        assert_eq!(error(&["--name", ""]), "--name cannot be empty");
        assert_eq!(error(&["--host", ""]), "--host cannot be empty");
        assert!(error(&["--port", "8787", "--host", "0.0.0.0"])
            .starts_with("--host must be a loopback target"));
    }

    #[test]
    fn host_without_port_remains_accepted_and_ignored_for_other_targets() {
        assert_eq!(
            parse_status_args(&args(&["--host", "localhost", "--pid", "7"]))
                .unwrap()
                .check,
            Check::Pid(7)
        );
    }
}
