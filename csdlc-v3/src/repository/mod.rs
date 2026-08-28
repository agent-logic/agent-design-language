use std::fmt;
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

    pub fn relative_display(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn require_file(&self, path: &Path, label: &'static str) -> Result<(), RepositoryContextError> {
        let metadata =
            path.metadata()
                .map_err(|source| RepositoryContextError::RequiredFileUnavailable {
                    label,
                    path: path.to_path_buf(),
                    source,
                })?;
        if !metadata.is_file() {
            return Err(RepositoryContextError::RequiredPathNotFile {
                label,
                path: path.to_path_buf(),
            });
        }
        Ok(())
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
        }
    }
}

impl std::error::Error for RepositoryContextError {}
