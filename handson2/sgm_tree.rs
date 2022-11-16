use core::cmp::max;

#[derive(Default)]
pub struct SegmentTree<T: Ord + std::fmt::Display + Clone + Copy> {
    pub seg_tree: Vec<T>,
}

impl<T: Ord + std::fmt::Display + Clone + Copy> SegmentTree<T> {

    pub fn new(ciao: T) -> SegmentTree<T> {
        SegmentTree { seg_tree: Vec::new() }
    }

    pub fn build(&mut self, mut input_vec: Vec<T>) {
        let n = input_vec.len();

        self.seg_tree.append(&mut input_vec);
        self.seg_tree.pop();

        if n%2 == 0 {
            self.seg_tree.append(&mut input_vec);
        }
        else {
            self.seg_tree.push(input_vec[n-1]);
            self.seg_tree.append(&mut input_vec);
            self.seg_tree.pop();
        }

        for i in (0..n-1).rev() {
            self.seg_tree[i] = max(self.seg_tree[2*i+1],self.seg_tree[2*i+2]);
        }
    }

    pub fn print_tree(self) {
        let n = self.seg_tree.len();
        for i in 0..n {
            print!("{} ", self.seg_tree[i]);
        }
    }

}
