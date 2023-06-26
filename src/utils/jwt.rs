use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::structs::users_struct::{
    TokenInput,
};
use chrono::{Utc, FixedOffset, Duration};

pub fn create_jwt(token_input: TokenInput) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("uuid", token_input.uuid);
    claims.insert("id", token_input.id);
    claims.insert("name", token_input.name);

    let kst_offset = {
        let secs = 9 * 3600;
        FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
    };  // UTC+9 (한국)
    let now = Utc::now().with_timezone(&kst_offset);
    let expiration = now + Duration::hours(24);
    claims.insert("exp", expiration.timestamp().to_string());

    let token = claims.sign_with_key(&key).unwrap();
    token
}

pub fn verify_toekn(token: String) {

}