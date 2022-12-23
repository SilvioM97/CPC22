use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod lib;
use lib::max_attractions;

fn main() -> std::io::Result<()> {
    //Read file (for n and D)
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
    let n: usize = numbers[0] as usize; //Number of cities n
    let days: usize = numbers[1] as usize; //Number of days D

    let mut cities: Vec<Vec<i32>> = vec![vec![0; days + 1]; n]; //Matrix storing the (cumulative) #attractions in each city for each day

    //Read file (for cities)
    for i in 0..n {
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
        //Update values and weights
        for k in 1..days + 1 {
            cities[i][k] = cities[i][k - 1] + numbers[k - 1]; //#attractions if we stay k days in the i-th city
        }
    }

    println!("{:? }", max_attractions(&cities, days as i32));

    Ok(())
}
