use std::fmt::Formatter;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Time
	DateFailParse(String),

	// -- Base64
	FailToB64uDecode,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}
// endregion: --- Error Boilerplate
