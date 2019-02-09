// extern crate wasm_bindgen;

// fn get_rand_number(amt: i32) -> Vec<i32> {
    // let mut numbers: Vec<i32> = vec![];

    // for _i in 0..amt {
    //     let num = vec![2, 3];
    //     let addr = &num as *const Vec<i32>;
    //     let mut number = addr as i32;
    //     number = (number % 1000).abs();
    //     numbers.push(number);
    // }
    // numbers
// }

// struct Sortable {
//     size: i32,
// }

// impl Sortable {
//     fn new(size: i32) -> Sortable {
//         Sortable { size: size }
//     }
//     fn get_sort<T>(&self, vec: Vec<T>) -> Vec<(i32, T)> {
//         // Vec<T>
//         // Vec<(i32, T)>

//         let random_numbers = get_rand_number(self.size);
//         let sort_order: Vec<i32> = (0..self.size).map(|i| random_numbers[i as usize]).collect();

//         let mut sorted_vector: Vec<(i32, T)> =
//             sort_order.into_iter().zip(vec.into_iter()).collect();

//         sorted_vector

//         // sorted_vector.sort_by(|a, b| a.0.cmp(&b.0));
//         // sorted_vector.into_iter().map(|(_a, b)| b).collect()
//     }
// }

// fn make_custom_pass(amt_low: usize, amt_cap: usize, amt_num: usize, amt_sym: usize) -> String {
//     let upper: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
//     let lower: &[u8] = b"abcdefghijklmnopqurtuvwxyz";
//     let numbers: &[u8] = b"0123456789";
//     let symbols: &[u8] = b")(*&^%$#@!~";

//     let total: usize = amt_low + amt_cap + amt_num + amt_sym;
//     let mut raw_pass: String = "".to_string();

//     let sortable: Sortable = Sortable::new(total as i32);

//     // raw_pass.push_str(&upper_list.unwrap());
//     // raw_pass.push_str(&lower_list.unwrap());
//     // raw_pass.push_str(&number_list.unwrap());
//     // raw_pass.push_str(&symbol_list.unwrap());

//     let mut pass: Vec<u8> = raw_pass.into_bytes();

//     // pass.shuffle(&mut thread_rng());
//     // pass.shuffle(&mut thread_rng());

//     // str::from_utf8(&pass).unwrap().to_string()
// }

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub extern "C" fn make_password(
//     low: usize,
//     cap: usize,
//     num: usize,
//     sym: usize,
// ) -> std::string::String {
//     make_custom_pass(low, cap, num, sym)
// }

fn main() {
    // let sortable = Sortable::new(4);
    // let sorted = sortable.get_sort(vec![
    //     "A".to_string(),
    //     "B".to_string(),
    //     "C".to_string(),
    //     "D".to_string(),
    // ]);

    // println!("{:?}", sorted);

    let mut rand_numbers: Vec<u32> = vec![];

    let numbers: Vec<u32> = vec![0, 1];

    for i in 0..numbers.len() {
        let addr = &numbers[i] as *const u32;
        let num = addr as u32;
        rand_numbers.push(num % 122);
    }

    println!("{:?}", rand_numbers);
}
