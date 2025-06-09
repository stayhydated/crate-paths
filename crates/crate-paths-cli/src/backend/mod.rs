pub mod specific;
pub mod transitive;

use crate::error::CratePathCliError;

#[cfg(feature = "specific")]
pub fn process() -> Result<(), CratePathCliError> {
    crate::backend::specific::process()
}

#[cfg(not(feature = "specific"))]
pub fn process() -> Result<(), CratePathCliError> {
    crate::backend::transitive::process()
}
