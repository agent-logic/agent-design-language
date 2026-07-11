use std::{
    collections::{BTreeMap, BTreeSet},
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    process::ExitCode,
    sync::Arc,
};

use adl_runtime_kernel::{
    generate_runtime_instance_id,
    proof::{build_proof_runtime, load_capsule, run_proof},
    serve_control_listener_until, verifying_key_from_hex, ControlAuthority, ControlCapability,
    ControlService, KernelExit, TrustedControlKey, DEFAULT_CONTROL_API_PORT,
};

#[tokio::main]
async fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "demo".to_owned());
    let capsule = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("adl-runtime-kernel-continuity.json"));

    match command.as_str() {
        "serve" => {
            let proof = match build_proof_runtime(&capsule, 3) {
                Ok(proof) => proof,
                Err(error) => {
                    eprintln!("runtime topology invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let public_key = match std::env::var("ADL_RUNTIME_V3_CONTROL_PUBLIC_KEY_HEX")
                .map_err(|_| ())
                .and_then(|value| verifying_key_from_hex(&value).map_err(|_| ()))
            {
                Ok(key) => key,
                Err(()) => {
                    eprintln!("runtime control key is missing or invalid");
                    return ExitCode::from(78);
                }
            };
            let key_id = std::env::var("ADL_RUNTIME_V3_CONTROL_KEY_ID")
                .unwrap_or_else(|_| "operator".to_owned());
            let principal = std::env::var("ADL_RUNTIME_V3_CONTROL_PRINCIPAL")
                .unwrap_or_else(|_| "operator".to_owned());
            let authority = ControlAuthority::new(BTreeMap::from([(
                key_id,
                TrustedControlKey {
                    principal,
                    verifying_key: public_key,
                    capabilities: BTreeSet::from([
                        ControlCapability::Read,
                        ControlCapability::Stop,
                    ]),
                },
            )]));
            let listener = match tokio::net::TcpListener::bind((
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                DEFAULT_CONTROL_API_PORT,
            ))
            .await
            {
                Ok(listener) => listener,
                Err(error) => {
                    eprintln!("runtime control API bind failed: {error}");
                    return ExitCode::from(70);
                }
            };
            let mut handle = match proof.kernel.start().await {
                Ok(handle) => handle,
                Err(error) => {
                    eprintln!("runtime kernel failed to start: {error}");
                    return ExitCode::from(70);
                }
            };
            let instance_id = generate_runtime_instance_id();
            eprintln!(
                "adl_event schema=adl.runtime.instance.v1 event=control_ready instance_id={instance_id} port=20997"
            );
            let service = Arc::new(ControlService::new(
                instance_id,
                proof.recorder,
                handle.control(),
                authority,
                1024,
            ));
            let api_shutdown = tokio_util::sync::CancellationToken::new();
            let mut api = tokio::spawn(serve_control_listener_until(
                service,
                listener,
                api_shutdown.clone().cancelled_owned(),
            ));
            tokio::select! {
                signal = tokio::signal::ctrl_c() => {
                    if let Err(error) = signal {
                        eprintln!("runtime signal handler failed: {error}");
                        return ExitCode::from(70);
                    }
                    api_shutdown.cancel();
                    let shutdown = handle.shutdown(std::time::Duration::from_secs(10)).await;
                    drain_control_api(&mut api).await;
                    match shutdown {
                        Ok(exit) => {
                            process_exit(exit)
                        },
                        Err(error) => {
                            eprintln!("runtime shutdown failed: {error}");
                            ExitCode::from(70)
                        }
                    }
                }
                exit = handle.wait_for_exit() => match exit {
                    Ok(exit) => {
                        api_shutdown.cancel();
                        drain_control_api(&mut api).await;
                        process_exit(exit)
                    },
                    Err(error) => {
                        eprintln!("runtime kernel task failed: {error}");
                        ExitCode::from(70)
                    }
                },
                result = &mut api => {
                    match result {
                        Ok(Ok(())) => eprintln!("runtime control API stopped unexpectedly"),
                        Ok(Err(error)) => eprintln!("runtime control API failed: {error}"),
                        Err(error) => eprintln!("runtime control API task failed: {error}"),
                    }
                    let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                    ExitCode::from(70)
                },
            }
        }
        "demo" => match run_proof(&capsule, 3).await {
            Ok((exit, continuity)) => {
                println!(
                    "{}",
                    serde_json::json!({
                        "schema": "adl.runtime_kernel.proof.v1",
                        "exit": format!("{exit:?}"),
                        "capsule": continuity,
                    })
                );
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("runtime kernel proof failed: {error}");
                ExitCode::from(70)
            }
        },
        "fatal-once" => {
            let marker = capsule.with_extension("fatal-once");
            if !marker.exists() {
                if let Err(error) = run_proof(&capsule, 3).await {
                    eprintln!("failed to checkpoint before fatal exit: {error}");
                    return ExitCode::from(74);
                }
                if let Err(error) = tokio::fs::write(&marker, b"fatal child exit injected").await {
                    eprintln!("failed to write fatal-once marker: {error}");
                    return ExitCode::from(74);
                }
                eprintln!("classified_fatal_exit:first_generation");
                ExitCode::from(70)
            } else {
                match run_proof(&capsule, 3).await {
                    Ok(_) => match load_capsule(&capsule).await {
                        Ok(capsule) if capsule.generation >= 2 => ExitCode::SUCCESS,
                        Ok(_) => {
                            eprintln!("runtime recovery did not advance continuity generation");
                            ExitCode::from(70)
                        }
                        Err(error) => {
                            eprintln!("runtime recovery capsule invalid: {error}");
                            ExitCode::from(70)
                        }
                    },
                    Err(error) => {
                        eprintln!("runtime recovery proof failed: {error}");
                        ExitCode::from(70)
                    }
                }
            }
        }
        _ => {
            eprintln!("usage: adl-runtime-kernel [serve|demo|fatal-once] [capsule-path]");
            ExitCode::from(64)
        }
    }
}

fn process_exit(exit: KernelExit) -> ExitCode {
    match exit {
        KernelExit::Clean => ExitCode::SUCCESS,
        KernelExit::Fatal { component } => {
            eprintln!("classified_fatal_exit:{component}");
            ExitCode::from(70)
        }
        KernelExit::ShutdownFailed { components } => {
            eprintln!("classified_shutdown_failure:{components:?}");
            ExitCode::from(74)
        }
        KernelExit::ShutdownDeadlineExceeded { aborted } => {
            eprintln!("classified_shutdown_deadline:{aborted:?}");
            ExitCode::from(70)
        }
    }
}

async fn drain_control_api(
    api: &mut tokio::task::JoinHandle<Result<(), adl_runtime_kernel::ControlApiError>>,
) {
    if tokio::time::timeout(std::time::Duration::from_secs(2), &mut *api)
        .await
        .is_err()
    {
        api.abort();
    }
}
