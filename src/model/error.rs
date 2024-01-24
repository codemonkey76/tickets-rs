use crate::model::store;
use crate::pwd;
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
	ListLimitOverMax {
		max: i64,
		actual: i64,
	},

	EntityNotFound {
		entity: &'static str,
		id: i64,
	},

	// -- Modules
	#[from]
	Pwd(pwd::Error),
	#[from]
	Store(store::Error),

	// -- Externals
	#[from]
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

	#[from]
	SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),

	#[from]
	ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
