use crate::crypt;
use crate::model::store;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	EntityNotFound { entity: &'static str, id: i64 },
	// -- Modules
	Crypt(crypt::Error),
	Store(store::Error),

	// -- Externals
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:    --- Froms
impl From<crypt::Error> for Error {
	fn from(value: crypt::Error) -> Self {
		Self::Crypt(value)
	}
}
impl From<store::Error> for Error {
	fn from(value: store::Error) -> Self {
		Self::Store(value)
	}
}
impl From<sqlx::Error> for Error {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
