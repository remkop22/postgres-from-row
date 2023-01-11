#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "postgres")]
mod active_postgres {
    pub use postgres::{Error, Row};
    pub use postgres_from_row_derive::FromRowPostgres as FromRow;
}

#[cfg(feature = "tokio-postgres")]
mod active_postgres {
    pub use postgres_from_row_derive::FromRowTokioPostgres as FromRow;
    pub use tokio_postgres::{Error, Row};
}

/// A trait that allows mapping rows from either [postgres](<https://docs.rs/postgres>) or [tokio-postgres](<https://docs.rs/tokio-postgres>), to other types.
#[cfg(any(feature = "postgres", feature = "tokio-postgres"))]
pub trait FromRow: Sized {
    /// Performce the conversion
    ///
    /// # Panics
    ///
    /// panics if the row does not contain the expected column names.
    fn from_row(row: &active_postgres::Row) -> Self;

    /// Try's to perform the conversion.
    ///
    /// Will return an error if the row does not contain the expected column names.
    fn try_from_row(row: &active_postgres::Row) -> Result<Self, active_postgres::Error>;
}

#[doc(hidden)]
pub use active_postgres::FromRow;

//
// #[cfg(all(feature = "postgres", feature = "tokio-postgres"))]
// compile_error!("Can't combine feature `postgres` and `tokio-postgres`");
//
// #[cfg(not(any(feature = "postgres", feature = "tokio-postgres")))]
// compile_error!("Must have at least one enabled feature: `postgres` or `tokio-postgres`.");
