use crate::sorting::*;
mod sorting;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::time::Instant;

fn main() {
    // LIST TEST NUMBER
    let mut list: Vec<i32> = Vec::new();
    let mut rng = thread_rng();
    let range = Uniform::new_inclusive(0, 20);
    for _ in 0..=20 {
        list.push(rng.sample(range));
    }

    println!("Test List: {:?}", list);
    println!("");

    test_bubble_sort(&list);
    test_insertion_sort(&list);
    test_heap_sort(&list);
    test_quick_sort(&list);
    test_selection_sort(&list);
    test_bucket_sort(&list);
    test_merge_sort(&list);
}

fn test_bubble_sort(list: &Vec<i32>) {
    let bub_start = Instant::now();
    let bubble_sorted = bubble_sort(list.clone());
    let bub_elapsed = bub_start.elapsed();

    println!("| BUBBLE SORT |");
    println!("{:?}", bubble_sorted);
    println!("Elapsed time: {:?}", bub_elapsed);
    println!("");
}

fn test_insertion_sort(list: &Vec<i32>) {
    let ins_start = Instant::now();
    let insertion_sorted = insertion_sort(list.clone());
    let ins_elapsed = ins_start.elapsed();

    println!("| INSERTION SORT |");
    println!("{:?}", insertion_sorted);
    println!("Elapsed time: {:?}", ins_elapsed);
    println!("");
}

fn test_heap_sort(list: &Vec<i32>) {
    let hea_start = Instant::now();
    let mut heap_sorted = list.clone();
    heap_sort(&mut heap_sorted);
    let hea_elapsed = hea_start.elapsed();

    println!("| HEAP SORT |");
    println!("{:?}", heap_sorted);
    println!("Elapsed time: {:?}", hea_elapsed);
    println!("");
}

fn test_quick_sort(list: &Vec<i32>) {
    let qui_start = Instant::now();
    let quick_sorted = quick_sort(list.clone());
    let qui_elapsed = qui_start.elapsed();

    println!("| QUICK SORT |");
    println!("{:?}", quick_sorted);
    println!("Elapsed time: {:?}", qui_elapsed);
    println!("");
}

fn test_selection_sort(list: &Vec<i32>) {
    let sel_start = Instant::now();
    let selection_sorted = selection_sort(list.clone());
    let sel_elapsed = sel_start.elapsed();

    println!("| SELECTION SORT |");
    println!("{:?}", selection_sorted);
    println!("Elapsed time: {:?}", sel_elapsed);
    println!("");
}

fn test_bucket_sort(list: &Vec<i32>) {
    let buc_start = Instant::now();
    let bucket_sorted = bucket_sort(list.clone());
    let buc_elapsed = buc_start.elapsed();

    println!("| BUCKET SORT |");
    println!("{:?}", bucket_sorted);
    println!("Elapsed time: {:?}", buc_elapsed);
    println!("");
}

fn test_merge_sort(list: &Vec<i32>) {
    let mer_start = Instant::now();
    let merge_sorted = merge_sort(list.clone());
    let mer_elapsed = mer_start.elapsed();

    println!("| MERGE SORT |");
    println!("{:?}", merge_sorted);
    println!("Elapsed time: {:?}", mer_elapsed);
    println!("");
}
