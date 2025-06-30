use axum::Json;
use base64::{Engine, engine::general_purpose};
use bs58;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct SignMessage {
    message: String,
    secret: String,
}

#[derive(Deserialize)]
pub struct VerifyMessage {
    message: String,
    signature: String,
    pubkey: String,
}

pub async fn sign_message(Json(body): Json<SignMessage>) -> Json<serde_json::Value> {
    let secret_bytes = match bs58::decode(&body.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid base58 secret key"
            }));
        }
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid secret key format"
            }));
        }
    };

    let signature = keypair.sign(body.message.as_bytes());

    Json(json!({
        "success": true,
        "data": {
            "signature": general_purpose::STANDARD.encode(signature.to_bytes()),
            "public_key": bs58::encode(keypair.public.to_bytes()).into_string(),
            "message": body.message
        }
    }))
}

pub async fn verify_message(Json(body): Json<VerifyMessage>) -> Json<serde_json::Value> {
    let pubkey_bytes = match bs58::decode(&body.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid base58 public key"
            }));
        }
    };

    let signature_bytes = match general_purpose::STANDARD.decode(&body.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid base64 signature"
            }));
        }
    };

    let pubkey = match PublicKey::from_bytes(&pubkey_bytes) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid public key format"
            }));
        }
    };

    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid signature format"
            }));
        }
    };

    let valid = pubkey.verify(body.message.as_bytes(), &signature).is_ok();

    Json(json!({
        "success": true,
        "data": {
            "valid": valid,
            "message": body.message,
            "pubkey": body.pubkey
        }
    }))
}
