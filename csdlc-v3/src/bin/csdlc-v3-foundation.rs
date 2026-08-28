use csdlc_v3::application::FoundationState;
use csdlc_v3::repository::RepositoryContext;
use std::path::PathBuf;

fn main() {
    match run() {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("csdlc-v3-foundation: {error}");
            std::process::exit(2);
        }
    }
}

fn run() -> Result<String, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let flag = args
        .next()
        .ok_or("usage: csdlc-v3-foundation --repo-root <path>")?;
    if flag != "--repo-root" {
        return Err("usage: csdlc-v3-foundation --repo-root <path>".into());
    }
    let root = args
        .next()
        .map(PathBuf::from)
        .ok_or("usage: csdlc-v3-foundation --repo-root <path>")?;
    if args.next().is_some() {
        return Err("usage: csdlc-v3-foundation --repo-root <path>".into());
    }
    let context = RepositoryContext::discover(root)?;
    let state = FoundationState::load(&context)?;
    Ok(state.to_machine_json())
}
