use std::{
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

use sha2::{Digest, Sha256};
use tokio::{
    fs,
    sync::watch,
    task::JoinHandle,
    time::{self, Duration, Instant, MissedTickBehavior},
};
use tokio_util::sync::CancellationToken;

pub type ConfigParser<T> =
    Arc<dyn Fn(&str) -> Result<T, ConfigReloadError> + Send + Sync + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigReloadOptions {
    pub poll_interval: Duration,
    pub debounce: Duration,
}

impl Default for ConfigReloadOptions {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_millis(250),
            debounce: Duration::from_millis(500),
        }
    }
}

impl ConfigReloadOptions {
    pub fn validate(self) -> Result<Self, ConfigReloadError> {
        if self.poll_interval.is_zero() {
            return Err(ConfigReloadError::InvalidOptions(
                "poll_interval must be non-zero",
            ));
        }
        if self.debounce.is_zero() {
            return Err(ConfigReloadError::InvalidOptions(
                "debounce must be non-zero",
            ));
        }
        Ok(self)
    }
}

#[derive(Debug)]
pub struct ConfigSnapshot<T> {
    generation: u64,
    value: T,
    source: PathBuf,
    loaded_at: SystemTime,
}

impl<T> ConfigSnapshot<T> {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn source(&self) -> &Path {
        &self.source
    }

    pub fn loaded_at(&self) -> SystemTime {
        self.loaded_at
    }
}

#[derive(Debug)]
pub struct ConfigReloadController<T> {
    handle: HotReloadHandle<T>,
    shutdown: CancellationToken,
    task: JoinHandle<ConfigReloadOutcome>,
}

impl<T> ConfigReloadController<T> {
    pub fn handle(&self) -> HotReloadHandle<T> {
        self.handle.clone()
    }

    pub fn shutdown_token(&self) -> CancellationToken {
        self.shutdown.clone()
    }

    pub async fn shutdown(self) -> Result<ConfigReloadOutcome, ConfigReloadError> {
        self.shutdown.cancel();
        self.join().await
    }

    pub async fn join(self) -> Result<ConfigReloadOutcome, ConfigReloadError> {
        self.task
            .await
            .map_err(|error| ConfigReloadError::WatcherJoin(error.to_string()))
    }
}

#[derive(Debug)]
pub struct HotReloadHandle<T> {
    receiver: watch::Receiver<Arc<ConfigSnapshot<T>>>,
}

impl<T> Clone for HotReloadHandle<T> {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.clone(),
        }
    }
}

impl<T> HotReloadHandle<T> {
    pub fn current(&self) -> Arc<ConfigSnapshot<T>> {
        Arc::clone(&self.receiver.borrow())
    }

    pub async fn changed(&mut self) -> Result<Arc<ConfigSnapshot<T>>, ConfigReloadError> {
        self.receiver
            .changed()
            .await
            .map_err(|_| ConfigReloadError::WatcherClosed)?;
        Ok(self.current())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigReloadOutcome {
    pub reloads_applied: u64,
    pub invalid_updates_rejected: u64,
    pub shutdown_requested: bool,
}

#[derive(Debug)]
pub enum ConfigReloadError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse(String),
    Validation(String),
    InvalidOptions(&'static str),
    WatcherClosed,
    WatcherJoin(String),
}

impl ConfigReloadError {
    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse(message.into())
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }
}

impl fmt::Display for ConfigReloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(f, "failed to read {}: {source}", path.display())
            }
            Self::Parse(message) => write!(f, "config parse failed: {message}"),
            Self::Validation(message) => write!(f, "config validation failed: {message}"),
            Self::InvalidOptions(message) => write!(f, "invalid reload options: {message}"),
            Self::WatcherClosed => write!(f, "config reload watcher closed"),
            Self::WatcherJoin(message) => write!(f, "config reload watcher join failed: {message}"),
        }
    }
}

impl std::error::Error for ConfigReloadError {}

pub async fn start_config_reload<T>(
    path: impl Into<PathBuf>,
    parser: ConfigParser<T>,
    options: ConfigReloadOptions,
) -> Result<ConfigReloadController<T>, ConfigReloadError>
where
    T: Send + Sync + 'static,
{
    start_config_reload_with_shutdown(path, parser, options, CancellationToken::new()).await
}

pub async fn start_config_reload_with_shutdown<T>(
    path: impl Into<PathBuf>,
    parser: ConfigParser<T>,
    options: ConfigReloadOptions,
    shutdown: CancellationToken,
) -> Result<ConfigReloadController<T>, ConfigReloadError>
where
    T: Send + Sync + 'static,
{
    let path = path.into();
    let options = options.validate()?;
    let signature = file_signature(&path).await?;
    let initial = load_snapshot(&path, &parser, 0).await?;
    let (sender, receiver) = watch::channel(Arc::new(initial));
    let task_shutdown = shutdown.clone();
    let task_path = path.clone();
    let task = tokio::spawn(async move {
        watch_config(task_path, parser, options, task_shutdown, signature, sender).await
    });

    Ok(ConfigReloadController {
        handle: HotReloadHandle { receiver },
        shutdown,
        task,
    })
}

async fn watch_config<T>(
    path: PathBuf,
    parser: ConfigParser<T>,
    options: ConfigReloadOptions,
    shutdown: CancellationToken,
    mut last_evaluated: FileSignature,
    sender: watch::Sender<Arc<ConfigSnapshot<T>>>,
) -> ConfigReloadOutcome
where
    T: Send + Sync + 'static,
{
    let mut interval = time::interval(options.poll_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let mut pending: Option<(FileSignature, Instant)> = None;
    let mut generation = 0;
    let mut reloads_applied = 0;
    let mut invalid_updates_rejected = 0;

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                return ConfigReloadOutcome {
                    reloads_applied,
                    invalid_updates_rejected,
                    shutdown_requested: true,
                };
            }
            _ = interval.tick() => {
                if let Ok(signature) = file_signature(&path).await {
                    if signature != last_evaluated
                        && pending.as_ref().map(|(pending, _)| pending) != Some(&signature)
                    {
                        pending = Some((signature, Instant::now() + options.debounce));
                    }
                }

                let ready = pending
                    .as_ref()
                    .is_some_and(|(_, deadline)| Instant::now() >= *deadline);
                if !ready {
                    continue;
                }

                let Some((signature, _)) = pending.take() else {
                    continue;
                };
                generation += 1;
                match load_snapshot(&path, &parser, generation).await {
                    Ok(snapshot) => {
                        sender.send_replace(Arc::new(snapshot));
                        last_evaluated = signature;
                        reloads_applied += 1;
                    }
                    Err(_) => {
                        last_evaluated = signature;
                        invalid_updates_rejected += 1;
                    }
                }
            }
        }
    }
}

async fn load_snapshot<T>(
    path: &Path,
    parser: &ConfigParser<T>,
    generation: u64,
) -> Result<ConfigSnapshot<T>, ConfigReloadError> {
    let raw = fs::read_to_string(path)
        .await
        .map_err(|source| ConfigReloadError::Io {
            path: path.to_path_buf(),
            source,
        })?;
    let value = parser(&raw)?;
    Ok(ConfigSnapshot {
        generation,
        value,
        source: path.to_path_buf(),
        loaded_at: SystemTime::now(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileSignature {
    len: u64,
    sha256: [u8; 32],
}

async fn file_signature(path: &Path) -> Result<FileSignature, ConfigReloadError> {
    let raw = fs::read(path)
        .await
        .map_err(|source| ConfigReloadError::Io {
            path: path.to_path_buf(),
            source,
        })?;
    let digest = Sha256::digest(&raw);
    Ok(FileSignature {
        len: raw.len() as u64,
        sha256: digest.into(),
    })
}
