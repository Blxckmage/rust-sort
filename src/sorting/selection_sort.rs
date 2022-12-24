pub fn selection_sort(mut list: Vec<i32>) -> Vec<i32> {
    let n = list.len();

    for i in 0..n {
        let mut min_index = i;
        for j in i + 1..n {
            if list[j] < list[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            list.swap(i, min_index);
        }
    }

    list
}
