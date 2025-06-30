use axum::Json;
use base64::{Engine, engine::general_purpose};
use serde::Deserialize;
use serde_json::json;
use solana_program::pubkey::Pubkey;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    mintAuthority: String,
    mint: String,
    decimals: u8,
}

#[derive(Deserialize)]
pub struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> Json<serde_json::Value> {
    let mint = match payload.mint.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mint address"
            }));
        }
    };

    let mint_authority = match payload.mintAuthority.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mint authority"
            }));
        }
    };

    let program_id = spl_token::id();

    let ix = match spl_token::instruction::initialize_mint(
        &program_id,
        &mint,
        &mint_authority,
        None,
        payload.decimals,
    ) {
        Ok(ix) => ix,
        Err(e) => {
            return Json(json!({
                "success": false,
                "error": format!("Failed to create instruction: {}", e)
            }));
        }
    };

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": ix.accounts.iter().map(|a| {
                json!({
                    "pubkey": a.pubkey.to_string(),
                    "is_signer": a.is_signer,
                    "is_writable": a.is_writable
                })
            }).collect::<Vec<_>>(),
            "instruction_data": general_purpose::STANDARD.encode(ix.data)
        }
    }))
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> Json<serde_json::Value> {
    let mint = match payload.mint.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mint address"
            }));
        }
    };

    let destination = match payload.destination.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid destination address"
            }));
        }
    };

    let authority = match payload.authority.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid authority address"
            }));
        }
    };

    let program_id = spl_token::id();

    let ix = match spl_token::instruction::mint_to(
        &program_id,
        &mint,
        &destination,
        &authority,
        &[],
        payload.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => {
            return Json(json!({
                "success": false,
                "error": format!("Failed to create instruction: {}", e)
            }));
        }
    };

    Json(json!({
        "success": true,
        "data": {
            "program_id": ix.program_id.to_string(),
            "accounts": ix.accounts.iter().map(|a| {
                json!({
                    "pubkey": a.pubkey.to_string(),
                    "is_signer": a.is_signer,
                    "is_writable": a.is_writable
                })
            }).collect::<Vec<_>>(),
            "instruction_data": general_purpose::STANDARD.encode(ix.data)
        }
    }))
}
