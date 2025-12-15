use std::io;

fn main() {
  let a = [1, 2, 3, 4, 5];

  loop {
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
      .read_line(&mut index)
      .expect("Failed to read line");

    let index: usize = match index.as_str().trim().parse::<usize>() {
      Ok(num) => {
        modulo(num);
        num % a.len()
      }
      Err(_) => continue,
    };

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
  }
}

fn modulo(num: usize) {
  if num.is_multiple_of(4) {
    println!("index is divisible by 4");
  } else if num.is_multiple_of(3) {
    println!("index is divisible by 3");
  } else if num.is_multiple_of(2) {
    println!("index is divisible by 2");
  } else {
    println!("index is not divisible by 4, 3, or 2");
  }
}
