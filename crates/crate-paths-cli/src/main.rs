use clap::{Parser, Subcommand, ValueEnum};
use crate_paths_cli::error::CratePathCliError;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: CargoCommand,
}

#[derive(Subcommand)]
enum CargoCommand {
    /// Build a tree of a crate's item paths
    #[command(name = "crate-paths", version)]
    CratePaths(CratePathsArgs),
}

#[derive(Parser)]
struct CratePathsArgs {
    /// Crate name, e.g. "clap" or "std"
    #[arg(short, long)]
    crate_name: String,

    /// Output path for the generated paths tree
    #[arg(short, long)]
    output_path: PathBuf,

    /// Use a specific backend instead of auto-detection
    #[arg(short, long, value_enum)]
    backend: Option<Backend>,

    /// Crate version (only used with docsrs backend)
    #[arg(long, default_value = "latest")]
    crate_version: String,

    /// Skip local lookup in auto-detection
    #[arg(long)]
    skip_local: bool,

    /// Skip calling `cargo init` when the output path is a directory
    #[arg(long)]
    skip_init: bool,
}

#[derive(Clone, ValueEnum)]
enum Backend {
    /// Fetch from rustup (for std library crates)
    Rustup,
    /// Fetch locally (crate must be a dependency in current workspace)
    Local,
    /// Fetch from docs.rs
    Docsrs,
}

fn main() -> Result<(), CratePathCliError> {
    let cli = Cli::parse();
    let CargoCommand::CratePaths(args) = cli.command;

    match args.backend {
        Some(Backend::Rustup) => crate_paths_cli::backend::run_rustup(
            &args.crate_name,
            &args.output_path,
            args.skip_init,
        ),
        Some(Backend::Local) => {
            crate_paths_cli::backend::run_local(&args.crate_name, &args.output_path, args.skip_init)
        },
        Some(Backend::Docsrs) => crate_paths_cli::backend::run_docsrs(
            &args.crate_name,
            &args.crate_version,
            &args.output_path,
            args.skip_init,
        ),
        None => {
            crate_paths_cli::backend::run_auto(&args.crate_name, &args.output_path, args.skip_local)
        },
    }
}
