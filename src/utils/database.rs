use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use std::{env, time::Duration};

use crate::errors::{DatabaseErroTypes, Errors};

pub type DBConnManager = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref DB_CONN_POLL: r2d2::Pool<ConnectionManager<PgConnection>> = {
        let conn_man = ConnectionManager::<PgConnection>::new(
            env::var("DATABASE_URL")
                .map_err(|_| Errors::Database(DatabaseErroTypes::NoDatabaseUrl))
                .unwrap(),
        );

        r2d2::Pool::builder()
            .connection_timeout(Duration::from_secs(60))
            .test_on_check_out(true)
            .build(conn_man)
            .map_err(|_| Errors::Database(DatabaseErroTypes::FailedConnectionPool))
            .unwrap()
    };
}

pub fn connect(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .map_err(|_| {
            Errors::Database(DatabaseErroTypes::InvalidDatabasUrl(
                database_url.to_string(),
            ))
        })
        .unwrap()
}

pub fn clone() -> DBConnManager {
    DB_CONN_POLL.clone()
}
