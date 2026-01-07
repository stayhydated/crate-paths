use crate::error::CratePathCliError;
use crate_paths_cli_core::item::ItemEntry;
use crate_paths_cli_core::tree::ModTree;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WriterError {
    #[error("Failed to write to file: {0}")]
    FileWrite(#[source] io::Error),
    #[error("Failed to write to folder: {0}")]
    FolderWrite(#[source] io::Error),
    #[error("Failed to initialize crate with cargo: {0}")]
    CargoInit(String),
    #[error("Failed to add crate paths with cargo: {0}")]
    CargoAdd(String),
}

pub fn write_items(
    crate_name: &str,
    items: Vec<ItemEntry>,
    output_path: &Path,
    skip_init: bool,
) -> Result<(), CratePathCliError> {
    let definition = ModTree::new(items).to_rust_module_string();

    if output_path.extension().is_some() {
        write_to_file(definition, output_path)?;
    } else {
        write_to_folder(crate_name, definition, output_path, skip_init)?;
    }

    Ok(())
}

fn write_to_file(definition: String, output_path: &Path) -> Result<(), WriterError> {
    fs::write(output_path, definition).map_err(WriterError::FileWrite)?;
    Ok(())
}

fn write_to_folder(
    crate_name: &str,
    definition: String,
    output_path: &Path,
    skip_init: bool,
) -> Result<(), WriterError> {
    let folder_name_suffix = "crate-paths";
    let new_crate_folder_name = format!("{}-{}", crate_name, folder_name_suffix);
    let new_crate_paths = output_path.join(&new_crate_folder_name);

    fs::create_dir_all(&new_crate_paths).map_err(WriterError::FolderWrite)?;

    let lib_rs_path = new_crate_paths.join("src").join("lib.rs");

    let needs_cargo_init = !lib_rs_path.exists();

    if needs_cargo_init && !skip_init {
        let cargo_init_status = Command::new("cargo")
            .arg("init")
            .arg("--lib")
            .current_dir(&new_crate_paths)
            .status()
            .map_err(|e| WriterError::CargoInit(format!("Failed to execute cargo init: {}", e)))?;

        // Add publish = false to Cargo.toml
        let cargo_toml_path = new_crate_paths.join("Cargo.toml");
        if cargo_toml_path.exists() {
            let mut cargo_toml_content = fs::read_to_string(&cargo_toml_path)
                .map_err(|e| WriterError::CargoInit(format!("Failed to read Cargo.toml: {}", e)))?;

            if !cargo_toml_content.contains("publish = false") {
                cargo_toml_content.push_str("\npublish = false\n");
                fs::write(&cargo_toml_path, cargo_toml_content).map_err(|e| {
                    WriterError::CargoInit(format!("Failed to write Cargo.toml: {}", e))
                })?;
            }
        }

        let cargo_add_crate_paths_status = Command::new("cargo")
            .arg("add")
            .arg("crate-paths")
            .current_dir(&new_crate_paths)
            .status()
            .map_err(|e| WriterError::CargoAdd(format!("Failed to execute cargo add: {}", e)))?;

        if !cargo_init_status.success() {
            return Err(WriterError::CargoInit(format!(
                "cargo init failed with status: {}",
                cargo_init_status
            )));
        }

        if !cargo_add_crate_paths_status.success() {
            return Err(WriterError::CargoAdd(format!(
                "cargo add failed with status: {}",
                cargo_add_crate_paths_status
            )));
        }
    }
    fs::create_dir_all(new_crate_paths.join("src")).map_err(WriterError::FolderWrite)?;

    fs::write(&lib_rs_path, definition).map_err(WriterError::FolderWrite)?;

    Ok(())
}
