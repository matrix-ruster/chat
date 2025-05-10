use crate::error::AppError;
use crate::User;
use jwt_simple::prelude::*;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";

#[derive(Serialize, Deserialize, Debug)]
pub struct Claim {
    pub id: i64,
    pub nickname: String,
    pub email: String,
}

pub struct EncodingKey(Ed25519KeyPair);

impl Debug for EncodingKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.to_pem()).finish()
    }
}

impl Deref for EncodingKey {
    type Target = Ed25519KeyPair;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }
    pub fn generate(&self, user: &User) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(
            Claim {
                id: user.id,
                nickname: user.nickname.to_owned(),
                email: user.email.to_owned(),
            },
            Duration::from_secs(JWT_DURATION),
        )
        .with_issuer(JWT_ISS)
        .with_audience(JWT_AUD);
        Ok(self.sign(claims)?)
    }
}

#[derive(Debug)]
pub struct DecodingKey(Ed25519PublicKey);

impl Deref for DecodingKey {
    type Target = Ed25519PublicKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    pub fn verify(&self, token: &str) -> Result<Claim, AppError> {
        let mut opts = VerificationOptions::default();
        opts.allowed_audiences = Some(HashSet::from_strings(&[JWT_AUD]));
        opts.allowed_issuers = Some(HashSet::from_strings(&[JWT_ISS]));
        let claims = self.verify_token::<Claim>(token, Some(opts))?;
        Ok(claims.custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::jwt::{DecodingKey, EncodingKey};
    use anyhow::Result;
    #[test]
    fn generate_and_verify() -> Result<()> {
        let encoding_pem = include_str!("../../../private.pem");
        let decoding_pem = include_str!("../../../public.pem");
        let user = User {
            id: 1,
            username: "test".to_string(),
            nickname: "xxx".to_string(),
            password: "test".to_string(),
            email: "johnsonsmile@163.com".to_string(),
            created_at: Default::default(),
        };
        let ek = EncodingKey::load(&encoding_pem)?;
        let dk = DecodingKey::load(&decoding_pem)?;
        let token = ek.generate(&user)?;
        dbg!("token: {}", &token);
        let claim = dk.verify(&token)?;
        dbg!("claim: {}", claim);
        Ok(())
    }
}
