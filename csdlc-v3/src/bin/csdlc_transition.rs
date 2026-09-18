use serde_json::json;
use std::{env, path::Path};

fn run() -> Result<serde_json::Value, String> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args == ["--help"] {
        return Ok(
            json!({"commands":["inventory --repo-root PRIMARY", "reconcile-history --spec FILE [--execute --preview DIGEST]", "request --spec FILE --output FILE", "fence --request FILE", "convert --request FILE", "status --request FILE", "verify --request FILE", "restore --request FILE", "resume --request FILE --decision FILE"],"request_schema":"csdlc.v3.live_transition.v1"}),
        );
    }
    if args.first().is_some_and(|s| s == "reconcile-history") {
        return match args.as_slice() {
            [_, flag, path] if flag == "--spec" => {
                csdlc_v3::transition::history::reconcile(Path::new(path), None)
            }
            [_, flag, path, execute, preview, digest]
                if flag == "--spec" && execute == "--execute" && preview == "--preview" =>
            {
                csdlc_v3::transition::history::reconcile(Path::new(path), Some(digest))
            }
            _ => Err("usage: reconcile-history --spec FILE [--execute --preview DIGEST]".into()),
        };
    }
    if args.len() == 3 && args[0] == "inventory" && args[1] == "--repo-root" {
        return csdlc_v3::transition::inventory(Path::new(&args[2]));
    }
    if args.len() == 5 && args[0] == "request" && args[1] == "--spec" && args[3] == "--output" {
        return csdlc_v3::transition::build_request(Path::new(&args[2]), Path::new(&args[4]));
    }
    if args.len() < 3 || args[1] != "--request" {
        return Err("usage: csdlc-transition fence|convert|status|restore|resume --request FILE [--decision FILE]".into());
    }
    if args[0] == "__guardian" && args.len() == 5 && args[3] == "--phase" {
        csdlc_v3::transition::guardian(Path::new(&args[2]), &args[4])?;
        return Ok(json!({"status":"released"}));
    }
    let decision = match args.len() {
        3 => None,
        5 if args[0] == "resume" && args[3] == "--decision" => Some(Path::new(&args[4])),
        _ => return Err("unknown or duplicate arguments".into()),
    };
    csdlc_v3::transition::execute(&args[0], Path::new(&args[2]), decision)
}
fn main() {
    match run() {
        Ok(v) => println!("{v}"),
        Err(reason) => {
            println!(
                "{}",
                json!({"status":"refused","reason":reason,"automatic_retry":false})
            );
            std::process::exit(2);
        }
    }
}
