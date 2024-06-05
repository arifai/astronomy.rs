use argon2::{
    self,
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

use crate::errors::{AuthErrorTypes, Errors};

/// Generate password to hash.
pub fn generate_pass_hash(password: &str) -> String {
    let salt: SaltString = SaltString::generate(&mut OsRng);
    let password_hash: PasswordHash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap_or_else(|_| panic!("{}", Errors::Auth(AuthErrorTypes::FailedGeneratePassword)));

    return password_hash.to_string();
}

/// Verify password.
pub fn pass_hashed_verify(password_hashed: &str, password: &str) -> bool {
    let parsed_hashed: PasswordHash = PasswordHash::new(&password_hashed).unwrap();
    let verify_password: Result<(), Error> =
        Argon2::default().verify_password(password.as_bytes(), &parsed_hashed);

    return verify_password.is_ok();
}

mod tests {
    #[test]
    fn test_generate_pass_hash() {
        let pass_hashed: String = super::generate_pass_hash("12345678");

        assert_ne!(pass_hashed, "");
        assert!(pass_hashed.len() > 10)
    }

    #[test]
    fn test_pass_hashed_verify() {
        let pass_hashed: String = super::generate_pass_hash("12345678");

        assert_eq!(super::pass_hashed_verify(&pass_hashed, "12345678"), true);
    }
}
