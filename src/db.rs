use axum::Extension;
use sqlx::{Error, pool::Pool, sqlite::{Sqlite, SqlitePoolOptions}};

use crate::pay::BenlnurlPayCallback;

pub struct BenlnurlDatabase {
    pub database: Pool<Sqlite>
}

pub async fn connect_to_database() -> Result<Pool<Sqlite>, Error> { 
    let db = SqlitePoolOptions::new().connect("benlnurl.db").await?;

    let crete_table_query = "
        create table if not exists benlnurl (
            username text not null unique,
            minSpendable integer not null,
            maxSpendable integer not null,
            address text not null,
            macaroon text not null,
            cert text not null
        )
    ";

    let _ = sqlx::query(crete_table_query).execute(&db).await?;

    Ok(db)
}

