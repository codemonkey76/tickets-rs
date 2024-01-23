use crate::{Error, Result};
use base64_url::base64::DecodeError;
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env()
			.unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause {ex:?}"))
	})
}

#[allow(non_snake_case)]
pub struct Config {
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,

	// -- Db
	pub DB_URL: String,

	// -- Web
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Config> {
		Ok(Config {
			// -- Crypt
			PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

			TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,

			TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,

			// -- Web
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
	get_env(name)?
		.b64_decode()
		.map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	get_env(name)?
		.parse()
		.map_err(|_| Error::ConfigWrongFormat(name))
}

impl Base64Decode for String {}

trait Base64Decode {
	fn b64_decode(&self) -> core::result::Result<Vec<u8>, DecodeError>
	where
		Self: AsRef<str>,
	{
		base64_url::decode(self.as_ref())
	}
}
