use hmac::{Hmac, Mac};
use jwt::{Claims, RegisteredClaims, SignWithKey};
use sha2::Sha256;
use std::sync::Arc;

use crate::conf::Config;

pub fn sign(conf: Arc<Config>, uid: String) -> Result<String, Box<dyn std::error::Error>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(conf.server.secret.as_bytes())?;

    let claims = Claims::new(RegisteredClaims {
        issuer: Some(conf.server.issuer.clone()),
        expiration: Some(
            (chrono::Utc::now() + chrono::Duration::seconds(conf.server.expiration as i64))
                .timestamp() as u64,
        ),
        issued_at: Some(chrono::Utc::now().timestamp() as u64),
        subject: Some(uid),
        ..Default::default()
    });

    let token = claims.sign_with_key(&key)?;

    Ok(token)
}
