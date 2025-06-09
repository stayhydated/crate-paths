pub mod error;

use super::BackendError;
use super::LocalBackendError;
use crate::error::CoreError;
use crate::item::ItemEntry;
use crate::parser;
use inflector::Inflector as _;
use std::path::PathBuf;
use std::process::Command;

fn fetch_crate_all_items_html(crate_name: &str) -> Result<String, LocalBackendError> {
    let cargo_status = Command::new("cargo")
        .args(["doc", "--workspace"])
        .status()
        .map_err(LocalBackendError::CargoDocExecution)?;

    if !cargo_status.success() {
        return Err(LocalBackendError::CargoDocStatus(
            cargo_status.code().unwrap_or(1),
        ));
    }

    let locate_project_output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format=plain"])
        .output()
        .map_err(LocalBackendError::CargoLocateProjectExecution)?;

    if !locate_project_output.status.success() {
        return Err(LocalBackendError::CargoLocateProjectStatus(
            locate_project_output.status.code().unwrap_or(1),
        ));
    }

    let cargo_toml_path_str = String::from_utf8(locate_project_output.stdout)
        .map_err(LocalBackendError::CargoLocateProjectOutput)?;

    let cargo_toml_path = PathBuf::from(cargo_toml_path_str.trim());

    let workspace_root = cargo_toml_path
        .parent()
        .ok_or_else(|| LocalBackendError::WorkspaceRootNotFound(cargo_toml_path.clone()))?;

    let base_doc_path = workspace_root.join("target").join("doc");
    let possible_names = [crate_name, &crate_name.to_snake_case()];

    for name in &possible_names {
        print!("OMG {}", name);
    }

    const ALL_HTML: &str = "all.html";

    let doc_file_path = possible_names
        .iter()
        .map(|name| base_doc_path.join(name).join(ALL_HTML))
        .find(|path| path.exists())
        .ok_or_else(|| {
            LocalBackendError::DocFileNotFound(base_doc_path.join(crate_name).join(ALL_HTML))
        })?;

    std::fs::read_to_string(&doc_file_path)
        .map_err(|e| LocalBackendError::FileRead(doc_file_path.clone(), e))
}

pub fn process(crate_name: &str) -> Result<Vec<ItemEntry>, CoreError> {
    let html_content = fetch_crate_all_items_html(crate_name)
        .map_err(BackendError::from)
        .map_err(CoreError::from)?;
    parser::parse_html_to_items(crate_name, &html_content).map_err(CoreError::from)
}
