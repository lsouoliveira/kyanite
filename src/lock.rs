use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static KYA_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
