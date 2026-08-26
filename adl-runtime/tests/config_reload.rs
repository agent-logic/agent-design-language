use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime::config_reload::{
    start_config_reload, ConfigParser, ConfigReloadError, ConfigReloadOptions, HotReloadHandle,
};
use serde::Deserialize;
use tempfile::TempDir;
use tokio::time::{sleep, timeout};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct TestConfig {
    name: String,
    workers: u32,
    left: u32,
    right: u32,
}

fn parser() -> ConfigParser<TestConfig> {
    Arc::new(|raw| {
        let config: TestConfig =
            toml::from_str(raw).map_err(|error| ConfigReloadError::parse(error.to_string()))?;
        if config.name.trim().is_empty() {
            return Err(ConfigReloadError::validation("name is required"));
        }
        if config.workers == 0 {
            return Err(ConfigReloadError::validation("workers must be positive"));
        }
        if config.left != config.right {
            return Err(ConfigReloadError::validation("pair must be complete"));
        }
        Ok(config)
    })
}

fn options() -> ConfigReloadOptions {
    ConfigReloadOptions {
        poll_interval: Duration::from_millis(10),
        debounce: Duration::from_millis(40),
    }
}

fn render(name: &str, workers: u32, pair: u32) -> String {
    format!("name = \"{name}\"\nworkers = {workers}\nleft = {pair}\nright = {pair}\n")
}

async fn write_config(path: &std::path::Path, body: String) {
    tokio::fs::write(path, body).await.expect("write config");
}

async fn wait_for_generation(
    handle: &HotReloadHandle<TestConfig>,
    generation: u64,
) -> Arc<adl_runtime::config_reload::ConfigSnapshot<TestConfig>> {
    let mut watcher = handle.clone();
    timeout(Duration::from_secs(2), async {
        loop {
            let snapshot = handle.current();
            if snapshot.generation() >= generation {
                return snapshot;
            }
            let snapshot = watcher.changed().await.expect("watcher update");
            if snapshot.generation() >= generation {
                return snapshot;
            }
        }
    })
    .await
    .expect("generation observed")
}

#[tokio::test]
async fn valid_reload_atomically_replaces_snapshot() {
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("runtime.toml");
    write_config(&path, render("stable-a", 2, 7)).await;

    let controller = start_config_reload(&path, parser(), options())
        .await
        .expect("start reload");
    let handle = controller.handle();
    assert_eq!(handle.current().value().name, "stable-a");

    write_config(&path, render("stable-b", 4, 9)).await;
    let snapshot = wait_for_generation(&handle, 1).await;

    assert_eq!(snapshot.value().name, "stable-b");
    assert_eq!(snapshot.value().workers, 4);
    assert_eq!(snapshot.value().left, snapshot.value().right);

    write_config(&path, render("stable-c", 6, 5)).await;
    let same_length_snapshot = wait_for_generation(&handle, 2).await;

    assert_eq!(same_length_snapshot.value().name, "stable-c");
    assert_eq!(same_length_snapshot.value().workers, 6);
    assert_eq!(
        render("stable-b", 4, 9).len(),
        render("stable-c", 6, 5).len()
    );

    let outcome = controller.shutdown().await.expect("shutdown");
    assert_eq!(outcome.reloads_applied, 2);
    assert!(outcome.shutdown_requested);
}

#[tokio::test]
async fn invalid_update_retains_last_known_good() {
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("runtime.toml");
    write_config(&path, render("stable", 2, 3)).await;

    let controller = start_config_reload(&path, parser(), options())
        .await
        .expect("start reload");
    let handle = controller.handle();

    write_config(&path, render("invalid", 0, 4)).await;
    sleep(Duration::from_millis(140)).await;
    let stable = handle.current();
    assert_eq!(stable.generation(), 0);
    assert_eq!(stable.value().name, "stable");

    write_config(&path, render("recovered", 5, 8)).await;
    let recovered = wait_for_generation(&handle, 1).await;
    assert_eq!(recovered.value().name, "recovered");

    let outcome = controller.shutdown().await.expect("shutdown");
    assert_eq!(outcome.invalid_updates_rejected, 1);
    assert_eq!(outcome.reloads_applied, 1);
}

#[tokio::test]
async fn file_events_are_debounced() {
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("runtime.toml");
    write_config(&path, render("initial", 1, 1)).await;
    let parses = Arc::new(AtomicUsize::new(0));
    let parse_count = Arc::clone(&parses);
    let parser: ConfigParser<TestConfig> = Arc::new(move |raw| {
        parse_count.fetch_add(1, Ordering::SeqCst);
        parser()(raw)
    });

    let controller = start_config_reload(&path, parser, options())
        .await
        .expect("start reload");
    let handle = controller.handle();

    write_config(&path, render("burst-1", 2, 2)).await;
    sleep(Duration::from_millis(5)).await;
    write_config(&path, render("burst-2", 3, 3)).await;
    sleep(Duration::from_millis(5)).await;
    write_config(&path, render("burst-3", 4, 4)).await;

    let snapshot = wait_for_generation(&handle, 1).await;
    assert_eq!(snapshot.value().name, "burst-3");
    assert_eq!(parses.load(Ordering::SeqCst), 2);

    let outcome = controller.shutdown().await.expect("shutdown");
    assert_eq!(outcome.reloads_applied, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_readers_observe_complete_configurations() {
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("runtime.toml");
    write_config(&path, render("initial", 1, 1)).await;

    let controller = start_config_reload(&path, parser(), options())
        .await
        .expect("start reload");
    let handle = controller.handle();
    let mut readers = Vec::new();

    for _ in 0..16 {
        let reader = handle.clone();
        readers.push(tokio::spawn(async move {
            for _ in 0..2_000 {
                let snapshot = reader.current();
                assert_eq!(snapshot.value().left, snapshot.value().right);
                assert!(!snapshot.value().name.is_empty());
                tokio::task::yield_now().await;
            }
        }));
    }

    for pair in 2..12 {
        write_config(&path, render(&format!("config-{pair}"), pair, pair)).await;
        let _ = wait_for_generation(&handle, pair as u64 - 1).await;
    }

    for reader in readers {
        reader.await.expect("reader task");
    }
    assert_eq!(handle.current().value().name, "config-11");

    let outcome = controller.shutdown().await.expect("shutdown");
    assert_eq!(outcome.reloads_applied, 10);
}

#[tokio::test]
async fn watcher_shutdown_is_clean() {
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("runtime.toml");
    write_config(&path, render("initial", 1, 1)).await;

    let controller = start_config_reload(&path, parser(), options())
        .await
        .expect("start reload");
    let handle = controller.handle();

    let outcome = timeout(Duration::from_secs(1), controller.shutdown())
        .await
        .expect("shutdown deadline")
        .expect("shutdown result");
    assert!(outcome.shutdown_requested);

    write_config(&path, render("after-shutdown", 2, 2)).await;
    sleep(Duration::from_millis(100)).await;
    assert_eq!(handle.current().generation(), 0);
    assert_eq!(handle.current().value().name, "initial");
}
