use std::time::Instant;
use rand::{Rng, thread_rng};

fn main() {
    let mut arr1 = gen_random_array();
    let mut  arr2 = gen_random_array();
    let mut rng = thread_rng();
    let to_find = rng.gen_range(1..=1000);
    arr1.sort();
    arr2.sort();
    check_fun(binary_search, arr1, to_find);
    check_fun(binary_search, arr2, to_find);
}

fn check_fun<F>(f: F, arr: Vec<i32>, item: i32)
    where
        F: Fn(Vec<i32>, i32) -> i32,
{
    let start = Instant::now();
    let result = f(arr, item);
    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("Time elapsed: {:?}", elapsed);
}


fn gen_random_array() -> Vec<i32> {
    let mut rng = thread_rng();
    let len = rng.gen_range(1..=1000);

    let mut arr = Vec::with_capacity(len);
    for _ in 0..len {
        arr.push(rng.gen_range(1..=1000));
    }
    arr
}

fn binary_search(sorted_array: Vec<i32>, item: i32) -> i32 {
    let mut low = 0;
    let mut high = sorted_array.len() - 1;

    while low <= high {
        let mid: usize = (low + high) / 2;
        if sorted_array[mid] == item {
            return mid as i32;
        }
        if sorted_array[mid] > item {
            high = (mid - 1) as usize;
        }
        else {
            low = (mid + 1) as usize;
        }
    }
    -1
}