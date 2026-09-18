//! Installed agent entrypoint: secrets enter through private files, never argv.
use adl::codefriend::agent::{Journal, Pairing, Transport};
use anyhow::{ensure, Result};
use std::os::unix::fs::PermissionsExt;
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn private_bytes(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(path)?;
    ensure!(
        m.is_file() && m.permissions().mode() & 0o077 == 0 && m.len() <= limit,
        "agent_private_file_required"
    );
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take(limit + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= limit, "agent_file_limit");
    Ok(bytes)
}
trait AgentIo {
    fn pair(&self, origin: &str, code: &str) -> Result<Pairing>;
    fn poll(&self, origin: &str, journal: &Journal, consent: &Path) -> Result<Option<String>>;
    fn unpair(&self, origin: &str, journal: &Journal) -> Result<()>;
}
struct Network;
impl AgentIo for Network {
    fn pair(&self, origin: &str, code: &str) -> Result<Pairing> {
        Transport::new(origin)?.pair(code, now())
    }
    fn poll(&self, origin: &str, journal: &Journal, consent: &Path) -> Result<Option<String>> {
        Transport::new(origin)?.poll_once(journal, consent)
    }
    fn unpair(&self, origin: &str, journal: &Journal) -> Result<()> {
        Transport::new(origin)?.unpair(journal)
    }
}
fn run_with(mut args: impl Iterator<Item = String>, io: &impl AgentIo) -> Result<()> {
    let command = args.next().unwrap_or_default();
    if command == "--help" {
        println!("codefriend-agent pair --store PATH --origin HTTPS_ORIGIN --code-file PRIVATE_FILE\ncodefriend-agent run|once --store PATH --consent PRIVATE_FILE\ncodefriend-agent unpair|forget --store PATH");
        return Ok(());
    }
    let mut flags = BTreeMap::new();
    while let Some(key) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| anyhow::anyhow!("agent_argument_value"))?;
        ensure!(
            flags.insert(key, value).is_none(),
            "agent_duplicate_argument"
        );
    }
    let store = flags
        .remove("--store")
        .ok_or_else(|| anyhow::anyhow!("agent_store_required"))?;
    let journal = Journal::open(Path::new(&store))?;
    journal.expire(now())?;
    match command.as_str() {
        "pair" => {
            let origin = flags
                .remove("--origin")
                .ok_or_else(|| anyhow::anyhow!("agent_origin_required"))?;
            let file = flags
                .remove("--code-file")
                .ok_or_else(|| anyhow::anyhow!("agent_code_file_required"))?;
            ensure!(flags.is_empty(), "agent_unknown_argument");
            let bytes = private_bytes(Path::new(&file), 1024)?;
            let code = std::str::from_utf8(&bytes)?.trim();
            // Do not exchange another code if this installation is already paired.
            ensure!(
                !Path::new(&store).join("pairing.json").exists(),
                "agent_already_paired"
            );
            let pairing = io.pair(&origin, code)?;
            journal.save_pairing(&pairing, now())?;
            println!("{{\"status\":\"paired\"}}");
        }
        "run" | "once" => {
            let file = flags
                .remove("--consent")
                .ok_or_else(|| anyhow::anyhow!("agent_consent_file_required"))?;
            ensure!(flags.is_empty(), "agent_unknown_argument");
            let pairing = journal.pairing(now())?;
            loop {
                // Re-read local consent every poll; removing it stops all future dispatch.
                let result = io.poll(&pairing.origin, &journal, Path::new(&file));
                match &result {
                    Ok(Some(_)) => println!("{{\"status\":\"run_reported\"}}"),
                    Ok(None) => {}
                    Err(_) => eprintln!("adl_event=agent_poll_failed_no_automatic_dispatch_retry"),
                }
                if command == "once" {
                    result?;
                    break;
                }
                std::thread::sleep(Duration::from_secs(2));
            }
        }
        "forget" => {
            ensure!(flags.is_empty(), "agent_unknown_argument");
            journal.forget_pairing()?;
            println!("{{\"status\":\"local_pairing_removed_remote_revocation_unconfirmed\"}}");
        }
        "unpair" => {
            ensure!(flags.is_empty(), "agent_unknown_argument");
            let pairing = journal.pairing(now())?;
            io.unpair(&pairing.origin, &journal)?;
            println!("{{\"status\":\"unpaired\"}}");
        }
        _ => anyhow::bail!("agent_command_invalid"),
    }
    Ok(())
}
fn main() {
    if run_with(std::env::args().skip(1), &Network).is_err() {
        eprintln!("adl_event=agent_command_failed");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    // PVF: agent/PVF.json; deterministic CLI dispatch boundary. Network behavior
    // is independently covered by codefriend_agent protocol integration tests.
    struct FixtureIo {
        calls: Cell<usize>,
        report: bool,
    }
    impl AgentIo for FixtureIo {
        fn pair(&self, origin: &str, code: &str) -> Result<Pairing> {
            ensure!(code == "private-code", "wrong_code");
            self.calls.set(self.calls.get() + 1);
            Ok(Pairing {
                schema: adl::codefriend::agent::PROTOCOL.into(),
                origin: origin.into(),
                agent_id: "fixture-agent".into(),
                subject: "github-123".into(),
                agent_token: "a".repeat(64),
                model_token: "b".repeat(64),
                expires_at: now() + 600,
            })
        }
        fn poll(&self, _: &str, _: &Journal, consent: &Path) -> Result<Option<String>> {
            ensure!(
                consent.file_name().is_some_and(|p| p == "consent.json"),
                "wrong_consent"
            );
            self.calls.set(self.calls.get() + 1);
            Ok(self.report.then(|| "run-one".into()))
        }
        fn unpair(&self, _: &str, journal: &Journal) -> Result<()> {
            self.calls.set(self.calls.get() + 1);
            journal.forget_pairing()
        }
    }
    #[test]
    fn cli_private_pair_once_unpair_and_argument_guards() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("agent-cli-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let store = root.join("state");
        let code = root.join("code");
        let consent = root.join("consent.json");
        fs::write(&code, "private-code\n").unwrap();
        fs::set_permissions(&code, fs::Permissions::from_mode(0o600)).unwrap();
        let io = FixtureIo {
            calls: Cell::new(0),
            report: true,
        };
        let invoke = |args: Vec<String>| run_with(args.into_iter(), &io);
        let pair_args = || {
            vec![
                "pair".into(),
                "--store".into(),
                store.display().to_string(),
                "--origin".into(),
                "https://codefriend.example/".into(),
                "--code-file".into(),
                code.display().to_string(),
            ]
        };
        invoke(vec!["--help".into()]).unwrap();
        assert!(invoke(vec!["pair".into(), "--store".into()]).is_err());
        assert!(invoke(vec!["pair".into()]).is_err());
        assert!(invoke(vec![
            "pair".into(),
            "--store".into(),
            "a".into(),
            "--store".into(),
            "b".into()
        ])
        .is_err());
        invoke(pair_args()).unwrap();
        assert_eq!(io.calls.get(), 1);
        assert!(invoke(pair_args()).is_err());
        invoke(vec![
            "once".into(),
            "--store".into(),
            store.display().to_string(),
            "--consent".into(),
            consent.display().to_string(),
        ])
        .unwrap();
        assert_eq!(io.calls.get(), 2);
        invoke(vec![
            "unpair".into(),
            "--store".into(),
            store.display().to_string(),
        ])
        .unwrap();
        assert!(!store.join("pairing.json").exists());
        invoke(pair_args()).unwrap();
        invoke(vec![
            "forget".into(),
            "--store".into(),
            store.display().to_string(),
        ])
        .unwrap();
        assert!(invoke(vec![
            "invalid".into(),
            "--store".into(),
            store.display().to_string()
        ])
        .is_err());
        fs::set_permissions(&code, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(invoke(pair_args()).is_err());
        assert_eq!(io.calls.get(), 4);
        fs::remove_dir_all(root).unwrap();
    }
}
