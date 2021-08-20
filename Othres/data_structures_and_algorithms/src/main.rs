pub mod bin_search;
pub mod basic_sort;

// use crate::bin_search::bin_search_test;
use crate::basic_sort::*;
use rand::Rng;

// 乱数配列生成
fn vec_gen(size: usize) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::with_capacity(size); // 指定されたサイズの領域確保
    let mut rng = rand::thread_rng();          // randomの初期化
    
    for _ in 0..size {
        let x: i32 = rng.gen();
        array.push(x%1380);
    }

    array
}

fn main() {
    let n = 10;

    let mut array: Vec<i32> = vec_gen(n);
    bubble_sort(&mut array);
    println!("{:?}", array);

    array = vec_gen(n);
    selection_sort(&mut array);
    println!("{:?}", array);

    array = vec_gen(n);
    array = insertion_sort(&mut array);
    println!("{:?}", array);
}
