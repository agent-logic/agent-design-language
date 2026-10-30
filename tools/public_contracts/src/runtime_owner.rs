use super::*;
use anyhow::ensure;
fn call(
    f: &RuntimeFixture,
    binary: &Path,
    args: &[&str],
    identity: &str,
    prefix: &Path,
    execute: bool,
) -> Result<Output> {
    let mut e = env();
    if execute {
        e.insert("PROBE_EXEC".into(), "1".into());
    }
    let mut argv = vec!["runtime", prefix.to_str().context("prefix")?, identity];
    argv.extend(args);
    invoke(binary, &f.root, &argv, &e)
}
fn refused(o: Output) -> Result<()> {
    ensure!(
        o.status.code() == Some(2) && o.stdout.is_empty() && !text(&o).1.contains("fixture-stderr"),
        "owner refusal: {o:?}"
    );
    Ok(())
}
pub fn run(binary: &Path) -> Result<Value> {
    let mut skipped = Vec::new();
    let f = RuntimeFixture::new()?;
    let o = call(
        &f,
        binary,
        &["run", "space ; $(literal).yaml", "--out", "out path"],
        &f.identity,
        &f.prefix,
        false,
    )?;
    ensure!(
        o.status.code() == Some(17)
            && text(&o)
                == (
                    "run\nspace ; $(literal).yaml\n--out\nout path\n".into(),
                    "fixture-stderr\n".into()
                )
    );
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        let o = call(
            &f,
            binary,
            &["space ; $(literal)"],
            &f.identity,
            &f.prefix,
            true,
        )?;
        ensure!(
            o.status.code() == Some(17)
                && text(&o) == ("space ; $(literal)\n".into(), "fixture-stderr\n".into())
        );
        executable(
            &f.executable,
            "#!/bin/sh\npwd\nprintf '%s\\n' \"$PROBE_EXEC\"\nkill -TERM $$\n",
        )?;
        let o = call(&f, binary, &[], &f.identity, &f.prefix, true)?;
        ensure!(
            o.status.signal() == Some(15)
                && text(&o) == (format!("{}\n1\n", f.root.display()), "".into())
        );
    }
    let f = RuntimeFixture::new()?;
    refused(call(&f, binary, &[], "", &f.prefix, false)?)?;
    refused(call(&f, binary, &[], &"c".repeat(64), &f.prefix, false)?)?;
    fs::write(f.package.join("archive.sha256"), "c".repeat(64))?;
    refused(call(&f, binary, &[], &f.identity, &f.prefix, false)?)?;
    let mut f = RuntimeFixture::new()?;
    for (key, value) in [("target", "foreign-target"), ("schema", "foreign-package")] {
        let old = f.manifest[key].clone();
        f.manifest[key] = json!(value);
        f.write()?;
        refused(call(&f, binary, &[], &f.identity, &f.prefix, false)?)?;
        f.manifest[key] = old;
    }
    f.manifest["files"]["bin/adl-runtime"]["sha256"] = json!("invalid");
    f.write()?;
    refused(call(&f, binary, &[], &f.identity, &f.prefix, false)?)?;
    #[cfg(unix)]
    {
        let f = RuntimeFixture::new()?;
        let alias = f.root.join("alias");
        std::os::unix::fs::symlink(&f.prefix, &alias)?;
        refused(call(&f, binary, &[], &f.identity, &alias, false)?)?;
        fs::remove_file(f.prefix.join("current"))?;
        fs::create_dir(f.prefix.join("current"))?;
        refused(call(&f, binary, &[], &f.identity, &f.prefix, false)?)?;
    }
    let f = RuntimeFixture::new()?;
    fs::remove_file(f.package.join("archive.tar"))?;
    refused(call(&f, binary, &[], &f.identity, &f.prefix, false)?)?;
    fs::write(f.package.join("archive.tar"), "synthetic")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for mode in [0o644, 0o4755] {
            fs::set_permissions(&f.executable, fs::Permissions::from_mode(mode))?;
            if fs::metadata(&f.executable)?.permissions().mode() & 0o7777 != mode {
                skipped.push(format!(
                    "mode {mode:o}: filesystem does not retain requested permission bits"
                ));
                continue;
            }
            refused(call(&f, binary, &[], &f.identity, &f.prefix, false)?)?;
        }
    }
    fs::remove_file(&f.executable)?;
    refused(call(&f, binary, &[], &f.identity, &f.prefix, false)?)?;
    #[cfg(unix)]
    {
        let f = RuntimeFixture::new()?;
        for name in [
            "manifest.json",
            "archive.sha256",
            "archive.tar",
            "bin/adl-runtime",
        ] {
            let path = f.package.join(name);
            let saved = f.root.join(format!(
                "saved-{}",
                path.file_name().unwrap().to_string_lossy()
            ));
            fs::rename(&path, &saved)?;
            std::os::unix::fs::symlink(&saved, &path)?;
            refused(call(&f, binary, &[], &f.identity, &f.prefix, false)?)?;
            fs::remove_file(&path)?;
            fs::rename(&saved, &path)?;
        }
    }
    Ok(
        json!({"status":if skipped.is_empty(){"passed"}else{"passed_with_platform_limitations"},"case_groups":7,"skipped_subcases":skipped,"scope":"synthetic Runtime selection, not qualification"}),
    )
}
