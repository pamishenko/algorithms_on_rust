use std::time::Instant;
use rand::{Rng, thread_rng};

fn main() {
    let mut arr1 = gen_random_vector();
    let mut arr2 = gen_random_vector();

    check_fun(selection_sort, &mut arr1);
    check_fun(selection_sort, &mut arr2);
}

fn selection_sort(vec: &mut Vec<i32>) {
    let mut new_vec: Vec<i32> = vec![];
    for _i in 0..vec.len(){
        let smallest_index = find_smallest(vec);
        let smallest_item = vec[smallest_index];
        new_vec.push(smallest_item);
        vec.remove(smallest_index);
    }
    *vec = new_vec;
}

fn find_smallest(vec: &Vec<i32>) -> usize  {
    let mut smallest = vec[0];
    let mut smallest_id: usize = 0;
    for i in 1..vec.len() {
        if vec[i] < smallest {
            smallest = vec[i];
            smallest_id = smallest_id + 1;
        }
    }
    smallest_id
}

fn check_fun<F>(f: F, arr: &mut Vec<i32>)
    where
        F: Fn(&mut Vec<i32>),
{
    println!("created vector -> {:?}", arr);
    let start = Instant::now();
    f(arr);
    let elapsed = start.elapsed();

    println!("Time elapsed: {:?}", elapsed);
    println!("sorted vector -> {:?}", arr);
}

fn gen_random_vector() -> Vec<i32> {
    let mut rng = thread_rng();
    let len = rng.gen_range(1..=1000);

    let mut arr = Vec::with_capacity(len);
    for _ in 0..len {
        arr.push(rng.gen_range(1..=1000));
    }
    arr
}