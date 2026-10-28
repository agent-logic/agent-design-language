//! Native test probe for the actual public owner-selection modules.
#[allow(dead_code)] // Probe exercises explicit receipt selection; facade covers environment selection.
#[path = "../../../adl/src/cli/codefriend_owner.rs"]
mod codefriend_owner;
#[allow(dead_code)] // Probe exercises explicit receipt selection; facade covers environment selection.
#[path = "../../../adl/src/cli/runtime_owner.rs"]
mod runtime_owner;
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let result = (|| -> anyhow::Result<()> {
        match a.first().map(String::as_str) {
            Some("runtime") => {
                let selected = runtime_owner::SelectedRuntime::from_installation(
                    std::path::Path::new(&a[1]),
                    &a[2],
                )?;
                if std::env::var_os("PROBE_EXEC").is_some() {
                    selected.exec(&a[3..])?;
                } else {
                    let status = selected.invoke(&a[3..])?;
                    std::process::exit(status.code().unwrap_or(125));
                }
            }
            Some("codefriend") => {
                let selected = codefriend_owner::SelectedCodeFriend::from_installation(
                    std::path::Path::new(&a[1]),
                    &a[2],
                    &a[3],
                )?;
                if a.get(4).map(String::as_str) == Some("inspect") {
                    println!("{}", selected.command(&[]).get_program().to_string_lossy());
                } else {
                    selected.exec(&a[4..])?;
                }
            }
            _ => anyhow::bail!("owner probe requires runtime or codefriend arguments"),
        };
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("{e:#}");
        std::process::exit(2);
    }
}
