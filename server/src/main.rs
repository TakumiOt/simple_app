use anyhow::Ok;
use axum::{Router, http, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listner = TcpListener::bind(addr).await?;

    println!("Listning on: http://{}", addr);
    axum::serve(listner, app.into_make_service()).await.unwrap();

    Ok(())
}
