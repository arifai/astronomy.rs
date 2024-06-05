use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Authentication error: {0}")]
    Auth(AuthErrorTypes),

    #[error("Database error: {0}")]
    Database(DatabaseErroTypes),

    #[error(transparent)]
    External(#[from] anyhow::Error),
}

impl Errors {
    pub fn log(&self) {
        match self {
            Errors::External(_) => eprint!("{:?}", self),
            _ => println!("{}", self),
        }
    }
}

#[derive(Error, Debug)]
pub enum AuthErrorTypes {
    #[error("Invalid token authentication.")]
    InvalidToken,

    #[error("Failed to generate password hash.")]
    FailedGeneratePassword,

    #[error("Failed to validate claims.")]
    FailedValidateClaims,
}

#[derive(Error, Debug)]
pub enum DatabaseErroTypes {
    #[error("No `DATABASE_URL` in environment.")]
    NoDatabaseUrl,

    #[error("Failed connect to database, please check your `DATABASE_URL`: {0}.")]
    InvalidDatabasUrl(String),

    #[error("Could not build connection pool.")]
    FailedConnectionPool,
}
