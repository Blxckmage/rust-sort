pub fn bucket_sort(list: Vec<i32>) -> Vec<i32> {
    // Find the maximum value in the list
    let max = list.iter().max().unwrap();

    // Create a bucket for each possible value in the list
    let mut bucket = vec![0; *max as usize + 1];

    // Count the number of occurrences of each value in the list
    for &element in list.iter() {
        bucket[element as usize] += 1;
    }

    // Iterate through the bucket and add each value to the output list
    // the number of times it occurs in the input list
    let mut output = Vec::new();
    for (i, &count) in bucket.iter().enumerate() {
        for _ in 0..count {
            output.push(i as i32);
        }
    }

    output
}
