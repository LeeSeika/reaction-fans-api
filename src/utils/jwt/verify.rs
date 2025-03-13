use std::sync::Arc;

use hmac::{Hmac, Mac};
use jwt::{Claims, VerifyWithKey};
use sha2::Sha256;

use crate::conf::Config;
use crate::errs;

pub fn verify(conf: Arc<Config>, token: String) -> Result<(), errs::jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(conf.server.secret.as_bytes())
        .map_err(|e| errs::jwt::Error::Unexpected(e.to_string()))?;
    let claims: Claims = token
        .verify_with_key(&key)
        .map_err(|e| errs::jwt::Error::Unexpected(e.to_string()))?;

    let now = chrono::Utc::now().timestamp() as u64;
    if let Some(exp) = claims.registered.expiration {
        if now >= exp {
            return Err(errs::jwt::Error::Invalid);
        }
    }

    Ok(())
}
