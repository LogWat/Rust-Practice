pub fn bubble_sort(array: &mut Vec<u8>) {
    let array_size = array.len();

    for _ in 0..array_size {
        for j in 0..array_size-1 {
            if array[j] > array[j+1] {
                let tmp = array[j];
                array[j] = array[j+1];
                array[j+1] = tmp;
            }
        }
    }
}