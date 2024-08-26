use std::env;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const LOCK_FILE_NAME: &str = "centerpiece.lock";
const XDG_RUNTIME_DIR_ENV: &str = "XDG_RUNTIME_DIR";

#[derive(Debug)]
pub enum LockFileError {
    AlreadyLocked(PathBuf),
    IoError(std::io::Error),
    UnlockError(PathBuf, std::io::Error),
}

type LockFileResult<T> = Result<T, LockFileError>;

impl fmt::Display for LockFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_path = |path: &PathBuf| path.to_string_lossy().to_string();
        match self {
            LockFileError::AlreadyLocked(ref path) => {
                write!(
                    f,
                    "Could not hold an exclusive lock on the lockfile in {}\n\
                    If you think this is an error, please remove the lockfile manually",
                    display_path(path)
                )
            }
            LockFileError::IoError(ref err) => write!(f, "IoError: {}", err),
            LockFileError::UnlockError(ref path, ref err) => {
                write!(
                    f,
                    "Failed to remove the lockfile: {} - IoError: {}",
                    display_path(path),
                    err
                )
            }
        }
    }
}

impl From<std::io::Error> for LockFileError {
    fn from(err: std::io::Error) -> Self {
        LockFileError::IoError(err)
    }
}

/// Queries the environment for the $XDG_RUNTIME_DIR
fn get_xdg_runtime_dir() -> Option<String> {
    env::var(XDG_RUNTIME_DIR_ENV).ok()
}

#[derive(Debug)]
pub struct LockFile(PathBuf);

impl LockFile {
    fn get_or_init() -> &'static Option<Self> {
        static LOCK_FILE: OnceLock<Option<LockFile>> = OnceLock::new();
        LOCK_FILE.get_or_init(Self::init)
    }

    fn init() -> Option<Self> {
        Some(Self(Self::get_lock_file_path()?))
    }

    fn path(&self) -> &Path {
        self.0.as_path()
    }

    fn path_buf(&self) -> PathBuf {
        self.0.as_path().to_path_buf()
    }

    fn get_lock_file_path() -> Option<PathBuf> {
        let xdg_runtime_dir = get_xdg_runtime_dir()?;
        Some(Path::new(&xdg_runtime_dir).join(LOCK_FILE_NAME))
    }

    fn try_lock(&self) -> LockFileResult<()> {
        if self.path().is_file() {
            return Err(LockFileError::AlreadyLocked(self.path_buf()));
        } else {
            let _ = File::create(self.path())?;
        };
        Ok(())
    }

    fn remove_lockfile() -> LockFileResult<()> {
        if let Some(lock_file) = Self::get_or_init() {
            std::fs::remove_file(lock_file.path())
                .map_err(|err| LockFileError::UnlockError(lock_file.path_buf(), err))?
        }
        Ok(())
    }

    /// Attempts to remove the lock file - logs errors.
    /// *MUST* be run on every shutdown of `centerpiece`.
    pub fn unlock() {
        if let Err(err) = Self::remove_lockfile() {
            log::error!("{err}");
        }
    }
    /// Attempts to hold an exclusive lock in the runtime dir.
    /// If we can't find the XDG_RUNTIME_DIR, we don't hold a lock.
    /// If the lock is not successful, then exit `centerpiece`.
    pub fn run_exclusive() {
        Self::get_or_init().as_ref().map(|lock_file| {
            if let Err(err) = lock_file.try_lock() {
                log::error!("{err}");
                log::error!("Stopping centerpiece.");
                std::process::exit(1);
            }
        });
    }
}
