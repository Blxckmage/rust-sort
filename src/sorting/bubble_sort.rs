pub fn bubble_sort(mut list: Vec<i32>) -> Vec<i32> {
    let len = list.len();

    for _ in 0..len {
        for j in 1..len {
            if list[j - 1] > list[j] {
                list.swap(j - 1, j);
            }
        }
    }

    list
}
