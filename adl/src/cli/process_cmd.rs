use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use serde::Serialize;

#[path = "process_cmd/args.rs"]
mod args;

use args::{parse_pid, parse_status_args, status_usage, Check};

const SCHEMA: &str = "adl.process_status.v1";
const MAX_PID_FILE_BYTES: u64 = 64;
const PORT_CONNECT_TIMEOUT: Duration = Duration::from_millis(150);

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct ProcessStatusReport {
    schema: &'static str,
    check: &'static str,
    status: &'static str,
    pid: Option<u32>,
    host: Option<String>,
    port: Option<u16>,
    safe_order: &'static str,
    broad_process_scan: bool,
    uses_ps: bool,
    note: &'static str,
}

pub(crate) fn real_process(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("status") => real_process_status(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", process_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown process command '{other}'\n\n{}",
            process_usage()
        )),
    }
}

fn real_process_status(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", status_usage());
        return Ok(());
    }

    let parsed = parse_status_args(args)?;
    let report = classify_status(parsed.check)?;

    if parsed.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).context("serialize process status report")?
        );
    } else {
        print_human_report(&report);
    }

    Ok(())
}

fn classify_status(check: Check) -> Result<ProcessStatusReport> {
    match check {
        Check::Pid(pid) => Ok(pid_report("pid", pid, pid_is_live(pid))),
        Check::PidFile(path) => match read_pid_file(&path) {
            Ok(raw) => {
                let trimmed = raw.trim();
                match parse_pid(trimmed) {
                    Ok(pid) => Ok(pid_report("pid_file", pid, pid_is_live(pid))),
                    Err(_) => Ok(base_report(
                        "pid_file",
                        "invalid_metadata",
                        None,
                        None,
                        None,
                        "pid metadata exists but is not a positive integer",
                    )),
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(base_report(
                "pid_file",
                "missing_metadata",
                None,
                None,
                None,
                "pid metadata was not present; no process scan was attempted",
            )),
            Err(_) => Ok(base_report(
                "pid_file",
                "unknown",
                None,
                None,
                None,
                "pid metadata could not be read; no process scan was attempted",
            )),
        },
        Check::Port { host, port } => Ok(port_report(host, port)),
        Check::Name(_name) => Ok(base_report(
            "name",
            "unknown",
            None,
            None,
            None,
            "process-name lookup is intentionally not scanned by this helper",
        )),
    }
}

fn read_pid_file(path: &PathBuf) -> std::io::Result<String> {
    let metadata = fs::metadata(path)?;
    if !metadata.file_type().is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "pid metadata path is not a regular file",
        ));
    }
    let file = fs::File::open(path)?;
    let mut limited = file.take(MAX_PID_FILE_BYTES + 1);
    let mut raw = String::new();
    limited.read_to_string(&mut raw)?;
    if raw.len() as u64 > MAX_PID_FILE_BYTES {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "pid metadata exceeds bounded read limit",
        ));
    }
    Ok(raw)
}

fn pid_report(check: &'static str, pid: u32, live: Option<bool>) -> ProcessStatusReport {
    match live {
        Some(true) => base_report(
            check,
            "live_pid",
            Some(pid),
            None,
            None,
            "known pid responded to a bounded liveness probe",
        ),
        Some(false) => base_report(
            check,
            "stale_pid",
            Some(pid),
            None,
            None,
            "known pid did not respond to a bounded liveness probe",
        ),
        None => base_report(
            check,
            "unknown",
            Some(pid),
            None,
            None,
            "bounded pid liveness probe was unavailable",
        ),
    }
}

fn port_report(host: String, port: u16) -> ProcessStatusReport {
    if port_accepts_tcp_connection(&host, port) {
        return base_report(
            "port",
            "bound_port",
            None,
            Some(host),
            Some(port),
            "exact TCP connect probe reached a loopback service",
        );
    }

    match TcpListener::bind((host.as_str(), port)) {
        Ok(listener) => {
            drop(listener);
            base_report(
                "port",
                "unbound_port",
                None,
                Some(host),
                Some(port),
                "local bind probe succeeded",
            )
        }
        Err(err) if err.kind() == std::io::ErrorKind::AddrInUse => base_report(
            "port",
            "bound_port",
            None,
            Some(host),
            Some(port),
            "local bind probe found the address already in use",
        ),
        Err(_) => base_report(
            "port",
            "unknown",
            None,
            Some(host),
            Some(port),
            "local bind probe could not classify the port",
        ),
    }
}

fn port_accepts_tcp_connection(host: &str, port: u16) -> bool {
    match (host, port).to_socket_addrs() {
        Ok(addrs) => addrs
            .filter(|addr| addr.ip().is_loopback())
            .any(|addr| TcpStream::connect_timeout(&addr, PORT_CONNECT_TIMEOUT).is_ok()),
        Err(_) => false,
    }
}

fn base_report(
    check: &'static str,
    status: &'static str,
    pid: Option<u32>,
    host: Option<String>,
    port: Option<u16>,
    note: &'static str,
) -> ProcessStatusReport {
    ProcessStatusReport {
        schema: SCHEMA,
        check,
        status,
        pid,
        host,
        port,
        safe_order: "metadata_or_exact_target_first",
        broad_process_scan: false,
        uses_ps: false,
        note,
    }
}

#[cfg(unix)]
fn pid_is_live(pid: u32) -> Option<bool> {
    const EPERM: i32 = 1;
    const ESRCH: i32 = 3;

    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }

    if pid > i32::MAX as u32 {
        return Some(false);
    }

    let result = unsafe { kill(pid as i32, 0) };
    if result == 0 {
        return Some(true);
    }

    match std::io::Error::last_os_error().raw_os_error() {
        Some(EPERM) => Some(true),
        Some(ESRCH) => Some(false),
        _ => None,
    }
}

#[cfg(not(unix))]
fn pid_is_live(_pid: u32) -> Option<bool> {
    None
}

fn print_human_report(report: &ProcessStatusReport) {
    match (report.pid, report.host.as_deref(), report.port) {
        (Some(pid), _, _) => println!("{} pid={pid}", report.status),
        (_, Some(host), Some(port)) => println!("{} {host}:{port}", report.status),
        _ => println!("{}", report.status),
    }
}

fn process_usage() -> &'static str {
    "Usage:
  adl-process status (--pid <pid> | --pid-file <path> | --port <port> [--host <host>] | --name <label>) [--json]

Compatibility:
  adl process status (--pid <pid> | --pid-file <path> | --port <port> [--host <host>] | --name <label>) [--json]"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::thread;

    #[test]
    fn port_report_classifies_reachable_loopback_service_as_bound() {
        assert_reachable_loopback_service_reports_bound("127.0.0.1");
    }

    #[test]
    fn port_report_classifies_reachable_localhost_service_as_bound() {
        assert_reachable_loopback_service_reports_bound("localhost");
    }

    fn assert_reachable_loopback_service_reports_bound(host: &str) {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind local test listener");
        let port = listener.local_addr().expect("local listener addr").port();

        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept process-status probe");
            let _ = stream.write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n");
        });

        let report = port_report(host.to_string(), port);

        server.join().expect("test server should finish");
        assert_eq!(report.status, "bound_port");
        assert_eq!(
            report.note,
            "exact TCP connect probe reached a loopback service"
        );
        assert!(!report.broad_process_scan);
        assert!(!report.uses_ps);
    }

    #[test]
    fn port_report_preserves_unbound_status_for_free_loopback_port() {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind local test listener");
        let port = listener.local_addr().expect("local listener addr").port();
        drop(listener);

        let report = port_report("127.0.0.1".to_string(), port);

        assert_eq!(report.status, "unbound_port");
        assert_eq!(report.note, "local bind probe succeeded");
        assert!(!report.broad_process_scan);
        assert!(!report.uses_ps);
    }
}
