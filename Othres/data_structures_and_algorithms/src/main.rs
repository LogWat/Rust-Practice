pub mod bin_search;
pub mod basic_sort;

// use crate::bin_search::bin_search_test;
use crate::basic_sort::bubble_sort;
use rand::Rng;

fn main() {
    let n = 100;
    let mut array: Vec<u8> = Vec::with_capacity(n); // 100個分の領域確保
    let mut rng = rand::thread_rng();      // randomのinit

    for _ in 0..n {
        let i: u8 = rng.gen(); // generate
        array.push(i);
    }
    bubble_sort(&mut array);
    
    println!("{:?}", array);
}
