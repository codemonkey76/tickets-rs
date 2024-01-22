use serde::Serialize;
use std::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	CtxCannotNewRootCtx,
}

// region:    --- Error Boilerplate
impl Display for Error {
	fn fmt(
		&self,
		f: &mut Formatter<'_>,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
