#[cfg(feature = "postgres")]
use postgres::{Error, Row};

#[cfg(feature = "tokio-postgres")]
use tokio_postgres::{Error, Row};

#[cfg(feature = "postgres")]
pub use postgres_from_row_derive::FromRowPostgres as FromRow;

#[cfg(feature = "tokio-postgres")]
pub use postgres_from_row_derive::FromRowTokioPostgres as FromRow;

#[cfg(any(feature = "postgres", feature = "tokio-postgres"))]
pub trait FromRow: Sized {
    fn from_row(row: &Row) -> Self;
    fn try_from_row(row: &Row) -> Result<Self, Error>;
}

#[cfg(all(feature = "postgres", feature = "tokio-postgres"))]
compile_error!("Can't combine feature `postgres` and `tokio-postgres`");

#[cfg(not(any(feature = "postgres", feature = "tokio-postgres")))]
compile_error!("Must have at least one enabled feature: `postgres` or `tokio-postgres`.");
