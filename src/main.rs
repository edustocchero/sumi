mod router;
mod handlers;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router = router::router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
