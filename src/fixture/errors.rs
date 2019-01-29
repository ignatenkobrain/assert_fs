//! Error types for fixtures.

use std::error::Error;
use std::fmt;

pub(crate) trait ChainError {
    fn chain<F>(self, cause: F) -> Self
    where
        F: Error + Send + Sync + 'static;
}

pub(crate) trait ResultChainExt<T> {
    fn chain<C>(self, chainable: C) -> Result<T, C>
    where
        C: ChainError;

    fn chain_with<F, C>(self, chainable: F) -> Result<T, C>
    where
        F: FnOnce() -> C,
        C: ChainError;
}

impl<T, E> ResultChainExt<T> for Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    fn chain<C>(self, chainable: C) -> Result<T, C>
    where
        C: ChainError,
    {
        self.map_err(|e| chainable.chain(e))
    }

    fn chain_with<F, C>(self, chainable: F) -> Result<T, C>
    where
        F: FnOnce() -> C,
        C: ChainError,
    {
        self.map_err(|e| chainable().chain(e))
    }
}

/// Fixture initialization cause.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FixtureKind {
    /// Failed when walking the source tree.
    Walk,
    /// Failed when copying a file.
    CopyFile,
    /// Failed when creating a directory.
    CreateDir,
    /// Failed to cleanup fixture.
    Cleanup,
}

impl fmt::Display for FixtureKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FixtureKind::Walk => write!(f, "Failed when walking the source tree,"),
            FixtureKind::CopyFile => write!(f, "Failed when copying a file."),
            FixtureKind::CreateDir => write!(f, "Failed when creating a directory."),
            FixtureKind::Cleanup => write!(f, "Failed to cleanup fixture."),
        }
    }
}

/// Failure when initializing the fixture.
#[derive(Debug)]
pub struct FixtureError {
    kind: FixtureKind,
    cause: Option<Box<Error + Send + Sync + 'static>>,
}

impl FixtureError {
    /// Create a `FixtureError`.
    pub fn new(kind: FixtureKind) -> Self {
        Self { kind, cause: None }
    }

    /// Fixture initialization cause.
    pub fn kind(&self) -> FixtureKind {
        self.kind
    }
}

impl Error for FixtureError {
    fn description(&self) -> &str {
        "Failed to initialize fixture"
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|c| {
            let c: &Error = c.as_ref();
            c
        })
    }
}

impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            Some(ref cause) => write!(
                f,
                "Failed to initialize fixture: {}\nCause: {}",
                self.kind, cause
            ),
            None => write!(f, "Failed to initialize fixture: {}", self.kind),
        }
    }
}

impl ChainError for FixtureError {
    fn chain<F>(mut self, cause: F) -> Self
    where
        F: Error + Send + Sync + 'static,
    {
        self.cause = Some(Box::new(cause));
        self
    }
}
