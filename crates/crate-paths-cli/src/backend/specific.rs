use std::path::PathBuf;

use crate::consts;
use crate::error::CratePathCliError;
use crate::writer;
use clap::Parser;
use crate_paths_cli_core::backend;

// if you bothered looking at this code, please don't polute crates.io lmao
#[derive(Parser)]
#[command(
    name = "crate-paths",
    version,
    about = "Build a tree of a crate's item paths"
)]
struct SpecificArgs {
    #[clap(subcommand)]
    command: BackendCommand,

    /// Output path for the generated paths tree.
    /// Can be a directory or a file.
    #[arg(short, long)]
    output_path: PathBuf,

    /// Skip calling `cargo init` when the output path is a directory (just write to a file at this point)
    #[arg(long, default_value = "false")]
    skip_init: bool,
}

#[derive(Debug, Parser)]
pub enum BackendCommand {
    /// Fetch from docs.rs
    Docsrs {
        /// Crate name, e.g. "clap"
        #[arg(short, long)]
        crate_name: String,

        /// Crate version, e.g. "latest" or "6.9.69"
        #[arg(long, default_value = "latest")]
        crate_version: String,
    },
    /// Fetch locally.
    /// Crate has to be consumed by a member or `cargo doc`
    /// won't generate the docs for it.
    Local {
        // Crate name, e.g. "clap"
        #[arg(short, long)]
        crate_name: String,
    },
    /// Fetch from rustup
    Rustup {
        // Crate name, e.g. "std"
        #[arg(short, long)]
        crate_name: String,
    },
}

impl BackendCommand {
    pub fn process(&self, output_path: &PathBuf, skip_init: bool) -> Result<(), CratePathCliError> {
        let (crate_name, items) = match self {
            BackendCommand::Docsrs {
                crate_name,
                crate_version,
            } => {
                let items =
                    backend::docrs::process(crate_name, crate_version, consts::APP_USER_AGENT)
                        .map_err(CratePathCliError::from)?;

                (crate_name, items)
            },
            BackendCommand::Local { crate_name } => {
                let items = backend::local::process(crate_name).map_err(CratePathCliError::from)?;

                (crate_name, items)
            },
            BackendCommand::Rustup { crate_name } => {
                let items =
                    backend::rustup::process(crate_name).map_err(CratePathCliError::from)?;

                (crate_name, items)
            },
        };

        writer::write_items(crate_name, items, output_path, skip_init)?;

        Ok(())
    }
}

#[allow(unused, dead_code)]
pub fn process() -> Result<(), CratePathCliError> {
    let args = SpecificArgs::parse();

    args.command.process(&args.output_path, args.skip_init)?;

    Ok(())
}
