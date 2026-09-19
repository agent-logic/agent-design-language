//! Operator-owned CodeFriend backend; place behind a TLS ingress for hosted use.
use adl::codefriend::server::{Config, ProductionBackend, Service};
use anyhow::{ensure, Result};
use std::{io::Read, net::SocketAddr, sync::Arc};
#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    ensure!(
        (args.len() == 4 || (args.len() == 6 && args[4] == "--control-socket"))
            && args[0] == "--config" && args[2] == "--listen",
        "usage: codefriend-server --config <operator-config.json> --listen <loopback-address:port> [--control-socket <private-unix-socket>]"
    );
    let listen: SocketAddr = args[3].parse()?;
    ensure!(
        listen.ip().is_loopback(),
        "TLS_ingress_required_use_loopback_listener"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(&args[1])?
        .take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 1024 * 1024, "config_too_large");
    let config: Config = serde_json::from_slice(&bytes)?;
    let service = Service::open(config, Arc::new(ProductionBackend))?;
    #[cfg(unix)]
    let control = if args.len() == 6 {
        Some(adl::codefriend::server::control::ControlServer::bind(
            service.clone(),
            std::path::Path::new(&args[5]),
        )?)
    } else {
        None
    };
    #[cfg(not(unix))]
    ensure!(args.len() == 4, "unix_control_socket_required");
    let listener = tokio::net::TcpListener::bind(listen).await?;
    #[cfg(unix)]
    let control_task = control.map(|control| {
        tokio::spawn(async move {
            if control.serve().await.is_err() {
                eprintln!("adl_event component=codefriend_server event=control_listener_failed");
            }
        })
    });
    let maintenance = service.clone();
    let maintenance_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            let service = maintenance.clone();
            let outcome = tokio::task::spawn_blocking(move || service.expire()).await;
            if !matches!(outcome, Ok(Ok(()))) {
                eprintln!("adl_event component=codefriend_server event=retention_failure");
            }
        }
    });
    eprintln!(
        "adl_event component=codefriend_server event=listening address={}",
        listener.local_addr()?
    );
    let result = axum::serve(listener, service.router())
        .with_graceful_shutdown(async {
            if tokio::signal::ctrl_c().await.is_err() {
                eprintln!("adl_event component=codefriend_server event=shutdown_signal_failure");
            }
        })
        .await;
    #[cfg(unix)]
    if let Some(task) = control_task {
        task.abort();
        let _ = task.await;
    }
    maintenance_task.abort();
    let _ = maintenance_task.await;
    result?;
    Ok(())
}
