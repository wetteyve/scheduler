use rusty::fibonacci;
use std::io;

fn main() {
  println!("Calc n-th fibonacci number");

  loop {
    println!("Enter n:");

    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let num: u32 = match input.as_str().trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    let result = fibonacci(num);

    println!("The {num}-th value of fibonacci is: {result}");
  }
}
