//! Legacy ADL resume argument and diagnostic contract, independent of execution.
use std::path::PathBuf;

pub(super) struct ResumeArgs<'a> {
    pub run_id: &'a str,
    pub adl_path: Option<PathBuf>,
    pub steer_path: Option<PathBuf>,
}

pub(super) fn parse<'a>(args: &'a [String], usage: &str) -> Option<ResumeArgs<'a>> {
    if matches!(args.first().map(|s| s.as_str()), Some("--help" | "-h")) {
        println!("{}", usage);
        return None;
    }

    let Some(run_id) = args.first() else {
        eprintln!("resume requires <run_id>");
        eprintln!("{}", usage);
        std::process::exit(2);
    };
    let mut adl_path: Option<PathBuf> = None;
    let mut steer_path: Option<PathBuf> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--adl" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("resume --adl requires a path to the ADL document");
                    eprintln!("{}", usage);
                    std::process::exit(2);
                };
                adl_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--steer" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("resume --steer requires a path to steering patch JSON");
                    eprintln!("{}", usage);
                    std::process::exit(2);
                };
                steer_path = Some(PathBuf::from(path));
                i += 1;
            }
            other => {
                if args.len() > 1
                    && args
                        .iter()
                        .skip(1)
                        .all(|arg| arg != "--adl" && arg != "--steer")
                {
                    eprintln!("resume requires <run_id> plus --adl <path>");
                } else {
                    eprintln!(
                        "resume accepts <run_id> --adl <path> [--steer <steering.json>] (unknown arg: {other})"
                    );
                }
                eprintln!("{}", usage);
                std::process::exit(2);
            }
        }
        i += 1;
    }
    Some(ResumeArgs {
        run_id,
        adl_path,
        steer_path,
    })
}
