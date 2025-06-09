use thiserror::Error;

use crate::backend::BackendError;
use crate::parser::ParserError;

#[derive(Debug, Error)]
pub enum CoreError {
	#[error("Backend error: {0}")]
	Backend(#[from] BackendError),

	#[error("Parser error: {0}")]
	Parser(#[from] ParserError),
}
