pub mod docrs;
pub mod local;
pub mod rustup;

use thiserror::Error;

use crate::backend::docrs::error::DocsrsBackendError;
use crate::backend::local::error::LocalBackendError;
use crate::backend::rustup::error::RustupBackendError;

#[derive(Debug, Error)]
pub enum BackendError {
	#[error("Docsrs Backend Error: {0}")]
	Docsrs(#[from] DocsrsBackendError),

	#[error("Local Backend Error: {0}")]
	Local(#[from] LocalBackendError),

	#[error("Rustup Backend Error: {0}")]
	Rustup(#[from] RustupBackendError),
}
