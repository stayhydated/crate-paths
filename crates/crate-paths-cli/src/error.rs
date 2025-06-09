use crate::writer::WriterError;
use crate_paths_cli_core::error::CoreError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CratePathCliError {
    #[error("core error: {0}")]
    Core(#[from] CoreError),
    #[error("writer error: {0}")]
    Writer(#[from] WriterError),
}
