pub fn quick_sort(list: Vec<i32>) -> Vec<i32> {
    let mut list = list;
    let n = list.len();
    if n == 0 {
        return list;
    }
    if n > usize::MAX - 1 {
        return list;
    }
    quick_sort_helper(&mut list, 0, n - 1);
    list
}

fn quick_sort_helper(list: &mut [i32], left: usize, right: usize) {
    if left > usize::MAX - 1 || right > usize::MAX - 1 || left >= right {
        return;
    }
    let diff = right.checked_sub(left).unwrap_or(usize::MAX - 1);
    if diff > usize::MAX - 1 {
        return;
    }
    let pivot_index = partition(list, left, right);
    if pivot_index > usize::MAX - 1 || pivot_index == 0 {
        return;
    }
    quick_sort_helper(list, left, pivot_index - 1);
    quick_sort_helper(list, pivot_index + 1, right);
}

fn partition(list: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = list[right];
    let mut i = left;
    for j in left..right {
        if list[j] < pivot {
            list.swap(i, j);
            i += 1;
        }
    }
    list.swap(i, right);
    i
}
