use chrono::{Duration, Utc};
use core::convert::TryFrom;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::{AsymmetricKeyPair, AsymmetricPublicKey, AsymmetricSecretKey};
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::{public, Public};
use uuid::Uuid;

use crate::errors::{AuthErrorTypes, Errors};

pub struct EndUserClaims {
    uuid: Uuid,
}

pub struct PasetoService {
    keypair: AsymmetricKeyPair<V4>,
    exp: Duration,
}

impl PasetoService {
    pub fn new(public_key: &[u8], private_key: &[u8], exp: Duration) -> Self {
        Self {
            keypair: AsymmetricKeyPair {
                public: AsymmetricPublicKey::from(public_key).unwrap(),
                secret: AsymmetricSecretKey::from(&[private_key, public_key].concat()).unwrap(),
            },
            exp,
        }
    }
}

pub trait AccessToken: Send {
    /// Generate access token.
    fn generate(&self, end_user_claims: EndUserClaims) -> String;

    /// Validate access token.
    fn validate(&self, token: &str) -> Result<EndUserClaims, Errors>;
}

impl AccessToken for PasetoService {
    fn generate(&self, end_user_claims: EndUserClaims) -> String {
        let mut claims: Claims = Claims::new().unwrap();
        claims
            .expiration(&(Utc::now() + self.exp).to_rfc3339())
            .unwrap();
        claims.subject(&end_user_claims.uuid.to_string()).unwrap();
        let token: String = public::sign(&self.keypair.secret, &claims, None, None).unwrap();

        return token;
    }

    fn validate(&self, token: &str) -> Result<EndUserClaims, Errors> {
        let validation_rules: ClaimsValidationRules = ClaimsValidationRules::new();
        let untrusted_token: UntrustedToken<Public, V4> =
            UntrustedToken::<Public, V4>::try_from(token).unwrap();
        let trusted_token: pasetors::token::TrustedToken = public::verify(
            &self.keypair.public,
            &untrusted_token,
            &validation_rules,
            None,
            None,
        )
        .map_err(|_| Errors::Auth(AuthErrorTypes::FailedValidateClaims))
        .unwrap();

        let claims: &Claims = trusted_token
            .payload_claims()
            .ok_or(Errors::Auth(AuthErrorTypes::InvalidToken))
            .unwrap();
        let uuid: &serde_json::Value = claims
            .get_claim("sub")
            .ok_or(Errors::Auth(AuthErrorTypes::InvalidToken))
            .unwrap();
        let uuid: String = serde_json::from_value::<String>(uuid.clone())
            .map_err(|_| Errors::Auth(AuthErrorTypes::InvalidToken))
            .unwrap();

        return Ok(EndUserClaims {
            uuid: Uuid::parse_str(&uuid).unwrap(),
        });
    }
}

#[cfg(test)]
mod tests {
    use ed25519::{
        pkcs8::{DecodePrivateKey, DecodePublicKey},
        KeypairBytes, PublicKeyBytes,
    };

    use super::*;

    lazy_static! {
        static ref EXP: Duration = Duration::minutes(1);
        static ref PUBLIC_KEY: Vec<u8> = PublicKeyBytes::from_public_key_pem(
            "-----BEGIN PUBLIC KEY-----\n\
            MCowBQYDK2VwAyEAsL3ElqFG7ELhKdqz82cExUkmu+t0fy2yPd7rmAxhn/Y=\n\
            -----END PUBLIC KEY-----"
        )
        .unwrap()
        .to_bytes()
        .into();
        static ref PRIVATE_KEY: Vec<u8> = KeypairBytes::from_pkcs8_pem(
            "-----BEGIN PRIVATE KEY-----\n\
            MC4CAQAwBQYDK2VwBCIEIKHLJM3ffad6X6Z9OflMjVo0kCxYbI7vlxKujreBderF\n\
              -----END PRIVATE KEY-----"
        )
        .unwrap()
        .secret_key
        .into();
    }

    #[test]
    fn test_generate_access_token() {
        let service: PasetoService = PasetoService::new(&PUBLIC_KEY, &PRIVATE_KEY, *EXP);
        let result: String = service.generate(EndUserClaims {
            uuid: Uuid::new_v4(),
        });

        assert_ne!(result, "");
    }

    #[test]
    fn test_validate_access_token() {
        let uuid: Uuid = Uuid::new_v4();
        let service: PasetoService = PasetoService::new(&PUBLIC_KEY, &PRIVATE_KEY, *EXP);
        let claims: EndUserClaims = EndUserClaims { uuid: uuid };
        let token: String = service.generate(claims);
        let result: EndUserClaims = service.validate(&token).unwrap();

        assert_eq!(result.uuid, uuid);
    }
}
