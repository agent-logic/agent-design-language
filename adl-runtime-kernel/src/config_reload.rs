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
pub type ConfigRejectionReporter = Arc<dyn Fn(ConfigReloadRejection) + Send + Sync + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigReloadRejection {
    SourceUnavailable,
    ParseInvalid,
    ValidationInvalid,
}

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
    status_receiver: watch::Receiver<ConfigReloadStatus>,
}

impl<T> Clone for HotReloadHandle<T> {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.clone(),
            status_receiver: self.status_receiver.clone(),
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

    /// Returns the latest watcher state without affecting reload behavior.
    pub fn reload_status(&self) -> ConfigReloadStatus {
        *self.status_receiver.borrow()
    }

    /// Waits until the watcher state changes and returns the new state.
    pub async fn reload_status_changed(&mut self) -> Result<ConfigReloadStatus, ConfigReloadError> {
        self.status_receiver
            .changed()
            .await
            .map_err(|_| ConfigReloadError::WatcherClosed)?;
        Ok(self.reload_status())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ConfigReloadStatus {
    candidate_observations: u64,
    pending_candidate: bool,
    pending_cancellations: u64,
}

impl ConfigReloadStatus {
    pub fn candidate_observations(self) -> u64 {
        self.candidate_observations
    }

    pub fn pending_candidate(self) -> bool {
        self.pending_candidate
    }

    pub fn pending_cancellations(self) -> u64 {
        self.pending_cancellations
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigReloadOutcome {
    pub reloads_applied: u64,
    pub invalid_updates_rejected: u64,
    pub shutdown_requested: bool,
}

struct ConfigReloadWatchers<T> {
    snapshot: watch::Sender<Arc<ConfigSnapshot<T>>>,
    status: watch::Sender<ConfigReloadStatus>,
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
    start_config_reload_from_snapshot(
        path,
        parser,
        applier,
        options,
        shutdown,
        Some(signature),
        initial,
        0,
        None,
        false,
    )
}

pub async fn start_config_reload_with_fallback_and_applier_and_shutdown<T>(
    path: impl Into<PathBuf>,
    fallback: T,
    parser: ConfigParser<T>,
    applier: Option<ConfigApplier<T>>,
    options: ConfigReloadOptions,
    shutdown: CancellationToken,
) -> Result<ConfigReloadController<T>, ConfigReloadError>
where
    T: Send + Sync + 'static,
{
    start_config_reload_with_fallback_applier_reporter_and_shutdown(
        path, fallback, parser, applier, None, options, shutdown,
    )
    .await
}

pub async fn start_config_reload_with_fallback_applier_reporter_and_shutdown<T>(
    path: impl Into<PathBuf>,
    fallback: T,
    parser: ConfigParser<T>,
    applier: Option<ConfigApplier<T>>,
    rejection_reporter: Option<ConfigRejectionReporter>,
    options: ConfigReloadOptions,
    shutdown: CancellationToken,
) -> Result<ConfigReloadController<T>, ConfigReloadError>
where
    T: Send + Sync + 'static,
{
    let path = path.into();
    let options = options.validate()?;
    let initial_read = read_config(&path).await;
    let (last_evaluated, initial, invalid_updates_rejected, source_unavailable) = match initial_read
    {
        Ok(raw) => {
            let signature = FileSignature::from_bytes(&raw);
            match parse_snapshot(&path, &parser, &raw, 0).and_then(|snapshot| {
                if let Some(applier) = applier.as_ref() {
                    applier(snapshot.value())?;
                }
                Ok(snapshot)
            }) {
                Ok(snapshot) => (Some(signature), snapshot, 0, false),
                Err(error) => {
                    report_rejection(rejection_reporter.as_ref(), &error);
                    (
                        Some(signature),
                        ConfigSnapshot {
                            generation: 0,
                            value: fallback,
                            source: path.clone(),
                            loaded_at: SystemTime::now(),
                        },
                        1,
                        false,
                    )
                }
            }
        }
        Err(error) => {
            report_rejection(rejection_reporter.as_ref(), &error);
            (
                None,
                ConfigSnapshot {
                    generation: 0,
                    value: fallback,
                    source: path.clone(),
                    loaded_at: SystemTime::now(),
                },
                1,
                true,
            )
        }
    };
    start_config_reload_from_snapshot(
        path,
        parser,
        applier,
        options,
        shutdown,
        last_evaluated,
        initial,
        invalid_updates_rejected,
        rejection_reporter,
        source_unavailable,
    )
}

fn start_config_reload_from_snapshot<T>(
    path: PathBuf,
    parser: ConfigParser<T>,
    applier: Option<ConfigApplier<T>>,
    options: ConfigReloadOptions,
    shutdown: CancellationToken,
    last_evaluated: Option<FileSignature>,
    initial: ConfigSnapshot<T>,
    invalid_updates_rejected: u64,
    rejection_reporter: Option<ConfigRejectionReporter>,
    source_unavailable: bool,
) -> Result<ConfigReloadController<T>, ConfigReloadError>
where
    T: Send + Sync + 'static,
{
    let (sender, receiver) = watch::channel(Arc::new(initial));
    let (status_sender, status_receiver) = watch::channel(ConfigReloadStatus::default());
    let task_shutdown = shutdown.clone();
    let task_path = path.clone();
    let task = tokio::spawn(async move {
        watch_config(
            task_path,
            parser,
            applier,
            options,
            task_shutdown,
            last_evaluated,
            ConfigReloadWatchers {
                snapshot: sender,
                status: status_sender,
            },
            invalid_updates_rejected,
            rejection_reporter,
            source_unavailable,
        )
        .await
    });

    Ok(ConfigReloadController {
        handle: HotReloadHandle {
            receiver,
            status_receiver,
        },
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
    mut last_evaluated: Option<FileSignature>,
    watchers: ConfigReloadWatchers<T>,
    mut invalid_updates_rejected: u64,
    rejection_reporter: Option<ConfigRejectionReporter>,
    mut source_unavailable: bool,
) -> ConfigReloadOutcome
where
    T: Send + Sync + 'static,
{
    let mut interval = time::interval(options.poll_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let mut pending: Option<(FileSignature, Vec<u8>, Instant)> = None;
    let mut generation = 0;
    let mut reloads_applied = 0;
    let mut status = ConfigReloadStatus::default();

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
                        source_unavailable = false;
                        let signature = FileSignature::from_bytes(&raw);
                        if Some(signature) == last_evaluated {
                            if pending.take().is_some() {
                                status.pending_candidate = false;
                                status.pending_cancellations += 1;
                                watchers.status.send_replace(status);
                            }
                        } else if pending.as_ref().map(|(pending, _, _)| pending) != Some(&signature) {
                            pending = Some((signature, raw, Instant::now() + options.debounce));
                            status.candidate_observations += 1;
                            status.pending_candidate = true;
                            watchers.status.send_replace(status);
                        }
                    }
                    Err(error) => {
                        if !source_unavailable {
                            report_rejection(rejection_reporter.as_ref(), &error);
                            invalid_updates_rejected += 1;
                            source_unavailable = true;
                        }
                        if pending.take().is_some() {
                            status.pending_candidate = false;
                            status.pending_cancellations += 1;
                            watchers.status.send_replace(status);
                        }
                    }
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
                status.pending_candidate = false;
                watchers.status.send_replace(status);
                generation += 1;
                match parse_snapshot(&path, &parser, &raw, generation).and_then(|snapshot| {
                    if let Some(applier) = applier.as_ref() {
                        applier(snapshot.value())?;
                    }
                    Ok(snapshot)
                }) {
                    Ok(snapshot) => {
                        watchers.snapshot.send_replace(Arc::new(snapshot));
                        last_evaluated = Some(signature);
                        reloads_applied += 1;
                    }
                    Err(error) => {
                        report_rejection(rejection_reporter.as_ref(), &error);
                        last_evaluated = Some(signature);
                        invalid_updates_rejected += 1;
                    }
                }
            }
        }
    }
}

fn report_rejection(reporter: Option<&ConfigRejectionReporter>, error: &ConfigReloadError) {
    let Some(reporter) = reporter else {
        return;
    };
    let reason = match error {
        ConfigReloadError::Io { .. } => ConfigReloadRejection::SourceUnavailable,
        ConfigReloadError::Parse(_) => ConfigReloadRejection::ParseInvalid,
        ConfigReloadError::Validation(_) => ConfigReloadRejection::ValidationInvalid,
        ConfigReloadError::InvalidOptions(_)
        | ConfigReloadError::WatcherClosed
        | ConfigReloadError::WatcherJoin(_) => return,
    };
    reporter(reason);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn test_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("{name}-{unique}-{}", std::process::id()))
    }

    async fn write_config_atomically(path: &Path, content: &str) {
        let temp_path = path.with_extension(format!(
            "next-{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        fs::write(&temp_path, content)
            .await
            .expect("write temp config");
        fs::rename(&temp_path, path).await.expect("publish config");
    }

    #[tokio::test]
    async fn config_reload_debounces_duplicate_same_content_and_shutdown() {
        let path = test_path("config-reload-debounce");
        write_config_atomically(&path, "one").await;
        let parses = Arc::new(AtomicU64::new(0));
        let parser_parses = Arc::clone(&parses);
        let parser: ConfigParser<String> = Arc::new(move |raw| {
            parser_parses.fetch_add(1, Ordering::SeqCst);
            match raw {
                "one" | "two" => Ok(raw.to_string()),
                _ => Err(ConfigReloadError::validation(
                    "transient or unknown config candidate",
                )),
            }
        });

        let controller = start_config_reload(
            path.clone(),
            parser,
            ConfigReloadOptions {
                poll_interval: Duration::from_millis(10),
                debounce: Duration::from_millis(30),
            },
        )
        .await
        .expect("start reload");
        let mut handle = controller.handle();
        assert_eq!(handle.current().value(), "one");

        write_config_atomically(&path, "two").await;
        write_config_atomically(&path, "two").await;
        let changed = time::timeout(Duration::from_secs(1), handle.changed())
            .await
            .expect("changed")
            .expect("snapshot");
        assert_eq!(changed.value(), "two");

        write_config_atomically(&path, "two").await;
        assert!(
            time::timeout(Duration::from_millis(120), handle.changed())
                .await
                .is_err(),
            "same-content rewrite must not publish another snapshot"
        );

        let outcome = controller.shutdown().await.expect("shutdown");
        assert_eq!(outcome.reloads_applied, 1);
        assert_eq!(outcome.invalid_updates_rejected, 0);
        assert!(outcome.shutdown_requested);
        assert!(parses.load(Ordering::SeqCst) >= 2);
    }

    #[tokio::test]
    async fn config_reload_invalid_update_retains_last_known_good() {
        let path = test_path("config-reload-invalid");
        fs::write(&path, "good").await.expect("write initial");
        let parser: ConfigParser<String> = Arc::new(move |raw| {
            if raw == "bad" {
                Err(ConfigReloadError::validation("redacted invalid candidate"))
            } else {
                Ok(raw.to_string())
            }
        });

        let controller = start_config_reload(
            path.clone(),
            parser,
            ConfigReloadOptions {
                poll_interval: Duration::from_millis(10),
                debounce: Duration::from_millis(30),
            },
        )
        .await
        .expect("start reload");
        let mut handle = controller.handle();

        fs::write(&path, "bad").await.expect("write invalid");
        assert!(
            time::timeout(Duration::from_millis(140), handle.changed())
                .await
                .is_err(),
            "invalid candidate must not publish a new snapshot"
        );
        assert_eq!(handle.current().value(), "good");

        let outcome = controller.shutdown().await.expect("shutdown");
        assert_eq!(outcome.reloads_applied, 0);
        assert_eq!(outcome.invalid_updates_rejected, 1);
        assert!(outcome.shutdown_requested);
    }

    #[tokio::test]
    async fn invalid_initial_config_keeps_watcher_alive_until_a_valid_replacement() {
        let path = test_path("config-reload-invalid-initial");
        fs::write(&path, "bad").await.expect("write initial");
        let parser: ConfigParser<String> = Arc::new(move |raw| {
            if raw == "good" {
                Ok(raw.to_owned())
            } else {
                Err(ConfigReloadError::validation("redacted invalid candidate"))
            }
        });
        let controller = start_config_reload_with_fallback_and_applier_and_shutdown(
            path.clone(),
            "fallback".to_owned(),
            parser,
            None,
            ConfigReloadOptions {
                poll_interval: Duration::from_millis(10),
                debounce: Duration::from_millis(30),
            },
            CancellationToken::new(),
        )
        .await
        .expect("start reload with fallback");
        let mut handle = controller.handle();
        assert_eq!(handle.current().value(), "fallback");

        write_config_atomically(&path, "good").await;
        let changed = time::timeout(Duration::from_secs(1), handle.changed())
            .await
            .expect("changed")
            .expect("snapshot");
        assert_eq!(changed.value(), "good");

        let outcome = controller.shutdown().await.expect("shutdown");
        assert_eq!(outcome.reloads_applied, 1);
        assert_eq!(outcome.invalid_updates_rejected, 1);
        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn missing_initial_config_keeps_watcher_alive_until_created() {
        let path = test_path("config-reload-missing-initial");
        let _ = std::fs::remove_file(&path);
        let parser: ConfigParser<String> = Arc::new(|raw| Ok(raw.to_owned()));
        let controller = start_config_reload_with_fallback_and_applier_and_shutdown(
            path.clone(),
            "fallback".to_owned(),
            parser,
            None,
            ConfigReloadOptions {
                poll_interval: Duration::from_millis(10),
                debounce: Duration::from_millis(30),
            },
            CancellationToken::new(),
        )
        .await
        .expect("start reload with missing source");
        let mut handle = controller.handle();
        assert_eq!(handle.current().value(), "fallback");

        write_config_atomically(&path, "created").await;
        let changed = time::timeout(Duration::from_secs(1), handle.changed())
            .await
            .expect("changed")
            .expect("snapshot");
        assert_eq!(changed.value(), "created");

        let outcome = controller.shutdown().await.expect("shutdown");
        assert_eq!(outcome.reloads_applied, 1);
        assert_eq!(outcome.invalid_updates_rejected, 1);
        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn fallback_reload_reports_redacted_rejection_classes_without_poll_spam() {
        let path = test_path("config-reload-rejection-reporter");
        let _ = std::fs::remove_file(&path);
        let parser: ConfigParser<String> = Arc::new(|raw| match raw {
            "parse" => Err(ConfigReloadError::parse(
                "candidate included operator-secret",
            )),
            "invalid" => Err(ConfigReloadError::validation(
                "candidate included private_key",
            )),
            _ => Ok(raw.to_owned()),
        });
        let rejections = Arc::new(std::sync::Mutex::new(Vec::new()));
        let reported = Arc::clone(&rejections);
        let reporter: ConfigRejectionReporter = Arc::new(move |reason| {
            reported.lock().expect("rejection lock").push(reason);
        });
        let controller = start_config_reload_with_fallback_applier_reporter_and_shutdown(
            path.clone(),
            "fallback".to_owned(),
            parser,
            None,
            Some(reporter),
            ConfigReloadOptions {
                poll_interval: Duration::from_millis(10),
                debounce: Duration::from_millis(30),
            },
            CancellationToken::new(),
        )
        .await
        .expect("start reload with rejection reporter");
        let mut handle = controller.handle();

        time::sleep(Duration::from_millis(50)).await;
        assert_eq!(
            *rejections.lock().expect("rejection lock"),
            vec![ConfigReloadRejection::SourceUnavailable],
            "repeated missing-file polls must emit one transition diagnostic"
        );

        write_config_atomically(&path, "parse").await;
        time::sleep(Duration::from_millis(90)).await;
        write_config_atomically(&path, "invalid").await;
        time::sleep(Duration::from_millis(90)).await;
        write_config_atomically(&path, "good").await;
        let changed = time::timeout(Duration::from_secs(1), handle.changed())
            .await
            .expect("changed")
            .expect("snapshot");
        assert_eq!(changed.value(), "good");
        assert_eq!(
            *rejections.lock().expect("rejection lock"),
            vec![
                ConfigReloadRejection::SourceUnavailable,
                ConfigReloadRejection::ParseInvalid,
                ConfigReloadRejection::ValidationInvalid,
            ]
        );

        let outcome = controller.shutdown().await.expect("shutdown");
        assert_eq!(outcome.reloads_applied, 1);
        assert_eq!(outcome.invalid_updates_rejected, 3);
        let _ = std::fs::remove_file(path);
    }
}
