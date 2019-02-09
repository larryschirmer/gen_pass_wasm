use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_milli_epoch() -> u64 {
  let epoch_duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
  let mut epoch: u64 = 0;

  {
    epoch += epoch_duration.as_secs() * 1000;
  }
  {
    epoch += epoch_duration.subsec_nanos() as u64 / 1_000_000;
  }

  epoch
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

struct Rand31 {
  seed: u64,
}

impl Rand31 {
  pub fn new(seed: u64) -> Rand31 {
    Rand31 { seed: seed }
  }
  pub fn next(&mut self) -> u64 {
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

fn skip_n(r: &mut Rand31, amt: u32) -> u64 {
  let mut val: u64 = 0;
  let mut n = amt;

  while n > 0 {
    val = r.next();
    n -= 1;
  }

  val
}

pub fn get_random_numbers(amt: i32) -> Vec<i32> {
  let mut random_numbers: Vec<i32> = vec![];
  let mut last_seed: u64 = 0;

  for _ in 0..amt {
    let mut seed: u64 = get_milli_epoch();

    while seed == last_seed {
      seed = get_milli_epoch();
    }
    last_seed = seed;

    let mut r = Rand31::new(seed as u64);

    let random_number: i32 = skip_n(&mut r, get_last_n_digits(seed as u32, 2)) as i32;

    random_numbers.push(random_number);
  }

  random_numbers
}

pub fn get_random_numbers_btw(max: i32, amt: i32) -> Vec<i32> {
  let mut random_numbers: Vec<i32> = vec![];
  let mut last_seed: u64 = 0;

  for _ in 0..amt {
    let mut seed: u64 = get_milli_epoch();

    while seed == last_seed {
      seed = get_milli_epoch();
    }
    last_seed = seed;

    let mut r = Rand31::new(seed as u64);

    let random_number: i32 = skip_n(&mut r, get_last_n_digits(seed as u32, 2)) as i32;

    random_numbers.push(random_number % max);
  }

  random_numbers
}