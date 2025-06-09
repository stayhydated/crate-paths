use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustupBackendError {
	#[error("failed to execute 'rustup doc --path': {0}")]
	RustupDocExecution(#[source] std::io::Error),

	#[error("'rustup doc --path' command finished with a non-zero status code: {0}")]
	RustupDocStatus(i32),

	#[error("failed to read output of 'rustup doc --path': {0}")]
	RustupDocOutputRead(#[source] std::io::Error),

	#[error("output of 'rustup doc --path' was not valid UTF-8: {0}")]
	RustupDocOutputUtf8(#[from] std::string::FromUtf8Error),

	#[error("documentation path from 'rustup doc --path' does not exist: {0}")]
	DocPathNotFound(PathBuf),

	#[error("expected documentation file not found: {0}")]
	DocFileNotFound(PathBuf),

	#[error("documentation path from 'rustup doc --path' is invalid: {0}")]
	DocPathInvalid(PathBuf),

	#[error("failed to read documentation file {0}: {1}")]
	FileRead(PathBuf, #[source] std::io::Error),
}
