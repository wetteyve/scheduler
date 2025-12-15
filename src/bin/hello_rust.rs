use rusty::hello_napi;
use std::env;

// execute -> cargo run --bin hello_rust

fn main() {
  let input = env::args().nth(1);
  println!("{}", hello_napi(input));
}
