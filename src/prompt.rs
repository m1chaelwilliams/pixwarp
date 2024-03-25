#[cfg(feature = "headless")]
use std::{io, str::FromStr};

#[cfg(feature = "headless")]
pub fn io_get_strip() -> Option<String> {
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => Some(buf.trim().to_string()),
        Err(_) => None
    }
}

#[cfg(feature = "headless")]
pub fn prompt<T, F>(validate_fn: F) -> T 
where
    T: FromStr,
    F: Fn(String) -> Option<T>
{
    loop {
        if let Some(raw_response) = io_get_strip() {
            if let Some(validated_response) = validate_fn(raw_response) {
                return validated_response;
            }
        }
    }
}