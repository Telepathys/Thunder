use std::sync::Mutex;

pub static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());