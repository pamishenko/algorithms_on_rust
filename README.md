# algorithms_on_rust
## Description
In this project, I will be implementing educational algorithms in Rust language based on the book "Grooking Algorithms" by Aditya Bhargava. The book is an excellent resource for learning algorithms and data structures at various levels of knowledge.

The goal of this project is to learn how to apply the studied algorithms and data structures in practice, as well as gain experience working with the Rust programming language.

[Algorithms](#binary-search) \
    1. [Binary Search](#binary-search) \
    2. [Selection sort](#selection-sort) 

## Algorithm speed
>Algorithm speed, also known as time complexity, is the amount of time it takes for an algorithm to execute and is measured by the total number of operations performed by the algorithm. It is typically measured relative to the size of the input data to the algorithm, so algorithm speed can be expressed as a function of the input data size. For example, if a sorting algorithm performs O(nlog(n)) operations to sort an array of length n, its speed would be O(nlog(n)).

## Binary search
>Binary Search is an algorithm for searching for an element in an ordered array. It works by comparing the element you want to find with the element in the middle of the array. If the element is equal to the searched element, the search is completed. Otherwise, the algorithm recursively continues the search in the left or right half of the array, depending on which half the element may be in.

implementation:

```
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