//! Legacy workflow argument contract, independent of private execution modules.
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub(super) struct WorkflowArgs {
    pub(super) adl_path: PathBuf,
    pub(super) print_plan: bool,
    pub(super) print_prompts: bool,
    pub(super) do_trace: bool,
    pub(super) do_run: bool,
    pub(super) out_dir: PathBuf,
    pub(super) quiet: bool,
    pub(super) do_open: bool,
    pub(super) allow_unsigned: bool,
    pub(super) allow_embedded_signature_key: bool,
    pub(super) signature_key_path: Option<PathBuf>,
    pub(super) resume_path: Option<PathBuf>,
    pub(super) steer_path: Option<PathBuf>,
    pub(super) overlay_path: Option<PathBuf>,
}

// None means help was printed. Invalid arguments retain the legacy exit-2 contract.
pub(super) fn parse(args: &[String], usage: &str) -> Option<WorkflowArgs> {
    let adl_path: PathBuf = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("missing ADL yaml path");
            eprintln!("Try: adl examples/v0-3-concurrency-fork-join.adl.yaml --print-plan");
            eprintln!("{}", usage);
            std::process::exit(2);
        }
    };

    let mut print_plan = false;
    let mut print_prompts = false;
    let mut do_trace = false;
    let mut do_run = false;
    let mut out_dir = PathBuf::from("out");
    let mut quiet = false;
    let mut do_open = false;
    let mut allow_unsigned = false;
    let mut allow_embedded_signature_key = false;
    let mut signature_key_path: Option<PathBuf> = None;
    let mut resume_path: Option<PathBuf> = None;
    let mut steer_path: Option<PathBuf> = None;
    let mut overlay_path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        let a = &args[i];
        match a.as_str() {
            "--print-plan" => print_plan = true,
            "--print-prompts" | "--print-prompt" => print_prompts = true,
            "--trace" => do_trace = true,
            "--run" => do_run = true,
            "--resume" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("--resume requires a run.json path");
                    eprintln!("{}", usage);
                    std::process::exit(2);
                };
                resume_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--steer" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("--steer requires a path to steering patch JSON");
                    eprintln!("{}", usage);
                    std::process::exit(2);
                };
                steer_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--overlay" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("--overlay requires a path to overlay.json");
                    eprintln!("{}", usage);
                    std::process::exit(2);
                };
                overlay_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--out" => {
                let Some(dir) = args.get(i + 1) else {
                    eprintln!("--out requires a directory path");
                    eprintln!("{}", usage);
                    std::process::exit(2);
                };
                out_dir = PathBuf::from(dir);
                i += 1;
            }
            "--quiet" | "--no-step-output" => quiet = true,
            "--open" | "--open-artifacts" => do_open = true,
            "--allow-unsigned" => allow_unsigned = true,
            "--allow-embedded-signature-key" => allow_embedded_signature_key = true,
            "--signature-key" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("--signature-key requires a public key path");
                    eprintln!("{}", usage);
                    std::process::exit(2);
                };
                signature_key_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage);
                return None;
            }
            _ => {
                eprintln!("Unknown arg: {a}");
                eprintln!("Run 'adl --help' for usage.");
                eprintln!("{}", usage);
                std::process::exit(2);
            }
        }
        i += 1;
    }

    Some(WorkflowArgs {
        adl_path,
        print_plan,
        print_prompts,
        do_trace,
        do_run,
        out_dir,
        quiet,
        do_open,
        allow_unsigned,
        allow_embedded_signature_key,
        signature_key_path,
        resume_path,
        steer_path,
        overlay_path,
    })
}
