use std::error::Error;

use sqlx::postgres::{PgConnection, PgPool};
use sqlx::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(sqlx::FromRow)]
struct Row {}

// The application deals with a B
// This is using the Repository design pattern. The app deals with a
// `Box<dyn Repository>` that encapsulates all of the logic for
// interacting with the datastore. There is a sqlx implementation and
// other, non-sqlx implementations.
trait Repository {
    fn fetch_rows(&self) -> Result<Vec<Row>>;
}

struct Database {
    pool: PgPool,
}

impl Repository for Database {
    #[tokio::main]
    async fn fetch_rows(&self) -> Result<Vec<Row>> {
        let mut conn = self.pool.acquire().await?;
        let mut inner = Connection { conn: &mut conn };
        let rows = inner.fetch_rows().await?;
        Ok(rows)
    }
}

struct Connection<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> Connection<'a> {
    async fn fetch_rows(&mut self) -> Result<Vec<Row>> {
        let sql = "select * from table";
        let rows = sqlx::query_as(sql).fetch_all(&mut *self.conn).await?;
        Ok(rows)
    }
}

#[tokio::main]
async fn wrap_in_tx(pool: PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;
    let mut conn = Connection { conn: &mut tx };

    let _rows = conn.fetch_rows().await?;

    tx.rollback().await?;

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
