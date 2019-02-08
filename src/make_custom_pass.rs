extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::str;

pub fn make_custom_pass(amt_low: usize, amt_cap: usize, amt_num: usize, amt_sym: usize) -> String {
  let upper: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let lower: &[u8] = b"abcdefghijklmnopqurtuvwxyz";
  let numbers: &[u8] = b"0123456789";
  let symbols: &[u8] = b")(*&^%$#@!~";

  let mut raw_pass: String = "".to_string();
  let upper_list: Option<String> = (0..amt_cap)
    .map(|_| Some(*upper.choose(&mut thread_rng())? as char))
    .collect();
  let lower_list: Option<String> = (0..amt_low)
    .map(|_| Some(*lower.choose(&mut thread_rng())? as char))
    .collect();
  let number_list: Option<String> = (0..amt_num)
    .map(|_| Some(*numbers.choose(&mut thread_rng())? as char))
    .collect();
  let symbol_list: Option<String> = (0..amt_sym)
    .map(|_| Some(*symbols.choose(&mut thread_rng())? as char))
    .collect();

  raw_pass.push_str(&upper_list.unwrap());
  raw_pass.push_str(&lower_list.unwrap());
  raw_pass.push_str(&number_list.unwrap());
  raw_pass.push_str(&symbol_list.unwrap());

  let mut pass: Vec<u8> = raw_pass.into_bytes();

  pass.shuffle(&mut thread_rng());
  pass.shuffle(&mut thread_rng());

  str::from_utf8(&pass).unwrap().to_string()
}
