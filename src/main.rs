mod router;
mod handlers;
mod models;
mod contracts;

use std::error::Error;

use std::net::SocketAddr;
use std::sync::Arc;

use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;

pub struct Env {
    pub db: Pool<Postgres>,
}

pub type AppState = Arc<Env>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn_url = "postgres://postgres:postgres@localhost:5432/sumi_dev";
    let pool = PgPool::connect(conn_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;

    let sum: i32 = res.get("sum");
    println!("{}", sum);

    let env = Env { db: pool.clone() };
    let state = Arc::new(env);
        
    let router = router::router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
