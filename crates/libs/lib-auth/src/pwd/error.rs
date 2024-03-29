use serde::Serialize;
use derive_more::From;

use super::scheme;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, From)]
pub enum Error {
    PwdWithSchemeFailedParse,
    // -- Modules
    #[from]
    Scheme(scheme::Error)
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate