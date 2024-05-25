mod handlers;
mod models;
mod routes;

use routes::create_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    let app = create_router().await;

    axum::serve(listener, app).await.unwrap();
}
