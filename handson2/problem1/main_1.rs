use lib::SegmentTree;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod lib;

fn main() -> std::io::Result<()> {
    let mut seg_tree = SegmentTree::new();
    let mut n_ones: usize = 0; //Counter for number of Max queries
    let mut results: Vec<i32> = Vec::new(); //Vector containing the outputs

    //Read file (for n and m)
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
    let m: usize = numbers[1] as usize; //Number of queries

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
    seg_tree.build(input_vec); //Build the segment tree

    //Read file (for queries)
    for _i in 0..m {
        line = String::new();
        buf_reader
            .read_line(&mut line)
            .expect("No new lines to read!"); //Read one line
        iter = line.split_whitespace(); //Split the line by spaces
        numbers = Vec::new();
        //Convert the elements in iter into integers
        for num in iter {
            numbers.push(num.parse().unwrap());
        }
        //Update query
        if numbers[0] == 0 {
            seg_tree.update_query(
                (numbers[1] as usize) - 1,
                (numbers[2] as usize) - 1,
                0,
                numbers[3],
            );
        }
        //Max query
        else {
            results.push(seg_tree.max_query(
                (numbers[1] as usize) - 1,
                (numbers[2] as usize) - 1,
                0,
            ));
            println!("{:?} ", results[n_ones]);
            n_ones += 1;
        }
    }
    println!();

    Ok(())
}
