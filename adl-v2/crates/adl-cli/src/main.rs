use adl_compiler::compile;
use adl_language::{json_schema, parse_and_validate_json, parse_and_validate_yaml};
use clap::{Parser, Subcommand};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const SELECTOR_SCHEMA: &str = "adl.selector.v1";
const RECEIPT_SCHEMA: &str = "adl.selector.receipt.v1";

#[derive(Parser)]
#[command(name = "adl-v2", version, about = "Thin ADL v2 owner CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Validate {
        input: PathBuf,
        #[arg(long)]
        yaml: bool,
    },
    Schema,
    Plan {
        input: PathBuf,
        #[arg(long)]
        yaml: bool,
    },
    Run {
        input: PathBuf,
        #[arg(long)]
        yaml: bool,
    },
    Inspect {
        #[arg(long)]
        root: Option<PathBuf>,
    },
    Sign {
        input: PathBuf,
    },
    Verify {
        input: PathBuf,
    },
    Select {
        generation: String,
        #[arg(long)]
        root: Option<PathBuf>,
    },
    Rollback {
        #[arg(long)]
        root: Option<PathBuf>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Selector {
    schema: String,
    current: Option<Selection>,
    previous: Option<Selection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Selection {
    generation: String,
    executable: String,
    digest: String,
    receipt: String,
}

#[derive(Debug, Serialize)]
struct Envelope<T: Serialize> {
    schema: &'static str,
    ok: bool,
    result: T,
}

fn main() {
    if let Err(error) = dispatch(Cli::parse().command) {
        eprintln!("adl-v2: {error}");
        std::process::exit(2);
    }
}

fn dispatch(command: Command) -> Result<(), String> {
    match command {
        Command::Validate { input, yaml } => {
            let document = read_document(&input, yaml)?;
            print_json(&Envelope {
                schema: "adl.validate.v1",
                ok: true,
                result: document,
            })
        }
        Command::Schema => print_json(&Envelope {
            schema: "adl.schema.v1",
            ok: true,
            result: json_schema(),
        }),
        Command::Plan { input, yaml } => {
            let document = read_document(&input, yaml)?;
            let plan = compile(&document).map_err(|errors| {
                errors
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            })?;
            print_json(&Envelope {
                schema: "adl.plan.v1",
                ok: true,
                result: plan,
            })
        }
        Command::Run { input, yaml } => {
            let document = read_document(&input, yaml)?;
            let plan = compile(&document).map_err(|errors| {
                errors
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            })?;
            let result = serde_json::json!({"contract": adl_engine::ENGINE_CONTRACT_VERSION, "status": "planned", "plan": plan});
            print_json(&Envelope {
                schema: "adl.run.v1",
                ok: true,
                result,
            })
        }
        Command::Inspect { root } => print_json(&Envelope {
            schema: SELECTOR_SCHEMA,
            ok: true,
            result: load_selector(root.as_deref())?,
        }),
        Command::Sign { input } => print_json(&Envelope {
            schema: "adl.sign.v1",
            ok: true,
            result: digest_file(&input)?,
        }),
        Command::Verify { input } => print_json(&Envelope {
            schema: "adl.verify.v1",
            ok: true,
            result: digest_file(&input)?,
        }),
        Command::Select { generation, root } => mutate_selector(root.as_deref(), generation, false),
        Command::Rollback { root } => mutate_selector(root.as_deref(), String::new(), true),
    }
}

fn read_document(path: &Path, yaml: bool) -> Result<adl_language::AdlDocument, String> {
    let source = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if yaml {
        parse_and_validate_yaml(&source).map_err(format_diagnostics)
    } else {
        parse_and_validate_json(&source).map_err(format_diagnostics)
    }
}

fn format_diagnostics(diagnostics: Vec<adl_language::Diagnostic>) -> String {
    serde_json::to_string(&diagnostics).unwrap_or_else(|_| "validation failed".into())
}

fn print_json<T: Serialize>(value: &T) -> Result<(), String> {
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, value).map_err(|e| e.to_string())?;
    stdout.write_all(b"\n").map_err(|e| e.to_string())
}

fn selector_root(root: Option<&Path>) -> PathBuf {
    root.map(Path::to_path_buf)
        .or_else(|| std::env::var_os("ADL_DATA_ROOT").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from(".adl-v2"))
}

fn selector_path(root: Option<&Path>) -> PathBuf {
    selector_root(root).join("selector.json")
}

fn load_selector(root: Option<&Path>) -> Result<Selector, String> {
    let path = selector_path(root);
    if !path.exists() {
        return Ok(Selector {
            schema: SELECTOR_SCHEMA.into(),
            current: None,
            previous: None,
        });
    }
    serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

fn digest_file(path: &Path) -> Result<serde_json::Value, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    Ok(serde_json::json!({"path": path, "sha256": format!("{:x}", Sha256::digest(bytes))}))
}

fn mutate_selector(root: Option<&Path>, generation: String, rollback: bool) -> Result<(), String> {
    let root = selector_root(root);
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let lock = File::create(root.join("selector.lock")).map_err(|e| e.to_string())?;
    lock.lock_exclusive().map_err(|e| e.to_string())?;
    let mut selector = load_selector(Some(&root))?;
    let next = if rollback {
        selector
            .previous
            .clone()
            .ok_or_else(|| "no verified previous generation".to_string())?
    } else {
        let executable = root.join(&generation);
        if !executable.is_file() {
            return Err(format!(
                "generation executable is missing: {}",
                executable.display()
            ));
        }
        let digest = digest_file(&executable)?["sha256"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        Selection {
            generation: generation.clone(),
            executable: executable.display().to_string(),
            digest: digest.clone(),
            receipt: format!("sha256:{digest}"),
        }
    };
    selector.previous = selector.current.take();
    selector.current = Some(next.clone());
    let bytes = serde_json::to_vec_pretty(&selector).map_err(|e| e.to_string())?;
    let mut temp = tempfile::NamedTempFile::new_in(&root).map_err(|e| e.to_string())?;
    temp.write_all(&bytes).map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    temp.persist(selector_path(Some(&root)))
        .map_err(|e| e.error.to_string())?;
    print_json(&Envelope {
        schema: RECEIPT_SCHEMA,
        ok: true,
        result: serde_json::json!({"selection": next, "rollback": rollback}),
    })
}
