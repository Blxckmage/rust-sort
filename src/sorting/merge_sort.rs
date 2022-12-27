pub fn merge_sort(list: Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {
        return list;
    }

    let mid = list.len() / 2;
    let (left, right) = list.split_at(mid);

    let mut left = merge_sort(left.to_vec());
    let mut right = merge_sort(right.to_vec());

    merge(left.as_mut_slice(), right.as_mut_slice())
}

fn merge(mut left: &mut [i32], mut right: &mut [i32]) -> Vec<i32> {
    let mut result = Vec::new();

    while !left.is_empty() && !right.is_empty() {
        if left[0] < right[0] {
            result.push(left[0]);
            left = &mut left[1..];
        } else {
            result.push(right[0]);
            right = &mut right[1..];
        }
    }

    result.extend_from_slice(left);
    result.extend_from_slice(right);
    result
}
