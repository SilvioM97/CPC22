use binary_search_tree::BinarySearchTree;
use rand::Rng;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut maximums: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut heap: BinaryHeap<(i32,usize)> = BinaryHeap::new();
    let mut cont: bool = false; //A control variable to check if we took an element from heap or not

    for i in 0..(k-1) { 
        heap.push((nums[i],i)); //Load the heap with first k-1 elements
    }

    for i in (k-1)..n {
        heap.push((nums[i],i)); //Load the new element
        while !cont {
            if ((heap.peek()).unwrap()).1 >= (i-k+1) { //Check if the max is (strictly) in the window
                maximums.push(((heap.peek()).unwrap()).0); //Take the max
                cont = true; //Set cont to true so we go to the next iteration
            }
            else if ((heap.peek()).unwrap()).1 == i-k+1 { //Check if the max is in the first position of the window
                maximums.push(((heap.pop()).unwrap()).0); //Pop the max (if it is the 1st el. of the window we don't want it in the next iteration)
                cont = true; //Set cont to true so we go to the next iteration                        
            }
            else {
                heap.pop(); //Remove the element because it is not in the window, so we start again with a new element
            }
        }
        cont = false; //Reset the control variable
    } 
    maximums
}

 
pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut maximums: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
    
    for i in 0..(k-1) { //Load the BST
        tree.insert(nums[i]);
    }

    for i in (k-1)..n {
        tree.insert(nums[i]); //Insert the new element
        maximums.push(*(tree.max().unwrap())); //Take the max
        tree.remove(&(nums[i-k+1])); //Remove the element leaving the window
    }
    maximums
}


pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut maximums: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut deq: VecDeque<(i32,usize)> = VecDeque::new();
    let mut cont: bool = false; //A control variable to check if we took an element from deque or not

    for i in 0..n {
        while !cont {
            if !deq.is_empty() && (*((deq.front()).unwrap())).1 < (i-k+1) { //If the deque is not empty remove the head element if it's not in the window
                deq.pop_front();
            }
            else {
                cont = true; //If the deque is empty or if the head is in the window, set cont to true and go ahead
            }
        }
        cont = false; //Reset cont

        while !cont {
            if !deq.is_empty() && (*((deq.back()).unwrap())).0 < nums[i] { //Check if the deq is not empty and if the tail elements are smaller than the new element
                deq.pop_back(); //If so, remove them
            }
            else {
                cont = true; //When the deque is empty or the tail element is not smaller than the new one, set cont to true and go ahead
            }
        }
        cont = false; //Reset cont
        deq.push_back((nums[i],i)); //Move the window one position to the right and add the new element

        if i >= (k-1) { //Check if we already loaded the first k elements
            maximums.push((*((deq.front()).unwrap())).0) //If so, take the maximum as the front element
        }
    }
    maximums    
}


pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}