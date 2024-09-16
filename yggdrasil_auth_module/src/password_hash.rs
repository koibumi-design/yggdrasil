use crate::auth_provider::AuthError;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashError {
    IntoHashError,
    FromHashError,
}

impl Into<AuthError> for HashError {
    fn into(self) -> AuthError {
        match self {
            HashError::IntoHashError => {
                AuthError::VerifyAlgorithmError("Failed to hash password".to_owned())
            }
            HashError::FromHashError => AuthError::VerifyAlgorithmError(
                "Failed to verify password because of hash algorithm error.".to_owned(),
            ),
        }
    }
}

pub type IntoHashedFunction = fn(&str) -> Result<String, HashError>;
pub type VerifyHashedFunction = fn(&str, &str) -> Result<bool, HashError>;

pub struct HashFunction {
    pub into_hashed: IntoHashedFunction,
    pub verify: VerifyHashedFunction,
}

pub static BCRYPT_HASH_FUNCTION: HashFunction = HashFunction {
    into_hashed: hash_password_bcrypt,
    verify: verify_password_bcrypt,
};

pub static ARGON2_HASH_FUNCTION: HashFunction = HashFunction {
    into_hashed: hash_password_argon2,
    verify: verify_password_argon2,
};

fn hash_password_bcrypt(password: &str) -> Result<String, HashError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|_| HashError::IntoHashError)
}

fn verify_password_bcrypt(password: &str, hash: &str) -> Result<bool, HashError> {
    bcrypt::verify(password, hash).map_err(|_| HashError::FromHashError)
}

fn hash_password_argon2(password: &str) -> Result<String, HashError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(HashError::IntoHashError),
    }
}

fn verify_password_argon2(password: &str, hash: &str) -> Result<bool, HashError> {
    let parsed_hash = PasswordHash::new(&hash).map_err(|_| HashError::FromHashError)?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
