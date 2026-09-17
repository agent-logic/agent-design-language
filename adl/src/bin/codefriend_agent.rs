//! Installed agent entrypoint: secrets enter through private files, never argv.
use adl::codefriend::agent::{Journal, Transport};
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
fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
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
            let pairing = Transport::new(&origin)?.pair(code, now())?;
            journal.save_pairing(&pairing, now())?;
            println!("{{\"status\":\"paired\"}}");
        }
        "run" | "once" => {
            let file = flags
                .remove("--consent")
                .ok_or_else(|| anyhow::anyhow!("agent_consent_file_required"))?;
            ensure!(flags.is_empty(), "agent_unknown_argument");
            let pairing = journal.pairing(now())?;
            let transport = Transport::new(&pairing.origin)?;
            loop {
                // Re-read local consent every poll; removing it stops all future dispatch.
                let result = transport.poll_once(&journal, Path::new(&file));
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
            Transport::new(&pairing.origin)?.unpair(&journal)?;
            println!("{{\"status\":\"unpaired\"}}");
        }
        _ => anyhow::bail!("agent_command_invalid"),
    }
    Ok(())
}
fn main() {
    if run().is_err() {
        eprintln!("adl_event=agent_command_failed");
        std::process::exit(1);
    }
}
