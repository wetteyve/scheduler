fn main() {
  let x = 5;

  {
    let x = x + 1;
    println!("The value of x in the first-inner scope is: {x}");
  }

  {
    let x = x * 2;
    println!("The value of x in the second-inner scope is: {x}");
  }

  println!("The value of x is: {x}");

  // This works
  let spaces = "   ";
  let spaces = spaces.len();
  // This does not work: mismatched types
  // let mut spaces = "   ";
  // spaces = spaces.len()
  println!("The spaces length is: {spaces}");
}
