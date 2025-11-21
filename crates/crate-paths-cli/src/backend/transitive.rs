use std::path::PathBuf;

use crate::backend::specific::BackendCommand;
use crate::error::CratePathCliError;
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "crate-paths",
    version,
    about = "Build a tree of a crate's item paths"
)]
struct TransitiveArgs {
    /// Crate name, e.g. "clap" or "std"
    #[arg(short, long)]
    crate_name: String,

    /// Output file path for the generated paths tree.
    /// Can only be a file
    #[arg(short, long, value_parser = validate_file_path)]
    output_path: PathBuf,

    /// Skip local lookup
    #[arg(short, long)]
    skip_local: bool,
}

// avoid possible unintended crate pollution from the users lmao
fn validate_file_path(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if path.exists() {
        if path.is_file() {
            Ok(path)
        } else {
            Err("Path must be a file, not a directory".to_string())
        }
    } else if let Some(parent) = path.parent() {
        if parent.exists() && parent.is_dir() {
            Ok(path)
        } else {
            Err("Parent directory must exist".to_string())
        }
    } else {
        Ok(path)
    }
}

#[allow(unused, dead_code)]
pub fn process() -> Result<(), CratePathCliError> {
    let args = TransitiveArgs::parse();

    eprintln!("Attempting Rustup backend for crate: {}", args.crate_name);
    let rustup_command = BackendCommand::Rustup {
        crate_name: args.crate_name.clone(),
    };
    match rustup_command.process(&args.output_path, true) {
        Ok(()) => {
            return Ok(());
        },
        Err(e_rustup) => {
            eprintln!(
                "Rustup backend failed: {}. Trying next backend...",
                e_rustup
            );
        },
    }

    if !args.skip_local {
        eprintln!("Attempting Local backend for crate: {}", args.crate_name);
        let local_command = BackendCommand::Local {
            crate_name: args.crate_name.clone(),
        };
        match local_command.process(&args.output_path, true) {
            Ok(()) => {
                return Ok(());
            },
            Err(e_local) => {
                eprintln!("Local backend failed: {}. Trying next backend...", e_local);
            },
        }
    }

    let crate_version = crate_paths_cli_core::backend::local::get_crate_version(&args.crate_name)
        .unwrap_or_else(|| "latest".to_owned());

    eprintln!(
        "Attempting Docs.rs backend for crate: {} version: {}",
        args.crate_name, crate_version
    );
    let docsrs_command = BackendCommand::Docsrs {
        crate_name: args.crate_name.clone(),
        crate_version,
    };
    match docsrs_command.process(&args.output_path, true) {
        Ok(()) => Ok(()),
        Err(e_docsrs) => {
            eprintln!("Docs.rs backend failed: {}", e_docsrs);
            eprintln!(
                "All backends (Rustup, Local, Docs.rs) failed to process crate: {}",
                args.crate_name
            );
            Err(e_docsrs)
        },
    }
}
