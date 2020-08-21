use std::error::Error;

use sqlx::postgres::PgPool;
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
        let sql = "select * from table";
        let rows = sqlx::query_as(sql).fetch_all(&self.pool).await?;
        Ok(rows)
    }
}

fn main() {
    println!("Hello, world!");
}
