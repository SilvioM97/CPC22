//Creates the difference array of the given array
pub fn create_diff_array(input_vec: &Vec<i32>) -> Vec<i32> {
    let mut diff_array: Vec<i32> = Vec::with_capacity(input_vec.len());
    diff_array.push(input_vec[0]);
    for i in 1..input_vec.len() {
        diff_array.push(input_vec[i] - input_vec[i - 1]);
    }
    return diff_array;
}

//Reconstruct an array from the corresponding difference array
pub fn reconstruct(diff_array: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(diff_array.len());
    result.push(diff_array[0]);
    for i in 1..diff_array.len() {
        result.push(diff_array[i] + result[i - 1]);
    }
    return result;
}

//Update a range in a difference array (add the value at the starting position and remove it from the end position +1)
pub fn update_range(diff_array: &mut Vec<i32>, i: usize, j: usize, d: i32) {
    diff_array[i] += d;
    if j + 1 < diff_array.len() {
        diff_array[j + 1] -= d;
    }
}
