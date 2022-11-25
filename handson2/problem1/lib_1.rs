use core::cmp::max;
use core::cmp::min;

//Class for segment tree's nodes
//4 fields:
//-key: the value in the node
//-lazy_key: the value in the corresponding node in the lazy tree
//-left_interval: left extreme of the interval covered by the node
//-right_interval: right extreme of the interval covered by the node
#[derive(Copy, Clone)]
pub struct Node {
    key: i32,
    lazy_key: i32,
    left_interval: usize,
    right_interval: usize,
}

//Class for segment tree
//2 fields:
//-seg_tree: vector containing the nodes of the tree s.t. the children of a node in position K are in position 2K+1 (left) and 2K+2 (right)
//-n: size of the original vector (number of leaves of the tree)
pub struct SegmentTree {
    seg_tree: Vec<Node>,
    n: usize,
}

impl Node {
    pub fn new(k: i32, lk: i32, li: usize, ri: usize) -> Node {
        return Node {
            key: k,
            lazy_key: lk,
            left_interval: li,
            right_interval: ri,
        };
    }
}

impl SegmentTree {
    pub fn new() -> SegmentTree {
        SegmentTree {
            seg_tree: Vec::new(),
            n: 0,
        }
    }

    pub fn build(&mut self, mut input_vec: Vec<i32>) {
        let n = input_vec.len().next_power_of_two();
        self.n = n;
        let mut end = vec![-1; n - input_vec.len()];
        input_vec.append(&mut end); //Append some -1 values to the vector s.t. the new length is a power of two
                                    //Build the vector
        self.seg_tree = build_vec(
            &mut input_vec
                .into_iter()
                .enumerate()
                .map(|(i, x)| Node::new(x, -1, i, i))
                .collect(),
        );
    }

    //Propagate the value in the node k in lazy tree to its children and reset the lazy value to -1
    pub fn propagate(&mut self, k: usize, t: i32) {
        //Propagate to left child
        if self.seg_tree[2 * k + 1].lazy_key == -1 {
            self.seg_tree[2 * k + 1].lazy_key = t;
        } else {
            self.seg_tree[2 * k + 1].lazy_key = min(self.seg_tree[2 * k + 1].lazy_key, t);
        }
        //Propagate to right child
        if self.seg_tree[2 * k + 2].lazy_key == -1 {
            self.seg_tree[2 * k + 2].lazy_key = t;
        } else {
            self.seg_tree[2 * k + 2].lazy_key = min(self.seg_tree[2 * k + 2].lazy_key, t);
        }
        self.seg_tree[k].lazy_key = -1; //Reset value
    }

    pub fn update_query(&mut self, i: usize, j: usize, k: usize, t: i32) {
        //Check for pending update
        if self.seg_tree[k].lazy_key != -1 {
            self.seg_tree[k].key = min(self.seg_tree[k].key, self.seg_tree[k].lazy_key);
            if k < self.n - 1 {
                self.propagate(k, self.seg_tree[k].lazy_key);
            }
        }
        //Check if the range is empty
        if i > j {
            return;
        }
        //If there is a total overlap, update and propagate to children
        if self.seg_tree[k].left_interval == i && self.seg_tree[k].right_interval == j {
            self.seg_tree[k].key = min(self.seg_tree[k].key, t);
            if k < self.n - 1 {
                //Propagate
                self.propagate(k, t);
            }
            return;
        }
        //If there is a partial overlap, update recursively left and right and take the max of the two children
        if k < self.n - 1 {
            self.update_query(
                i,
                min(self.seg_tree[2 * k + 1].right_interval, j),
                2 * k + 1,
                t,
            );
            self.update_query(
                max(self.seg_tree[2 * k + 2].left_interval, i),
                j,
                2 * k + 2,
                t,
            );
            self.seg_tree[k].key = max(self.seg_tree[2 * k + 1].key, self.seg_tree[2 * k + 2].key);
        }
    }

    pub fn max_query(&mut self, i: usize, j: usize, k: usize) -> i32 {
        //Check for pending update
        if self.seg_tree[k].lazy_key != -1 {
            self.seg_tree[k].key = min(self.seg_tree[k].key, self.seg_tree[k].lazy_key);
            if k < self.n - 1 {
                self.propagate(k, self.seg_tree[k].lazy_key);
            }
        }
        //Check if the range is empty
        if i > j {
            return -1;
        }
        //If there is a total overlap, return the current node value
        if self.seg_tree[k].left_interval == i && self.seg_tree[k].right_interval == j {
            return self.seg_tree[k].key;
        }
        //If there is a partial overlap, return the maximum between the recursive calls to children
        else {
            return max(
                self.max_query(
                    i,
                    min(self.seg_tree[2 * k + 1].right_interval, j),
                    2 * k + 1,
                ),
                self.max_query(max(self.seg_tree[2 * k + 2].left_interval, i), j, 2 * k + 2),
            );
        }
    }

    //Print the tree
    /*pub fn print_tree(&self) {
        let n = self.seg_tree.len();
        for i in 0..n {
            if self.seg_tree[i].left_interval == 0 {
                println!("");
                for _i in 0..self.seg_tree[i].right_interval {
                    print!("   ");
                }
            }
            print!("([{:?}, ", self.seg_tree[i].left_interval);
            print!("{:?}] - ", self.seg_tree[i].right_interval);
            print!("{:?} - ", self.seg_tree[i].key);
            print!("{:?}) ", self.seg_tree[i].lazy_key);
        }
        println!("");
    }*/
}

//Given an array A, recursively does the pair_maxes of the
pub fn build_vec(input_vec: &mut Vec<Node>) -> Vec<Node> {
    let n = input_vec.len();

    if n < 2 {
        return input_vec.to_vec();
    } else {
        let mut prec = build_vec(&mut pair_maxes(input_vec.to_vec()));
        prec.append(input_vec);
        return prec;
    }
}

//Given an array A of even size, returns the array given by the max of the pairs A[i,i+1] for i=0,2,4,...,n-2
pub fn pair_maxes(input_vec: Vec<Node>) -> Vec<Node> {
    let mut result = Vec::new();

    if input_vec.len() == 1 {
        return input_vec;
    } else {
        for i in (0..input_vec.len()).step_by(2) {
            result.push(Node {
                key: max(input_vec[i].key, input_vec[i + 1].key),
                lazy_key: -1,
                left_interval: input_vec[i].left_interval,
                right_interval: input_vec[i + 1].right_interval,
            });
        }
    }
    return result;
}
