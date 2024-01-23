use crate::web::Error::Model;
use crate::web::Error::{
	CtxExt, LoginFailPwdNotMatching, LoginFailUserHasNoPwd,
	LoginFailUsernameNotFound,
};
use crate::{model, pwd, web};
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
	// -- RPC
	RpcMethodUnknown(String),
	RpcMissingParams { rpc_method: String },
	RpcFailJsonParams { rpc_method: String },

	//  -- Login
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd { user_id: i64 },
	LoginFailPwdNotMatching { user_id: i64 },

	// -- CtxExtError
	CtxExt(web::mw_auth::CtxExtError),

	// -- Modules
	Model(model::Error),
	Crypt(pwd::Error),

	// -- External Modules
	SerdeJson(String),
}

// region:    --- Froms
impl From<serde_json::Error> for Error {
	fn from(value: serde_json::Error) -> Self {
		Self::SerdeJson(value.to_string())
	}
}
impl From<pwd::Error> for Error {
	fn from(value: pwd::Error) -> Self {
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

			// -- Model
			Model(model::Error::EntityNotFound { entity, id }) => (
				StatusCode::BAD_REQUEST,
				ClientError::ENTITY_NOT_FOUND { entity, id: *id },
			),
			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
	SERVICE_ERROR,
}
// endregion: --- Client Error
