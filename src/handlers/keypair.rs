use axum::{Json, http::StatusCode, response::IntoResponse};
use bs58;
use serde_json::json;
use ed25519_dalek::Keypair;
use getrandom::getrandom;

pub async fn generate_keypair() -> impl IntoResponse {
    let mut keypair_bytes = [0u8; 64];
    if let Err(e) = getrandom(&mut keypair_bytes) {
        eprintln!("getrandom failed: {:?}", e);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Randomness failed" })),
        );
    }

    let keypair = match Keypair::from_bytes(&keypair_bytes) {
        Ok(kp) => kp,
        Err(e) => {
            eprintln!("Failed to create keypair: {:?}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid keypair bytes" })),
            );
        }
    };

    let pubkey = bs58::encode(keypair.public.to_bytes()).into_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "pubkey": pubkey,
                "secret": secret
            }
        })),
    )
}
