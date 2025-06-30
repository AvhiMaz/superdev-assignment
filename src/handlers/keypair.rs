use axum::Json;
use bs58;
use serde_json::json;
use ed25519_dalek::Keypair;
use getrandom::getrandom;



pub async fn generate_keypair() -> Json<serde_json::Value> {
    let mut keypair_bytes = [0u8; 64];
    if let Err(e) = getrandom(&mut keypair_bytes) {
        println!("getrandom failed: {:?}", e);
        return Json(json!({ "success": false, "error": "Randomness failed" }));
    }

    let keypair = match Keypair::from_bytes(&keypair_bytes) {
        Ok(kp) => kp,
        Err(e) => {
            println!("Failed to create keypair: {:?}", e);
            return Json(json!({ "success": false, "error": "Invalid keypair bytes" }));
        }
    };

    let pubkey = bs58::encode(keypair.public.to_bytes()).into_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    Json(json!({
        "success": true,
        "data": {
            "pubkey": pubkey,
            "secret": secret
        }
    }))
}

