pub fn heap_sort(list: &mut Vec<i32>) {
    // Build a max heap
    let mut n = list.len();
    for i in (0..n / 2).rev() {
        heapify(list, n, i);
    }

    // Extract elements from the heap one by one
    for i in (1..n).rev() {
        list.swap(0, i);
        n -= 1;
        heapify(list, n, 0);
    }
}

fn heapify(list: &mut Vec<i32>, n: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < n && list[left] > list[largest] {
        largest = left;
    }

    if right < n && list[right] > list[largest] {
        largest = right;
    }

    if largest != root {
        list.swap(root, largest);
        heapify(list, n, largest);
    }
}
