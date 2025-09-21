use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) exp: Option<i64>, // Expiration time as Unix timestamp
    sub: Option<String>,
    iat: Option<i64>,
}

pub(crate) fn decode_jwt(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT format".into());
    }

    let payload = parts[1];
    let decoded = base64_decode(payload)?;
    let claims: Claims = serde_json::from_slice(&decoded)?;

    Ok(claims)
}

// Simple base64 decode helper (you might want to use base64 crate instead)
fn base64_decode(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // JWT uses base64url encoding, need to handle padding
    let padded = match input.len() % 4 {
        2 => format!("{}==", input),
        3 => format!("{}=", input),
        _ => input.to_string(),
    };

    Ok(general_purpose::STANDARD.decode(padded)?)
}
