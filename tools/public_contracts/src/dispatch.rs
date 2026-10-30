use super::*;
use anyhow::ensure;
pub fn runtime(binary: &Path) -> Result<Value> {
    let f = RuntimeFixture::new()?;
    executable(&f.executable,"#!/bin/sh\nprintf 'owner\\n'\nprintf '<%s>\\n' \"$@\"\nprintf 'control=%s\\nlog=%s\\n' \"${ADL_OBSERVABILITY-unset}\" \"${ADL_OBSERVABILITY_LOG-unset}\"\nprintf 'owner-stderr\\n' >&2\nexit 17\n")?;
    let fixture = f.root.join("workflow space ; literal.yaml");
    fs::copy(
        root().join("adl/examples/v0-6-hitl-no-pause.adl.yaml"),
        &fixture,
    )?;
    let fixture = fixture.to_str().context("fixture path")?;
    let mut cases = vec![];
    let mut call =
        |name: &str, args: &[&str], extra: &[(&str, String)], qualified: bool| -> Result<Output> {
            let mut e = if qualified { f.selected() } else { env() };
            e.extend(extra.iter().map(|(k, v)| (k.to_string(), v.clone())));
            let o = invoke(binary, &f.root, args, &e)?;
            cases.push(name.to_owned());
            Ok(o)
        };
    let o = call(
        "explicit-run",
        &[fixture, "--run", "--allow-unsigned"],
        &[],
        true,
    )?;
    let (out, err) = text(&o);
    ensure!(
        o.status.code() == Some(17)
            && out.starts_with(&format!("owner\n<run>\n<{fixture}>\n"))
            && err.contains("command=adl stage=dispatch")
            && err.ends_with("owner-stderr\n"),
        "explicit dispatch: {o:?}"
    );
    let o = call("implicit-run", &[fixture], &[], true)?;
    ensure!(o.status.code() == Some(17) && text(&o).0.starts_with("owner\n<run>\n"));
    let o = call(
        "disabled-observability",
        &[fixture, "--run"],
        &[("ADL_OBSERVABILITY", "0".into())],
        true,
    )?;
    ensure!(
        o.status.code() == Some(17)
            && text(&o).0.contains("control=0\n")
            && text(&o).1 == "owner-stderr\n"
    );
    let log = f.root.join("compatibility.log");
    let o = call(
        "file-log-quiet-stderr",
        &[fixture, "--run"],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0".into()),
            ("ADL_OBSERVABILITY_LOG", log.to_string_lossy().into_owned()),
        ],
        true,
    )?;
    ensure!(
        o.status.code() == Some(17)
            && text(&o).1 == "owner-stderr\n"
            && text(&o).0.contains(&format!("log={}\n", log.display()))
    );
    ensure!(fs::read_to_string(log)?.contains("command=adl stage=dispatch"));
    let o = call("local-plan", &[fixture, "--print-plan"], &[], false)?;
    ensure!(
        o.status.success()
            && text(&o).0.starts_with("Resolved run:")
            && !text(&o).0.contains("owner\n")
    );
    let o = call("resume-help", &["resume", "--help"], &[], false)?;
    ensure!(
        o.status.success() && text(&o).0.contains("Semantics:") && !text(&o).0.contains("owner\n")
    );
    let o = call("resume-missing-id", &["resume"], &[], false)?;
    ensure!(
        o.status.code() == Some(2)
            && text(&o).1.contains("resume requires <run_id>")
            && text(&o).1.contains("Semantics:")
    );
    let o = call(
        "resume-forward",
        &["resume", "run-id", "--adl", fixture],
        &[],
        true,
    )?;
    ensure!(
        o.status.code() == Some(17)
            && text(&o)
                .0
                .starts_with("owner\n<resume>\n<run-id>\n<--adl>\n")
    );
    let o = call("missing-owner", &[fixture, "--run"], &[], false)?;
    ensure!(
        o.status.code() == Some(1)
            && o.stdout.is_empty()
            && text(&o).1.contains("ADL_RUNTIME_INSTALL_PREFIX")
    );
    Ok(
        json!({"status":"passed","count":cases.len(),"cases":cases,"limits":"Synthetic owner layout; real installed Runtime qualification recorded separately"}),
    )
}
pub fn refusals(binary: &Path) -> Result<Value> {
    let mut f = RuntimeFixture::new()?;
    let mut count = 0;
    let mut refused =
        |args: &[&str], message: &str, code: i32, extra: &BTreeMap<String, String>| -> Result<()> {
            let mut e = env();
            e.insert("ADL_OBSERVABILITY".into(), "0".into());
            e.extend(extra.clone());
            let o = invoke(binary, &f.root, args, &e)?;
            ensure!(
                o.status.code() == Some(code)
                    && o.stdout.is_empty()
                    && text(&o).1.contains(message),
                "expected {message}: {o:?}"
            );
            count += 1;
            Ok(())
        };
    let empty = BTreeMap::new();
    for flag in [
        "--resume",
        "--steer",
        "--overlay",
        "--out",
        "--signature-key",
    ] {
        refused(
            &["missing.yaml", flag],
            &format!("{flag} requires"),
            2,
            &empty,
        )?;
    }
    refused(
        &["missing.yaml", "--unknown"],
        "Unknown arg: --unknown",
        2,
        &empty,
    )?;
    for flag in ["--adl", "--steer"] {
        refused(
            &["resume", "run-id", flag],
            &format!("resume {flag} requires"),
            2,
            &empty,
        )?;
    }
    refused(
        &["resume", "run-id", "unexpected"],
        "resume requires <run_id> plus --adl",
        2,
        &empty,
    )?;
    refused(
        &["resume", "run-id", "--adl", "literal", "--unknown"],
        "unknown arg: --unknown",
        2,
        &empty,
    )?;
    refused(
        &[
            "resume",
            "run-id",
            "--adl",
            "literal path",
            "--steer",
            "patch",
        ],
        "ADL_RUNTIME_INSTALL_PREFIX",
        1,
        &empty,
    )?;
    refused(
        &[
            "missing.yaml",
            "--print-plan",
            "--print-prompt",
            "--trace",
            "--run",
            "--resume",
            "run.json",
            "--steer",
            "patch",
            "--overlay",
            "overlay",
            "--out",
            "literal out",
            "--no-step-output",
            "--open-artifacts",
            "--allow-unsigned",
            "--allow-embedded-signature-key",
            "--signature-key",
            "key",
        ],
        "ADL_RUNTIME_INSTALL_PREFIX",
        1,
        &empty,
    )?;
    executable(
        &f.executable,
        &format!("#!{}\n", f.root.join("absent-interpreter").display()),
    )?;
    let selected = f.selected();
    refused(
        &["csm", "status"],
        "selected Runtime invocation failed",
        1,
        &selected,
    )?;
    for (key, value, message) in [
        (
            "ADL_RUNTIME_ARCHIVE_SHA256",
            "A".repeat(64),
            "exact lowercase archive",
        ),
        (
            "ADL_RUNTIME_INSTALL_PREFIX",
            "installed".into(),
            "absolute and unaliased",
        ),
    ] {
        let mut e = selected.clone();
        e.insert(key.into(), value);
        refused(&["csm", "status"], message, 1, &e)?;
    }
    for (key, value) in [("schema", "foreign-schema"), ("target", "foreign-target")] {
        let saved = f.manifest[key].clone();
        f.manifest[key] = json!(value);
        f.write()?;
        refused(
            &["csm", "status"],
            "schema or host target mismatch",
            1,
            &selected,
        )?;
        f.manifest[key] = saved;
    }
    for (key, value) in [("mode", json!(420)), ("sha256", json!("bad"))] {
        let saved = f.manifest["files"]["bin/adl-runtime"][key].clone();
        f.manifest["files"]["bin/adl-runtime"][key] = value;
        f.write()?;
        refused(
            &["csm", "status"],
            "absent from the qualified manifest",
            1,
            &selected,
        )?;
        f.manifest["files"]["bin/adl-runtime"][key] = saved;
    }
    f.write()?;
    fs::write(f.package.join("archive.sha256"), "c".repeat(64))?;
    refused(
        &["csm", "status"],
        "archive identity mismatch",
        1,
        &selected,
    )?;
    fs::write(f.package.join("archive.sha256"), &f.identity)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&f.executable, fs::Permissions::from_mode(0o644))?;
        refused(&["csm", "status"], "executable mode mismatch", 1, &selected)?;
        fs::set_permissions(&f.executable, fs::Permissions::from_mode(0o755))?;
    }
    let manifest = f.package.join("manifest.json");
    fs::write(&manifest, [255])?;
    refused(&["csm", "status"], "metadata is not UTF-8", 1, &selected)?;
    fs::File::create(&manifest)?.set_len(8 * 1024 * 1024 + 1)?;
    refused(&["csm", "status"], "metadata is too large", 1, &selected)?;
    f.write()?;
    let archive = f.package.join("archive.tar");
    fs::remove_file(&archive)?;
    fs::create_dir(&archive)?;
    refused(&["csm", "status"], "not a regular file", 1, &selected)?;
    fs::remove_dir(&archive)?;
    fs::write(&archive, "synthetic")?;
    #[cfg(unix)]
    {
        let current = f.prefix.join("current");
        fs::remove_file(&current)?;
        std::os::unix::fs::symlink(format!("versions/{}", "c".repeat(64)), current)?;
        refused(&["csm", "status"], "selection does not match", 1, &selected)?;
    }
    Ok(
        json!({"status":"passed","cases":count,"scope":"public diagnostics and synthetic owner refusals; no qualification claim"}),
    )
}
