use clap::{Parser, Subcommand, ValueEnum};
use crate_paths_cli::error::CratePathCliError;
use heck::ToSnakeCase as _;
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

    /// Use a specific backend instead of cycling through all available backends
    #[arg(short, long, value_enum)]
    backend: Option<Backend>,

    /// Crate version (only used with docsrs backend)
    #[arg(long, default_value = "latest")]
    crate_version: String,
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

    let output_path = if args.output_path.extension().is_none() {
        args.output_path
            .join(format!("{}.rs", args.crate_name.to_snake_case()))
    } else {
        args.output_path
    };

    match args.backend {
        Some(Backend::Rustup) => {
            crate_paths_cli::backend::run_rustup(&args.crate_name, &output_path)
        },
        Some(Backend::Local) => crate_paths_cli::backend::run_local(&args.crate_name, &output_path),
        Some(Backend::Docsrs) => crate_paths_cli::backend::run_docsrs(
            &args.crate_name,
            &args.crate_version,
            &output_path,
        ),
        None => crate_paths_cli::backend::run_auto(&args.crate_name, &output_path),
    }
}
