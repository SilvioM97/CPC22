use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod lib;
use lib::patriotics_selections;

fn main() -> std::io::Result<()> {
    //Read file (for n)
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file = File::open(file_name).expect("File not found!");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    buf_reader
        .read_line(&mut line)
        .expect("No new lines to read!"); //Read one line
    let iter = line.split_whitespace(); //Split the line by spaces
    let mut inputs: usize = 0;
    //Convert the elements in iter into integers
    for num in iter {
        inputs = num.parse().unwrap();
    }
    let n: usize = inputs as usize; //Size of the input vector

    //Read file (for colors)
    line = String::new();
    buf_reader
        .read_line(&mut line)
        .expect("No new lines to read!"); //Read one line
    let mut inputs: Vec<i32> = Vec::with_capacity(n);
    //Convert the colors into integers (R=0, W=1, G=2, X=-1)
    for color in line.chars() {
        if color == 'R' {
            inputs.push(0);
        } else if color == 'W' {
            inputs.push(1);
        } else if color == 'G' {
            inputs.push(2);
        } else if color == 'X' {
            inputs.push(-1);
        }
    }

    println!("{:?}", patriotics_selections(inputs));

    Ok(())
}
