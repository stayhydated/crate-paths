use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocsrsBackendError {
	#[error("failed to build HTTP client: {0}")]
	ClientBuild(#[source] reqwest::Error),

	#[error("failed to GET {0}: {1}")]
	HttpGet(String, #[source] reqwest::Error),

	#[error("HTTP error for {0}: {1}")]
	HttpStatus(String, #[source] reqwest::Error),

	#[error("UTF-8 conversion error: {0}")]
	Utf8(#[from] std::str::Utf8Error),

	#[error("failed to read compressed response body: {0}")]
	Decompress(#[source] std::io::Error),
}
