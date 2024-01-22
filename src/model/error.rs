use crate::model::store;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	EntityNotFound { entity: &'static str, id: i64 },
	// -- Modules
	Store(store::Error),

	// -- Externals
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:    --- Froms
impl From<sqlx::Error> for Error {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}
impl From<store::Error> for Error {
	fn from(value: store::Error) -> Self {
		Self::Store(value)
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
