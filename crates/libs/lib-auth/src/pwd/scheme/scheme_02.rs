use argon2::{password_hash::SaltString, Algorithm, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};

use crate::{config::auth_config, pwd};

use super::{Error, Result, Scheme};

pub struct Scheme03;

impl Scheme for Scheme02 {
    fn hash(&self, to_hash: &crate::pwd::ContentToHash) -> Result<String> {
        let argon2 = get_argon2();

        let salt_b64 = SaltString::encode_b64(to_hash.salt.as_bytes())
            .map_err(|_| Error::Salt)?;

        let pwd = argon2
            .hash_password(to_hash.content.as_bytes(), &salt_b64)
            .map_err(|_| Error::Hash)?
            .to_string();

        Ok(pwd)
    }

    fn validate(&self, to_hash: &crate::pwd::ContentToHash, pwd_ref: &str) -> Result<()> {
        let argon2 = get_argon2();

        let parsed_hash_ref = PasswordHash::new(pwd_ref).map_err(|_| Error::Hash)?;

        argon2.verify_password(to_hash.content.as_bytes(), parsed_hash_ref)
        .map_err(|_| Error::PwdValidate)
        
    }
}

fn get_argon2() -> &'static Argon2<'static> {
    static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

    let val = INSTANCE.get_or_init(|| {
        let key = &auth_config().PWD_KEY;
        Argon2::new_with_secret(
            key,
            Algorithm::Argon2id,
            Version::V0x13,
            Params::default()
        ).unwrap() // TODO - needs to fail early
    });
    todo!();
}

// region:    --- Tests

#[cfg(test)]
mod tests  {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_scheme_02-hash_into_b64u_ok() -> Result<()> {
        // -- Setup & Fixtures
    }
}

// endregion: --- Tests