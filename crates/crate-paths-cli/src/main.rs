mod backend;
mod consts;
mod error;
mod writer;

use error::CratePathCliError;

fn main() -> Result<(), CratePathCliError> {
    crate::backend::process()
}
