use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Explicit repository context for the non-authoritative v3 foundation slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryContext {
    root: PathBuf,
    contract_path: PathBuf,
    predecessor_coverage_path: PathBuf,
    proportional_lifecycle_path: PathBuf,
}

impl RepositoryContext {
    /// Build context from an explicit repository root.
    ///
    /// This constructor intentionally does not consult the process current
    /// directory. Callers must pass the root they intend to inspect.
    pub fn discover(root: impl AsRef<Path>) -> Result<Self, RepositoryContextError> {
        let root = root.as_ref();
        let root =
            root.canonicalize()
                .map_err(|source| RepositoryContextError::RootUnavailable {
                    root: root.to_path_buf(),
                    source,
                })?;
        if !root.is_dir() {
            return Err(RepositoryContextError::RootNotDirectory { root });
        }
        let context = Self {
            contract_path: root.join("docs/csdlc-v3/CONTRACT.md"),
            predecessor_coverage_path: root.join("docs/csdlc-v3/predecessor-coverage.json"),
            proportional_lifecycle_path: root.join("docs/csdlc-v3/proportional-lifecycle.json"),
            root,
        };
        context.require_file(&context.contract_path, "v3 contract")?;
        context.require_file(
            &context.predecessor_coverage_path,
            "predecessor coverage matrix",
        )?;
        context.require_file(
            &context.proportional_lifecycle_path,
            "proportional lifecycle matrix",
        )?;
        Ok(context)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn contract_path(&self) -> &Path {
        &self.contract_path
    }

    pub fn predecessor_coverage_path(&self) -> &Path {
        &self.predecessor_coverage_path
    }

    pub fn proportional_lifecycle_path(&self) -> &Path {
        &self.proportional_lifecycle_path
    }

    pub fn issue_record_text(&self, issue: u64) -> Result<String, RepositoryContextError> {
        let path = self.issue_record_path(issue);
        self.read_required_text(path, "v2 issue record")
    }

    pub fn card_text(&self, issue: u64, card: &str) -> Result<String, RepositoryContextError> {
        let path = self.card_path(issue, card)?;
        self.read_required_text(path, "v2 issue card")
    }

    pub fn card_values_text(
        &self,
        issue: u64,
        card: &str,
    ) -> Result<String, RepositoryContextError> {
        let path = self.card_values_path(issue, card)?;
        self.read_required_text(path, "v2 issue card values")
    }

    pub fn issue_record_path(&self, issue: u64) -> PathBuf {
        self.root.join(format!(".csdlc/issues/{issue}/index.json"))
    }

    pub fn card_path(&self, issue: u64, card: &str) -> Result<PathBuf, RepositoryContextError> {
        if !matches!(card, "sip" | "stp" | "spp" | "vpp" | "srp" | "sor") {
            return Err(RepositoryContextError::UnsupportedCard {
                card: card.to_owned(),
            });
        }
        Ok(self
            .root
            .join(format!(".csdlc/issues/{issue}/cards/{card}.md")))
    }

    pub fn card_values_path(
        &self,
        issue: u64,
        card: &str,
    ) -> Result<PathBuf, RepositoryContextError> {
        if !matches!(card, "sip" | "stp" | "spp" | "vpp" | "srp" | "sor") {
            return Err(RepositoryContextError::UnsupportedCard {
                card: card.to_owned(),
            });
        }
        Ok(self
            .root
            .join(format!(".csdlc/issues/{issue}/cards/{card}.values.json")))
    }

    pub fn relative_display(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn require_file(&self, path: &Path, label: &'static str) -> Result<(), RepositoryContextError> {
        let canonical = path.canonicalize().map_err(|source| {
            RepositoryContextError::RequiredFileUnavailable {
                label,
                path: path.to_path_buf(),
                source,
            }
        })?;
        if !canonical.starts_with(&self.root) {
            return Err(RepositoryContextError::PathEscapesRoot {
                label,
                path: path.to_path_buf(),
                canonical,
                root: self.root.clone(),
            });
        }
        let metadata = fs::metadata(&canonical).map_err(|source| {
            RepositoryContextError::RequiredFileUnavailable {
                label,
                path: canonical.clone(),
                source,
            }
        })?;
        if !metadata.is_file() {
            return Err(RepositoryContextError::RequiredPathNotFile {
                label,
                path: canonical,
            });
        }
        Ok(())
    }

    fn read_required_text(
        &self,
        path: PathBuf,
        label: &'static str,
    ) -> Result<String, RepositoryContextError> {
        self.require_file(&path, label)?;
        let canonical = path.canonicalize().map_err(|source| {
            RepositoryContextError::RequiredFileUnavailable {
                label,
                path: path.clone(),
                source,
            }
        })?;
        fs::read_to_string(&canonical).map_err(|source| {
            RepositoryContextError::RequiredFileUnavailable {
                label,
                path: canonical,
                source,
            }
        })
    }
}

#[derive(Debug)]
pub enum RepositoryContextError {
    RootUnavailable {
        root: PathBuf,
        source: std::io::Error,
    },
    RootNotDirectory {
        root: PathBuf,
    },
    RequiredFileUnavailable {
        label: &'static str,
        path: PathBuf,
        source: std::io::Error,
    },
    RequiredPathNotFile {
        label: &'static str,
        path: PathBuf,
    },
    PathEscapesRoot {
        label: &'static str,
        path: PathBuf,
        canonical: PathBuf,
        root: PathBuf,
    },
    UnsupportedCard {
        card: String,
    },
}

impl fmt::Display for RepositoryContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootUnavailable { root, source } => {
                write!(
                    formatter,
                    "repository root {} is unavailable: {source}",
                    root.display()
                )
            }
            Self::RootNotDirectory { root } => {
                write!(
                    formatter,
                    "repository root {} is not a directory",
                    root.display()
                )
            }
            Self::RequiredFileUnavailable {
                label,
                path,
                source,
            } => write!(
                formatter,
                "required {label} file {} is unavailable: {source}",
                path.display()
            ),
            Self::RequiredPathNotFile { label, path } => {
                write!(
                    formatter,
                    "required {label} path {} is not a file",
                    path.display()
                )
            }
            Self::PathEscapesRoot {
                label,
                path,
                canonical,
                root,
            } => write!(
                formatter,
                "required {label} path {} resolves outside repository root {} as {}",
                path.display(),
                root.display(),
                canonical.display()
            ),
            Self::UnsupportedCard { card } => write!(formatter, "unsupported card {card:?}"),
        }
    }
}

impl std::error::Error for RepositoryContextError {}
