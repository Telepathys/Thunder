use sha256::{digest};

pub fn sha(pw: &str) -> String {
    digest(pw.as_bytes())
}