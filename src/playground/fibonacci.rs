#![deny(clippy::all)]

use napi_derive::napi;
use num_bigint::BigUint;
use num_traits::{One, Zero};

#[napi]
pub fn fibonacci(n: u32) -> String {
  let mut a: BigUint = BigUint::zero();
  let mut b: BigUint = BigUint::one();

  if n == 0 {
    return a.to_string();
  }
  if n == 1 {
    return b.to_string();
  }

  for _ in 2..=n {
    let temp = &a + &b;
    a = b;
    b = temp;
  }

  b.to_string()
}
