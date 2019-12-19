use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize};

#[derive(Deserialize)]
struct Event { }

fn main() {
    lambda!(handler)
}

fn handler(
    _: Event,
    _: Context,
) -> Result<String, HandlerError> {
    Ok(String::from("hello world"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_returns_message() {
        assert_eq!(
            handler(Event { }, Context::default()).expect("expected Ok(_) value"),
            "hello world"
        )
    }
}