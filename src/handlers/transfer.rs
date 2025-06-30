use axum::{Json, http::StatusCode, response::IntoResponse};
use base64::{Engine, engine::general_purpose};
use serde::Deserialize;
use serde_json::json;
use solana_program::{pubkey::Pubkey, system_instruction};
use spl_associated_token_account::get_associated_token_address;

#[derive(Deserialize)]
pub struct SendSol {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Deserialize)]
pub struct SendToken {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

pub async fn send_sol(Json(payload): Json<SendSol>) -> impl IntoResponse {
    let from = match payload.from.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid sender address" })),
            );
        }
    };

    let to = match payload.to.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid recipient address" })),
            );
        }
    };

    let ix = system_instruction::transfer(&from, &to, payload.lamports);

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "program_id": ix.program_id.to_string(),
                "accounts": ix.accounts.iter().map(|a| a.pubkey.to_string()).collect::<Vec<_>>(),
                "instruction_data": general_purpose::STANDARD.encode(ix.data)
            }
        })),
    )
}

pub async fn send_token(Json(payload): Json<SendToken>) -> impl IntoResponse {
    let owner = match payload.owner.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid owner address" })),
            );
        }
    };

    let destination = match payload.destination.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid destination address" })),
            );
        }
    };

    let mint = match payload.mint.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid mint address" })),
            );
        }
    };

    let from_ata = get_associated_token_address(&owner, &mint);
    let to_ata = get_associated_token_address(&destination, &mint);

    let program_id = spl_token::id();
    let ix = match spl_token::instruction::transfer(
        &program_id,
        &from_ata,
        &to_ata,
        &owner,
        &[],
        payload.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": format!("Failed to create token transfer instruction: {}", e)
                })),
            );
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": {
                "program_id": ix.program_id.to_string(),
                "accounts": ix.accounts.iter().map(|a| {
                    json!({
                        "pubkey": a.pubkey.to_string(),
                        "isSigner": a.is_signer,
                        "isWritable": a.is_writable,
                    })
                }).collect::<Vec<_>>(),
                "instruction_data": general_purpose::STANDARD.encode(ix.data)
            }
        })),
    )
}
