use crate::web::Error::{
	CtxExt, LoginFailPwdNotMatching, LoginFailUserHasNoPwd,
	LoginFailUsernameNotFound,
};
use crate::{crypt, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt::Formatter;
use std::sync::Arc;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	//  -- Login
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd { user_id: i64 },
	LoginFailPwdNotMatching { user_id: i64 },

	// -- CtxExtError
	CtxExt(web::mw_auth::CtxExtError),

	// -- Modules
	Model(model::Error),
	Crypt(crypt::Error)
}

// region:    --- Froms
impl From<crypt::Error> for Error {
	fn from(value: crypt::Error) -> Self {
		Self::Crypt(value)
	}
}
impl From<model::Error> for Error {
	fn from(value: model::Error) -> Self {
		Self::Model(value)
	}
}
// endregion: --- Froms

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!("{:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum response.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		response.extensions_mut().insert(Arc::new(self));

		response
	}
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Client Error
impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		#[allow(unreachable_patterns)]
		match self {
			// -- Login
			LoginFailUsernameNotFound
			| LoginFailUserHasNoPwd { .. }
			| LoginFailPwdNotMatching { .. } => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}

			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	SERVICE_ERROR,
}
// endregion: --- Client Error
