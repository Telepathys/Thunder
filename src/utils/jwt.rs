use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::game::components::user::user_component::{
    TokenInput,
};
use chrono::{Utc, FixedOffset, Duration};
use log::{error};

pub fn create_jwt(token_input: TokenInput) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("uuid", token_input.uuid);
    claims.insert("id", token_input.id);
    claims.insert("name", token_input.name);

    let kst_offset = {
        let secs = 9 * 3600;
        FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
    };
    let now = Utc::now().with_timezone(&kst_offset);
    let expiration = now + Duration::hours(24);
    claims.insert("exp", expiration.timestamp().to_string());

    let token = claims.sign_with_key(&key).unwrap();
    token
}

pub fn verify_token(token: &str) -> Result<BTreeMap<String, String>, String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();

    // 토큰 검증 및 페이로드 추출
    let claims: BTreeMap<String, String> = token.verify_with_key(&key).unwrap();
    
    // 만료 시간 확인
    if let Some(expiration) = claims.get("exp") {
        let expiration = match expiration.parse::<i64>() {
            Ok(exp) => exp,
            Err(err) => return Err(format!("Invalid expiration time: {}", err)),
        };
        let kst_offset = {
            let secs = 9 * 3600;
            FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
        };
        let now = Utc::now().with_timezone(&kst_offset).timestamp();
        // info!("expiration: {}", expiration);
        // info!("now: {}", now);
        if now >= expiration {
            error!("Token has expired");
            return Err("Token has expired".to_string());
        }
    } else {
        return Err("Missing expiration time in token".to_string());
    }

    let payload: BTreeMap<String, String> = claims.into();
    Ok(payload)
}