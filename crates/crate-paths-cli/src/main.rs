use crate_paths_cli::{backend, error::CratePathCliError};

fn main() -> Result<(), CratePathCliError> {
    backend::process()
}
