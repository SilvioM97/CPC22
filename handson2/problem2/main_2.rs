use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod lib;
use lib::{create_diff_array, reconstruct, update_range};

fn main() -> std::io::Result<()> {
    //Read file (for n, m and k)
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file = File::open(file_name).expect("File not found!");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    buf_reader
        .read_line(&mut line)
        .expect("No new lines to read!"); //Read one line
    let mut iter = line.split_whitespace(); //Split the line by spaces
    let mut numbers: Vec<i32> = Vec::new();
    //Convert the elements in iter into integers
    for num in iter {
        numbers.push(num.parse().unwrap());
    }
    let n: usize = numbers[0] as usize; //Size of the input vector
    let m: usize = numbers[1] as usize; //Number of operations
    let k: usize = numbers[2] as usize; //Number of queries

    //Read file (for input vector)
    line = String::new();
    buf_reader
        .read_line(&mut line)
        .expect("No new lines to read!"); //Read one line
    iter = line.split_whitespace(); //Split the line by spaces
    let mut input_vec: Vec<i32> = Vec::with_capacity(n);
    //Convert the elements in iter into integers
    for num in iter {
        input_vec.push(num.parse().unwrap());
    }

    //Read file (for operations)
    let mut operations_vec: Vec<(usize, usize, i32)> = Vec::with_capacity(m); //Array of operations O[1,m]
    for _i in 0..m {
        line = String::new();
        buf_reader
            .read_line(&mut line)
            .expect("No new lines to read!"); //Read one line
        iter = line.split_whitespace(); //Split the line by spaces
        numbers = Vec::with_capacity(4);
        //Convert the elements in iter into integers
        for num in iter {
            numbers.push(num.parse().unwrap());
        }
        operations_vec.push((numbers[0] as usize - 1, numbers[1] as usize - 1, numbers[2]));
        //Fill the operations array
    }

    //Read file (for queries)
    let mut queries_vec: Vec<(usize, usize)> = Vec::with_capacity(k); // Array of queries Q[1,k]
    for _i in 0..k {
        line = String::new();
        buf_reader
            .read_line(&mut line)
            .expect("No new lines to read!"); //Read one line
        iter = line.split_whitespace(); //Split the line by spaces
        numbers = Vec::with_capacity(3);
        //Convert the elements in iter into integers
        for num in iter {
            numbers.push(num.parse().unwrap());
        }
        queries_vec.push((numbers[0] as usize - 1, numbers[1] as usize - 1)); //Fill the queries array
    }

    //Execute operations
    let mut counter_vec: Vec<i32> = vec![0; m]; //Counter (difference) array which says for each i=1,...,m how many times O[i] must be executed
                                                //Updating the difference array
    for i in 0..k {
        update_range(&mut counter_vec, queries_vec[i].0, queries_vec[i].1, 1);
    }
    counter_vec = reconstruct(&counter_vec); //Reconstructing the true counter array

    let mut diff_array = create_diff_array(&input_vec); //Difference array for the input array

    //Updating the diff. array of input array by executing each operation O[i] the number of times told us by counter array
    //This is done multyplying the value d_i of the operation O[i] by its counter counter_vec[i] (0 if O[i] is not executed by any query)
    for i in 0..m {
        update_range(
            &mut diff_array,
            operations_vec[i].0,
            operations_vec[i].1,
            (operations_vec[i].2) * counter_vec[i],
        );
    }
    input_vec = reconstruct(&diff_array); //Reconstructing the updated input array from its updated difference array

    //Print the updated input array
    for i in 0..input_vec.len() {
        print!("{:?} ", input_vec[i]);
    }

    Ok(())
}
