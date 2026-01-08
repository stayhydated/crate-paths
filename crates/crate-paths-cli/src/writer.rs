use crate::error::CratePathCliError;
use crate_paths_cli_core::item::ItemEntry;
use crate_paths_cli_core::tree::ModTree;
use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WriterError {
    #[error("Failed to write to file: {0}")]
    FileWrite(#[source] io::Error),
}

pub fn write_items(items: Vec<ItemEntry>, output_path: &Path) -> Result<(), CratePathCliError> {
    let definition = ModTree::new(items).to_rust_module_string();
    fs::write(output_path, definition).map_err(WriterError::FileWrite)?;
    Ok(())
}
