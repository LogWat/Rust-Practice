// 基本交換法
pub fn bubble_sort(array: &mut Vec<i32>) {

    let array_size = array.len();

    for _ in 0..array_size {
        for j in 0..array_size-1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
            }
        }
    }
}

// 基本選択法
pub fn selection_sort(array: &mut Vec<i32>) {

    let array_size = array.len();

    for i in 0..array_size {
        let mut min: i32 = 0;
        let mut min_index = 0;
        for j in i..array_size {
            if j == i || min > array[j] {
                min = array[j];
                min_index = j;
            }
        }
        if min_index != i {
            array.swap(i, min_index);
        }
    }
}

// 基本挿入法
pub fn insertion_sort(array: &mut Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::with_capacity(array.len());
    let size = array.len();
    for _ in 0..size {
        let mut min_index = 0;
        for j in 0..array.len() {
            if array[min_index] > array[j] {
                min_index = j;
            }
        }
        ans.push(array[min_index]);
        array.swap_remove(min_index); // 削除した場所へ配列末尾を挿入することで計算量を減らしている
    }
    ans
}