use std::str;
mod helper;
use self::helper::{get_random_numbers_btw, get_random_numbers};

// extern crate wasm_bindgen;

struct Sortable {
    size: i32,
}

impl Sortable {
    fn new(size: i32) -> Sortable {
        Sortable { size: size }
    }
    fn get_sort<T>(&self, vec: Vec<T>) -> Vec<T> {
        let random_numbers = get_random_numbers(self.size);
        let sort_order: Vec<i32> = (0..self.size)
            .map(|i| random_numbers[i as usize] as i32)
            .collect();

        let mut sorted_vector: Vec<(i32, T)> =
            sort_order.into_iter().zip(vec.into_iter()).collect();

        sorted_vector.sort_by(|a, b| a.0.cmp(&b.0));
        sorted_vector.into_iter().map(|(_a, b)| b).collect()
    }
}

fn make_custom_pass(amt_low: usize, amt_cap: usize, amt_num: usize, amt_sym: usize) -> String {
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

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub extern "C" fn make_password(
    low: usize,
    cap: usize,
    num: usize,
    sym: usize,
) -> std::string::String {
    make_custom_pass(low, cap, num, sym)
}
