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
pub type ConfigApplier<T> =
    Arc<dyn Fn(&T) -> Result<(), ConfigReloadError> + Send + Sync + 'static>;

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
    start_config_reload_with_applier_and_shutdown(path, parser, None, options, shutdown).await
}

pub async fn start_config_reload_with_applier_and_shutdown<T>(
    path: impl Into<PathBuf>,
    parser: ConfigParser<T>,
    applier: Option<ConfigApplier<T>>,
    options: ConfigReloadOptions,
    shutdown: CancellationToken,
) -> Result<ConfigReloadController<T>, ConfigReloadError>
where
    T: Send + Sync + 'static,
{
    let path = path.into();
    let options = options.validate()?;
    let raw = read_config(&path).await?;
    let signature = FileSignature::from_bytes(&raw);
    let initial = parse_snapshot(&path, &parser, &raw, 0)?;
    if let Some(applier) = applier.as_ref() {
        applier(initial.value())?;
    }
    let (sender, receiver) = watch::channel(Arc::new(initial));
    let task_shutdown = shutdown.clone();
    let task_path = path.clone();
    let task = tokio::spawn(async move {
        watch_config(
            task_path,
            parser,
            applier,
            options,
            task_shutdown,
            signature,
            sender,
        )
        .await
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
    applier: Option<ConfigApplier<T>>,
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

    let mut pending: Option<(FileSignature, Vec<u8>, Instant)> = None;
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
                match read_config(&path).await {
                    Ok(raw) => {
                        let signature = FileSignature::from_bytes(&raw);
                        if signature == last_evaluated {
                            pending = None;
                        } else if pending.as_ref().map(|(pending, _, _)| pending) != Some(&signature) {
                            pending = Some((signature, raw, Instant::now() + options.debounce));
                        }
                    }
                    Err(_) => pending = None,
                }

                let ready = pending
                    .as_ref()
                    .is_some_and(|(_, _, deadline)| Instant::now() >= *deadline);
                if !ready {
                    continue;
                }

                let Some((signature, raw, _)) = pending.take() else {
                    continue;
                };
                generation += 1;
                match parse_snapshot(&path, &parser, &raw, generation).and_then(|snapshot| {
                    if let Some(applier) = applier.as_ref() {
                        applier(snapshot.value())?;
                    }
                    Ok(snapshot)
                }) {
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

fn parse_snapshot<T>(
    path: &Path,
    parser: &ConfigParser<T>,
    raw: &[u8],
    generation: u64,
) -> Result<ConfigSnapshot<T>, ConfigReloadError> {
    let raw =
        std::str::from_utf8(raw).map_err(|error| ConfigReloadError::Parse(error.to_string()))?;
    let value = parser(raw)?;
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

impl FileSignature {
    fn from_bytes(raw: &[u8]) -> Self {
        let digest = Sha256::digest(raw);
        Self {
            len: raw.len() as u64,
            sha256: digest.into(),
        }
    }
}

async fn read_config(path: &Path) -> Result<Vec<u8>, ConfigReloadError> {
    fs::read(path)
        .await
        .map_err(|source| ConfigReloadError::Io {
            path: path.to_path_buf(),
            source,
        })
}
