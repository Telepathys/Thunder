use sha1::{Sha1, Digest};
use sha256::{digest};

pub fn sha(pw: &str) -> String {
    digest(pw.as_bytes())
}

pub fn sha1(data: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}