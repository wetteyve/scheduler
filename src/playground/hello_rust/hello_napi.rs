#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn hello_napi(input: Option<String>) {
    let input = input.unwrap_or_else(|| "napi-rs".to_string());
    println!("Hello, {}!", input);

}
