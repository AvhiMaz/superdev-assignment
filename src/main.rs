use axum::{Router, routing::post};
use tokio::net::TcpListener;

mod handlers;
use handlers::*;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| {
        println!("No PORT env var set. Using default 3000");
        "3000".to_string()
    });
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
