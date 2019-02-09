use std::str;
extern crate js_sys;

// basic random number generator
// http://www.firstpr.com.au/dsp/rand31/
struct Rand31 {
  seed: u64,
}

impl Rand31 {
  fn new(seed: u64) -> Rand31 {
    Rand31 { seed: seed }
  }
  fn next(&mut self) -> u64 {
    let mut lo: u64 = 16807 * (self.seed & 0xFFFF);
    let hi: u64 = 16807 * (self.seed >> 16);
    lo += (hi & 0x7FFF) << 16;
    lo += hi >> 15;
    if lo > 0x7FFFFFFF {
      lo -= 0x7FFFFFFF;
    }
    self.seed = lo;
    self.seed
  }
}

fn get_milli_epoch() -> u64 {
  let epoch = js_sys::Date::now();
  epoch as u64
}

fn number_to_vec(n: u32) -> Vec<u32> {
  let mut digits = Vec::new();
  let mut n = n;
  while n > 9 {
    digits.push(n % 10);
    n = n / 10;
  }
  digits.push(n);
  digits.reverse();
  digits
}

fn get_last_n_digits(number: u32, amt: usize) -> u32 {
  let mut number_vec: Vec<u32> = number_to_vec(number);
  number_vec.reverse();
  number_vec = number_vec[..amt].iter().cloned().collect();
  number_vec.iter().sum()
}

fn skip_n(r: &mut Rand31, amt: u32) -> u64 {
  let mut val: u64 = 0;
  let mut n = amt;

  while n > 0 {
    val = r.next();
    n -= 1;
  }

  val
}

fn get_random_numbers(amt: i32) -> Vec<i32> {
  let mut random_numbers: Vec<i32> = vec![];

  // get unique seed on each request
  let seed: u64 = get_milli_epoch();

  for i in 0..amt {
    // use a variation of the seed each number
    let local_seed = seed + i as u64;
    let mut r = Rand31::new(local_seed);

    // use the last two digits of the seed to get the n-th generated number
    let random_number: i32 = skip_n(&mut r, get_last_n_digits(local_seed as u32, 2)) as i32;
    random_numbers.push(random_number);
  }

  random_numbers
}

struct Sortable {
  size: i32,
}

impl Sortable {
  fn new(size: i32) -> Sortable {
    Sortable { size: size }
  }
  fn get_sort<T>(&self, vec: Vec<T>) -> Vec<T> {
    let random_numbers = get_random_numbers(self.size);

    // generates a random number for each letter
    let sort_order: Vec<i32> = (0..self.size)
      .map(|i| random_numbers[i as usize] as i32)
      .collect();

    // pairs the random numbers with each letter, returning a tuple
    let mut sorted_vector: Vec<(i32, T)> = sort_order.into_iter().zip(vec.into_iter()).collect();

    // sorts the random numbers, and shuffles the letters
    sorted_vector.sort_by(|a, b| a.0.cmp(&b.0));

    // removes the numbers and returns shuffled letters
    sorted_vector.into_iter().map(|(_a, b)| b).collect()
  }
}

fn get_random_numbers_btw(max: i32, amt: i32) -> Vec<i32> {
  let mut random_numbers: Vec<i32> = vec![];

  // get unique seed on each request
  let seed: u64 = get_milli_epoch();

  for i in 0..amt {
    // use a variation of the seed each number
    let local_seed = seed + i as u64;
    let mut r = Rand31::new(local_seed);

    // use the last two digits of the seed to get the n-th generated number
    let random_number: i32 = skip_n(&mut r, get_last_n_digits(local_seed as u32, 2)) as i32;
    random_numbers.push(random_number % max);
  }

  random_numbers
}

pub fn make_custom_pass(amt_low: usize, amt_cap: usize, amt_num: usize, amt_sym: usize) -> String {
  let lower: &[u8] = b"abcdefghijklmnopqurtuvwxyz";
  let upper: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let number: &[u8] = b"0123456789";
  let symbol: &[u8] = b")(*&^%$#@!~";

  let total: usize = amt_low + amt_cap + amt_num + amt_sym;
  let sortable: Sortable = Sortable::new(total as i32);

  let mut pass: Vec<u8> = vec![];

  let lower_select = get_random_numbers_btw(26, amt_low as i32);
  let mut lower_list: Vec<u8> = (0..amt_low)
    .map(|i| lower[lower_select[i] as usize])
    .collect();
  pass.append(&mut lower_list);

  let upper_select = get_random_numbers_btw(26, amt_cap as i32);
  let mut upper_list: Vec<u8> = (0..amt_cap)
    .map(|i| upper[upper_select[i] as usize])
    .collect();
  pass.append(&mut upper_list);

  let number_select = get_random_numbers_btw(10, amt_num as i32);
  let mut number_list: Vec<u8> = (0..amt_num)
    .map(|i| number[number_select[i] as usize])
    .collect();
  pass.append(&mut number_list);

  let symbol_select = get_random_numbers_btw(11, amt_sym as i32);
  let mut symbol_list: Vec<u8> = (0..amt_sym)
    .map(|i| symbol[symbol_select[i] as usize])
    .collect();
  pass.append(&mut symbol_list);

  pass = sortable.get_sort(pass);
  pass = sortable.get_sort(pass);

  str::from_utf8(&pass).unwrap().to_string()
}
