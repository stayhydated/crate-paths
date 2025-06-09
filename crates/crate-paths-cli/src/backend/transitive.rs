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

    /// Output path for the generated paths tree.
    /// Can be a directory or a file.
    #[arg(short, long)]
    output_path: PathBuf,

    /// Skip calling `cargo init` when the output path is a directory (just write to a file at this point)
    #[arg(long, default_value = "false")]
    skip_init: bool,
}

#[allow(unused, dead_code)]
pub fn process() -> Result<(), CratePathCliError> {
    let args = TransitiveArgs::parse();

    eprintln!("Attempting Rustup backend for crate: {}", args.crate_name);
    let rustup_command = BackendCommand::Rustup {
        crate_name: args.crate_name.clone(),
    };
    match rustup_command.process(&args.output_path, args.skip_init) {
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

    eprintln!("Attempting Local backend for crate: {}", args.crate_name);
    let local_command = BackendCommand::Local {
        crate_name: args.crate_name.clone(),
    };
    match local_command.process(&args.output_path, args.skip_init) {
        Ok(()) => {
            return Ok(());
        },
        Err(e_local) => {
            eprintln!("Local backend failed: {}. Trying next backend...", e_local);
        },
    }

    let crate_version = "latest".to_owned();

    eprintln!(
        "Attempting Docs.rs backend for crate: {} version: {}",
        args.crate_name, crate_version
    );
    let docsrs_command = BackendCommand::Docsrs {
        crate_name: args.crate_name.clone(),
        crate_version,
    };
    match docsrs_command.process(&args.output_path, args.skip_init) {
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
