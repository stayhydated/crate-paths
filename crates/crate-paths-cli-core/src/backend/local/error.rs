use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocalBackendError {
	#[error("failed to execute 'cargo doc --workspace': {0}")]
	CargoDocExecution(#[source] std::io::Error),

	#[error("'cargo doc --workspace' command finished with a non-zero status code: {0}")]
	CargoDocStatus(i32),

	#[error("expected documentation file not found: {0}")]
	DocFileNotFound(PathBuf),

	#[error("failed to read documentation file {0}: {1}")]
	FileRead(PathBuf, #[source] std::io::Error),

	#[error("failed to execute 'cargo locate-project --workspace --message-format=plain': {0}")]
	CargoLocateProjectExecution(#[source] std::io::Error),

	#[error(
		"'cargo locate-project --workspace --message-format=plain' command finished with a non-zero status code: {0}"
	)]
	CargoLocateProjectStatus(i32),

	#[error(
		"failed to parse output of 'cargo locate-project --workspace --message-format=plain': {0}"
	)]
	CargoLocateProjectOutput(#[source] std::string::FromUtf8Error),

	#[error("could not determine workspace root from Cargo.toml path: {0}")]
	WorkspaceRootNotFound(PathBuf),
}
