pub fn bucket_sort(list: Vec<i32>) -> Vec<i32> {
    let max = list.iter().max().unwrap();

    let mut bucket = vec![0; *max as usize + 1];

    for &element in list.iter() {
        bucket[element as usize] += 1;
    }

    let mut output = Vec::new();
    for (i, &count) in bucket.iter().enumerate() {
        for _ in 0..count {
            output.push(i as i32);
        }
    }

    output
}
