pub fn insertion_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 1..list.len() {
        let mut j = i;
        let temp = list[i];

        while j > 0 && list[j - 1] > temp {
            list[j] = list[j - 1];
            j -= 1;
        }

        list[j] = temp;
    }

    list
}
