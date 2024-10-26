#![feature(test)]
#![feature(let_chains)]

extern crate test;

mod csv;
mod visualization;
mod complexity;

use anyhow::Result;
use itertools::Itertools;
use prover::logic::Logic;
use std::env;
use std::str::FromStr;
use anyhow::Context;
use prover::codeloc;
use crate::complexity::calculate_complexity;
use crate::csv::{generate_random_formulas, read_csv};
use crate::visualization::visualize_data;

fn main() -> Result<()>
{
    let args = env::args().collect_vec();
    if args.contains(&String::from("--generate-random-formulas"))
    {
        let max_number_of_operators = usize::from_str(&args[args.len()-2].as_str()).context(codeloc!())?;
        let number_of_formulas_per_group = usize::from_str(&args[args.len()-1].as_str()).context(codeloc!())?;

        generate_random_formulas(max_number_of_operators, number_of_formulas_per_group).context(codeloc!())?;
    }
    else
    {
        let csv_lines = read_csv().context(codeloc!())?;
        let complexity = calculate_complexity(&csv_lines).context(codeloc!())?;
        println!("\n\nComplexity = {}", complexity);

        if !args.contains(&String::from("--headless"))
        {
            visualize_data(&csv_lines, &complexity).context(codeloc!())?;
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::hint::black_box;
    use test::Bencher;
    use prover::logic::LogicFactory;

    #[bench]
    fn generate_csv(bencher : &mut Bencher)
    {
        let logic = env::args().collect_vec().into_iter()
            .filter_map(|arg| LogicFactory::get_logic_by_name(&arg).ok())
            .last().unwrap();

        black_box(csv::generate_csv(&logic)).unwrap();
    }
}
