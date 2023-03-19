# algorithms_on_rust
## Description
In this project, I will be implementing educational algorithms in Rust language based on the book "Grooking Algorithms" by Aditya Bhargava. The book is an excellent resource for learning algorithms and data structures at various levels of knowledge.

The goal of this project is to learn how to apply the studied algorithms and data structures in practice, as well as gain experience working with the Rust programming language.

[Algorithms](#binary-search) \
    1. [Binary Search](#binary-search) \
    2. [Selection sort](#selection-sort) \
    3. [Recursion](#recursion) 

## Algorithm speed
>Algorithm speed, also known as time complexity, is the amount of time it takes for an algorithm to execute and is measured by the total number of operations performed by the algorithm. It is typically measured relative to the size of the input data to the algorithm, so algorithm speed can be expressed as a function of the input data size. For example, if a sorting algorithm performs O(nlog(n)) operations to sort an array of length n, its speed would be O(nlog(n)).

## Binary search
>Binary Search is an algorithm for searching for an element in an ordered array. It works by comparing the element you want to find with the element in the middle of the array. If the element is equal to the searched element, the search is completed. Otherwise, the algorithm recursively continues the search in the left or right half of the array, depending on which half the element may be in.

implementation:

```rust
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
```

## Selection sort
> Selection sort is a sorting algorithm that sequentially iterates through an array and at each iteration finds the minimum element, then swaps it with the first element of the array. Then the process is repeated for the subarray starting from the second element, and so on until the end of the array. This algorithm has a quadratic complexity of O(n^2) in the worst and average cases, but unlike bubble sort, it has a linear complexity of O(n) in the best case when the array is already sorted.

implementation:

```rust
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
```
Function `selection_sort` sorts a mutable reference to a vector of integers using the selection sort algorithm. It creates an empty vector called `new_vec` to hold the sorted integers. It then iterates over the input vector using a for loop, finding the index of the smallest integer using the `find_smallest` function and storing the integer at that index in a variable called `smallest_item`. The integer is then pushed onto `new_vec`, and removed from the original vector using the remove method. Finally, vec is assigned to `new_vec`, so that the sorted integers replace the original unsorted integers. \
Function `find_smallest` takes an immutable reference to a vector of integers, iterates over the vector with a for loop to find the index of the smallest integer, and returns that index.

## Recursion
>Recursion is the process where a function calls itself during its execution. In this way, the function can solve a problem by breaking it down into simpler subproblems that are solved by calling the same function with arguments that differ from the original ones. Recursion can be used in programming to solve problems that can be broken down into smaller subproblems, or to traverse and process data structures such as trees or lists. However, using recursion can lead to memory problems if the function calls themselves become too deep or frequent.

implementation:

```rust
struct Box {
    offspring: Option<Vec<Box>>,
    key: bool,
}

fn find_key(vec_boxes:  &Vec<Box>) {
    for v in vec_boxes.iter(){
        if v.key == true {
            println!("Found!!");
        } else if let Some(offspring) = &v.offspring{
            find_key(offspring);
        };
    }
}
```

This example defines a struct named Box, which has two fields: offspring and key. offspring is an optional vector that contains elements of type Box. key is a Boolean value that indicates whether the current object has a key value or not. \
The function find_key takes a reference to a vector vec_boxes of type Vec<Box>. The function searches for an object in this vector whose key field value is true. If such an object is found, the function prints "Found!!" and stops searching. If the offspring field of the current object contains a value, the function calls itself with that value as an argument to search for the key value in that subset. If offspring is not defined, the function simply continues iterating through the objects in the vec_boxes vector.

