#![feature(test)]

extern crate test;

mod csv;
mod visualization;
mod complexity;

use crate::csv::read_csv;
use crate::complexity::calculate_complexity;
use crate::visualization::visualize_data;

fn main()
{
    let csv_lines = read_csv().unwrap();
    let complexity = calculate_complexity(&csv_lines);
    visualize_data(&csv_lines, complexity).unwrap();
}

#[cfg(test)]
mod tests
{
    use std::hint::black_box;
    use super::*;
    use test::Bencher;

    #[bench]
    fn generate_csv(bencher : &mut Bencher)
    {
        black_box(csv::generate_csv()).unwrap();
    }
}
