use std::fmt::{Display, Formatter};
use anyhow::{Context, Result};
use prover::codeloc;
use crate::csv::ParsedCSVLine;

pub enum Complexity
{
    Logarithmic(f64),
    Linear(f64),
    LogLinear(f64),
    Quadratic(f64),
    Exponential(f64),
    Factorial(f64),
}

impl Complexity
{
    pub fn logarithmic(max_input : u64, max_output : u64) -> Complexity
    {
        return Complexity::from(max_input, max_output, |k| Complexity::Logarithmic(k));
    }

    pub fn linear(max_input : u64, max_output : u64) -> Complexity
    {
        return Complexity::from(max_input, max_output, |k| Complexity::Linear(k));
    }

    pub fn log_linear(max_input : u64, max_output : u64) -> Complexity
    {
        return Complexity::from(max_input, max_output, |k| Complexity::LogLinear(k));
    }

    pub fn quadratic(max_input : u64, max_output : u64) -> Complexity
    {
        return Complexity::from(max_input, max_output, |k| Complexity::Quadratic(k));
    }

    pub fn exponential(max_input : u64, max_output : u64) -> Complexity
    {
        return Complexity::from(max_input, max_output, |k| Complexity::Exponential(k));
    }

    pub fn factorial(max_input : u64, max_output : u64) -> Complexity
    {
        return Complexity::from(max_input, max_output, |k| Complexity::Factorial(k));
    }

    pub fn from(max_input : u64, max_output : u64, complexity_factory : fn(f64) -> Complexity) -> Complexity
    {
        let complexity = complexity_factory(1f64);
        let k = (max_output as f64) / complexity.plot(max_input as f64);
        return complexity_factory(k);
    }

    pub fn plot(&self, n : f64) -> f64
    {
        fn factorial(n : f64) -> f64
        {
            return match n as u64
            {
                0 => 1.0, 1 => 1.0, 2 => 2.0, 3 => 6.0, 4 => 24.0, 5 => 120.0, 6 => 720.0, 7 => 5040.0,
                8 => 40320.0, 9 => 362880.0, 10 => 3628800.0, 11 => 39916800.0, _ => 479001600.0,
            };
        }

        return match self
        {
            Complexity::Logarithmic(k) => { n.log10() * (*k) }
            Complexity::Linear(k) => { n * (*k) }
            Complexity::LogLinear(k) => { n * n.log10() * (*k) }
            Complexity::Quadratic(k) => { n * n * (*k) }
            Complexity::Exponential(k) => { 2f64.powf(n) * (*k) }
            Complexity::Factorial(k) => { factorial(n) * (*k) }
        }
    }

    pub fn to_onotation_string(&self) -> String
    {
        return String::from(match self
        {
            Complexity::Logarithmic(_) => { "O(log(n))" }
            Complexity::Linear(_) => { "O(n)" }
            Complexity::LogLinear(_) => { "O(n*log(n))" }
            Complexity::Quadratic(_) => { "O(n²)" }
            Complexity::Exponential(_) => { "O(2ⁿ)" }
            Complexity::Factorial(_) => { "O(n!)" }
        })
    }

    pub fn to_friendly_string(&self) -> String
    {
        return String::from(match self
        {
            Complexity::Logarithmic(_) => { "Logarithmic" }
            Complexity::Linear(_) => { "Linear" }
            Complexity::LogLinear(_) => { "LogLinear" }
            Complexity::Quadratic(_) => { "Quadratic" }
            Complexity::Exponential(_) => { "Exponential" }
            Complexity::Factorial(_) => { "Factorial" }
        })
    }

    pub fn to_scaled_function_string(&self) -> String
    {
        return match self
        {
            Complexity::Logarithmic(k) => { format!("f1(n) = log(n) * {:.4}", k) }
            Complexity::Linear(k) => { format!("f2(n) = n * {:.4}", k) }
            Complexity::LogLinear(k) => { format!("f3(n) = n*log(n) * {:.4}", k) }
            Complexity::Quadratic(k) => { format!("f4(n) = n² * {:.4}", k) }
            Complexity::Exponential(k) => { format!("f5(n) = 2ⁿ * {:.4}", k) }
            Complexity::Factorial(k) => { format!("f6(n) = n! * {:.4}", k) }
        }
    }
}

impl Display for Complexity
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", self.to_onotation_string());
    }
}

pub fn calculate_complexity(csv_lines : &Vec<ParsedCSVLine>) -> Result<Complexity>
{
    let max_input = csv_lines.iter().map(|line| line.input).max().context(codeloc!())?;
    let max_output = csv_lines.iter().map(|line| line.output).max().context(codeloc!())?;

    let mut complexities =
    [
        (f64::MAX, Complexity::logarithmic(max_input, max_output)),
        (f64::MAX, Complexity::linear(max_input, max_output)),
        (f64::MAX, Complexity::log_linear(max_input, max_output)),
        (f64::MAX, Complexity::quadratic(max_input, max_output)),
        (f64::MAX, Complexity::exponential(max_input, max_output)),
        (f64::MAX, Complexity::factorial(max_input, max_output)),
    ];

    for complexity_index in 0..complexities.len()
    {
        let (_, complexity) = &complexities[complexity_index];

        let mut average_output = 0f64;
        for csv_line in csv_lines {
            average_output += csv_line.output as f64;
        }
        average_output /= csv_lines.len() as f64;

        let mut residual_sum_of_squares = 0f64;
        for csv_line in csv_lines
        {
            let diff = (csv_line.output as f64) - complexity.plot(csv_line.input as f64);
            residual_sum_of_squares += diff * diff;
        }

        let mut total_sum_of_squares = 0f64;
        for csv_line in csv_lines
        {
            let diff = (csv_line.output as f64) - average_output;
            total_sum_of_squares += diff * diff;
        }

        let r_squared = residual_sum_of_squares / total_sum_of_squares;
        complexities[complexity_index].0 = r_squared;
    }

    println!("\n\nComplexity,,Scaled function,R²");
    for (r_squared, complexity) in &complexities
    {
        println!("{},{},{},{:.4}",
            complexity.to_friendly_string(), complexity.to_onotation_string(),
            complexity.to_scaled_function_string(), r_squared);
    }

    let (_, complexity) = complexities.into_iter()
        .filter(|(r_squared, _)| !r_squared.is_nan())
        .min_by(|(r_squared1, _), (r_squared2, _)| r_squared1.total_cmp(r_squared2))
        .expect("Cannot determine complexity!");

    return Ok(complexity);
}
