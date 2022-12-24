use crate::sorting::*;
mod sorting;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::time::Instant;

fn main() {
    // LIST TEST NUMBER
    let mut list: Vec<i32> = Vec::new();
    let mut rng = thread_rng();
    let range = Uniform::new_inclusive(0, 15);
    for _ in 0..15 {
        list.push(rng.sample(range));
    }

    println!("Test List: {:?}", list);
    println!("");

    // BUBBLE SORT
    let bub_start = Instant::now();
    let bubble_sorted = bubble_sort(list.clone());
    let bub_elapsed = bub_start.elapsed();

    println!("| BUBBLE SORT |");
    println!("{:?}", bubble_sorted);
    println!("Elapsed time: {:?}", bub_elapsed);
    println!("");

    // INSERTION SORT
    let ins_start = Instant::now();
    let insertion_sorted = insertion_sort(list.clone());
    let ins_elapsed = ins_start.elapsed();

    println!("| INSERTION SORT |");
    println!("{:?}", insertion_sorted);
    println!("Elapsed time: {:?}", ins_elapsed);
    println!("");

    // HEAP SORT
    let hea_start = Instant::now();
    let mut heap_sorted = list.clone();
    heap_sort(&mut heap_sorted);
    let hea_elapsed = hea_start.elapsed();

    println!("| HEAP SORT |");
    println!("{:?}", heap_sorted);
    println!("Elapsed time: {:?}", hea_elapsed);
    println!("");

    // QUICK SORT
    let qui_start = Instant::now();
    let quick_sorted = quick_sort(list.clone());
    let qui_elapsed = qui_start.elapsed();

    println!("| QUICK SORT |");
    println!("{:?}", quick_sorted);
    println!("Elapsed time: {:?}", qui_elapsed);
    println!("");

    // SELECTION SORT
    let sel_start = Instant::now();
    let selection_sorted = selection_sort(list.clone());
    let sel_elapsed = sel_start.elapsed();

    println!("| SELECTIOB SORT |");
    println!("{:?}", selection_sorted);
    println!("Elapsed time: {:?}", sel_elapsed);
    println!("");

    // BUCKET SORT
    let buc_start = Instant::now();
    let bucket_sorted = bucket_sort(list.clone());
    let buc_elapsed = buc_start.elapsed();

    println!("| BUCKET SORT |");
    println!("{:?}", bucket_sorted);
    println!("Elapsed time: {:?}", buc_elapsed);
    println!("");
}
