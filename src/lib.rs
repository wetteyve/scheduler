#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod playground;

pub use playground::*;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn get_array_length(arr: Vec<Unknown>) -> u32 {
  arr.len() as u32
}
