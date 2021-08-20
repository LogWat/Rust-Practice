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