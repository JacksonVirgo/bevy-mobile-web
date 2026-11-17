use server::routes;
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let assets = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");
    let app = routes::router().nest_service("/assets", ServeDir::new(assets));
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    info!("Listening on {address}");
    let _ = axum::serve(listener, app).await;
}
