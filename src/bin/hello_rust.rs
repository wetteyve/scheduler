use std::env;

// execute -> cargo run --bin hello_rust

fn main() {
  let input = env::args().nth(1).unwrap_or_else(|| "rust".to_string());
  println!("Hello, {}!", input);
}
