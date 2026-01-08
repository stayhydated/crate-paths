use std::path::Path;

use crate::consts;
use crate::error::CratePathCliError;
use crate::writer;
use crate_paths_cli_core::backend;

pub fn run_docsrs(
    crate_name: &str,
    crate_version: &str,
    output_path: &Path,
) -> Result<(), CratePathCliError> {
    let items = backend::docrs::process(crate_name, crate_version, consts::APP_USER_AGENT)?;
    writer::write_items(items, output_path)?;
    Ok(())
}

pub fn run_local(crate_name: &str, output_path: &Path) -> Result<(), CratePathCliError> {
    let items = backend::local::process(crate_name)?;
    writer::write_items(items, output_path)?;
    Ok(())
}

pub fn run_rustup(crate_name: &str, output_path: &Path) -> Result<(), CratePathCliError> {
    let items = backend::rustup::process(crate_name)?;
    writer::write_items(items, output_path)?;
    Ok(())
}

pub fn run_auto(crate_name: &str, output_path: &Path) -> Result<(), CratePathCliError> {
    eprintln!("Attempting Rustup backend for crate: {}", crate_name);
    match backend::rustup::process(crate_name) {
        Ok(items) => {
            writer::write_items(items, output_path)?;
            return Ok(());
        },
        Err(e) => {
            eprintln!("Rustup backend failed: {}. Trying next backend...", e);
        },
    }

    eprintln!("Attempting Local backend for crate: {}", crate_name);
    match backend::local::process(crate_name) {
        Ok(items) => {
            writer::write_items(items, output_path)?;
            return Ok(());
        },
        Err(e) => {
            eprintln!("Local backend failed: {}. Trying next backend...", e);
        },
    }

    let crate_version =
        backend::local::get_crate_version(crate_name).unwrap_or_else(|| "latest".to_owned());

    eprintln!(
        "Attempting Docs.rs backend for crate: {} version: {}",
        crate_name, crate_version
    );
    match backend::docrs::process(crate_name, &crate_version, consts::APP_USER_AGENT) {
        Ok(items) => {
            writer::write_items(items, output_path)?;
            Ok(())
        },
        Err(e) => {
            eprintln!("Docs.rs backend failed: {}", e);
            eprintln!(
                "All backends (Rustup, Local, Docs.rs) failed to process crate: {}",
                crate_name
            );
            Err(e.into())
        },
    }
}
