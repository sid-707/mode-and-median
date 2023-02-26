/*
Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

Generate list randomly
*/

mod generators;
mod stats;

use std::env;
use stats::{ median, mode };

fn main() {
    let args: Vec<String> = env::args().collect();

    let range = args[1].parse::<i32>().unwrap();

    let list = generators::generate_sorted_list(range);

    println!("sorted list: {:?}", list);
    println!();

    let median = median::get_median(&list);

    println!("median: {:?}", median);

    let mode = mode::get_mode(&list);

    println!("mode: {:?}", mode);
}
