mod router;
mod handlers;

use std::error::Error;

use std::net::SocketAddr;

use sqlx::PgPool;
use sqlx::Row;

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

    let router = router::router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
