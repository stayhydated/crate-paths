pub mod error;

use super::BackendError;
use crate::error::CoreError;
use crate::item::ItemEntry;
use crate::parser;
use error::RustupBackendError;
use std::path::PathBuf;
use std::process::Command;

fn fetch_std_all_items_html(crate_name: &str) -> Result<String, RustupBackendError> {
    let output = Command::new("rustup")
        .args(["doc", "--path"])
        .output()
        .map_err(RustupBackendError::RustupDocExecution)?;

    if !output.status.success() {
        return Err(RustupBackendError::RustupDocStatus(
            output.status.code().unwrap_or(1),
        ));
    }

    let doc_path_str = String::from_utf8(output.stdout)?;
    let trimmed_doc_path_str = doc_path_str.trim();

    let mut doc_root_path = PathBuf::from(trimmed_doc_path_str);

    if doc_root_path.is_file() {
        if let Some(parent_dir) = doc_root_path.parent() {
            doc_root_path = parent_dir.to_path_buf();
        } else {
            return Err(RustupBackendError::DocPathInvalid(PathBuf::from(
                trimmed_doc_path_str,
            )));
        }
    }

    if !doc_root_path.is_dir() {
        return Err(RustupBackendError::DocPathNotFound(doc_root_path));
    }

    let all_html_path = doc_root_path.join(crate_name).join("all.html");

    if !all_html_path.is_file() {
        return Err(RustupBackendError::DocFileNotFound(all_html_path));
    }

    std::fs::read_to_string(&all_html_path)
        .map_err(|e| RustupBackendError::FileRead(all_html_path, e))
}

pub fn process(crate_name: &str) -> Result<Vec<ItemEntry>, CoreError> {
    let html_content = fetch_std_all_items_html(crate_name)
        .map_err(BackendError::from)
        .map_err(CoreError::from)?;

    parser::parse_html_to_items(crate_name, &html_content).map_err(CoreError::from)
}
