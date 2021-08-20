pub mod bin_search;

// use crate::bin_search::bin_search_test;

use rand::Rng;

fn main() {
    let mut array: Vec<u8> = vec![];
    let mut rng = rand::thread_rng(); // init

    for _ in 0..10 {
        let i: u8 = rng.gen(); // generate
        array.push(i);
    }
}
