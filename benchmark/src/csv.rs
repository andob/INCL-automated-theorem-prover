use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{BufRead, BufReader, Write};
use std::rc::Rc;
use std::str::FromStr;
use anyhow::{Context, Result};
use itertools::Itertools;
use prover::codeloc;
use prover::formula::Formula;
use prover::formula::to_string::FormulaFormatOptions;
use prover::logic::Logic;
use prover::parser::algorithm::LogicalExpressionParser;
use prover::problem::Problem;

const RANDOM_FORMULAS_FILE_NAME : &str = "random_formulas.txt";
const DATA_CSV_FILE_NAME : &str = "data.csv";

#[inline(always)]
fn measure_number_of_cpu_instructions<F>(function : F) -> Result<u64> where F : FnOnce() -> ()
{
    let mut count = 0u64;
    count_instructions::count_instructions(function, |_| count += 1).context(codeloc!())?;
    return Ok(count);
}

#[inline(always)]
fn measure_total_number_of_allocated_bytes<F>(function : F) -> u64 where F : FnOnce() -> ()
{
    let data = allocation_counter::measure(function);
    return data.bytes_total;
}

pub fn generate_random_formulas(max_number_of_operators : usize, number_of_formulas_per_group : usize) -> Result<()>
{
    let random_formulas_file_path = Path::new(RANDOM_FORMULAS_FILE_NAME);
    fs::remove_file(random_formulas_file_path).unwrap_or_default();

    let mut random_formulas_file = OpenOptions::new().create_new(true)
        .write(true).open(random_formulas_file_path).context(codeloc!())?;

    let mut formula_format_options = FormulaFormatOptions::default();
    formula_format_options.should_show_possible_worlds = false;

    for number_of_operators in 1..max_number_of_operators
    {
        for _ in 0..number_of_formulas_per_group
        {
            let formula = Formula::random(number_of_operators);
            let formula_as_string = formula.to_string_with_options(&formula_format_options);
            writeln!(random_formulas_file, "{}", formula_as_string).context(codeloc!())?;
        }
    }

    return Ok(());
}

pub fn read_random_problems(logic : &Rc<dyn Logic>) -> Result<Vec<Problem>>
{
    let mut problems : Vec<Problem> = vec![];

    let random_formulas_file = File::open(RANDOM_FORMULAS_FILE_NAME).context(codeloc!())?;
    let random_formulas_file_reader = BufReader::new(random_formulas_file);

    let mut formula_format_options = FormulaFormatOptions::default();
    formula_format_options.should_show_possible_worlds = false;

    for line_result in random_formulas_file_reader.lines()
    {
        if let Ok(line) = line_result
        {
            let formula = LogicalExpressionParser::parse(&logic, &line).context(codeloc!())?;
            let problem_id = formula.to_string_with_options(&formula_format_options);
            let problem = Problem { id:problem_id, logic:logic.clone(), premises:vec![], conclusion:formula };
            problems.push(problem);
        }
    }

    return Ok(problems);
}

pub fn generate_csv(logic : &Rc<dyn Logic>) -> Result<()>
{
    let program_args = env::args().collect_vec();

    let problems = read_random_problems(logic).context(codeloc!())?;

    let data_file_path = Path::new(DATA_CSV_FILE_NAME);
    fs::remove_file(data_file_path).unwrap_or_default();

    let mut data_file = OpenOptions::new().create_new(true)
        .write(true).open(data_file_path).context(codeloc!())?;

    writeln!(data_file, "{}", "Problem ID,Logic,Input,Output").context(codeloc!())?;

    for problem in problems.iter()
    {
        let problem_id = problem.id.clone();
        let logic_name = problem.logic.get_name();
        let input_indicator = calculate_input_indicator(&problem);

        if program_args.contains(&String::from("--cpu"))
        {
            let problem = problem.clone();
            let lambda = move || { problem.prove(); };
            let instruction_count = measure_number_of_cpu_instructions(lambda).context(codeloc!())?;

            writeln!(data_file, "{},{},{},{}", problem_id, logic_name, input_indicator, instruction_count).context(codeloc!())?;
        }
        else if program_args.contains(&String::from("--ram"))
        {
            let problem = problem.clone();
            let lambda = move || { problem.prove(); };
            let allocated_bytes_count = measure_total_number_of_allocated_bytes(lambda);

            writeln!(data_file, "{},{},{},{}", problem_id, logic_name, input_indicator, allocated_bytes_count).context(codeloc!())?;
        }
        else
        {
            let proof_tree = problem.clone().prove();
            let proof_tree_size = proof_tree.get_total_number_of_nodes();

            writeln!(data_file, "{},{},{},{}", problem_id, logic_name, input_indicator, proof_tree_size).context(codeloc!())?;
        }
    }

    return Ok(());
}

fn calculate_input_indicator(problem : &Problem) -> usize
{
    let mut input_indicator = 0;
    for premise in &problem.premises
    {
        input_indicator += premise.count_number_of_operators();
    }

    let logic_semantics = problem.logic.get_semantics();
    let non_conclusion = logic_semantics.reductio_ad_absurdum(&problem.conclusion);
    input_indicator += non_conclusion.count_number_of_operators();

    return input_indicator;
}

pub struct ParsedCSVLine { pub problem_id : String, pub logic : String, pub input : u64, pub output : u64 }

pub fn read_csv() -> Result<Vec<ParsedCSVLine>>
{
    let mut csv_lines: Vec<ParsedCSVLine> = vec![];
    let data_file = File::open("data.csv").context(codeloc!())?;
    let data_file_reader = BufReader::new(data_file);

    for (position, line_result) in data_file_reader.lines().enumerate()
    {
        if let Ok(line) = line_result && position > 0
        {
            let tokens = line.split(",").collect_vec();
            csv_lines.push(ParsedCSVLine
            {
                problem_id: tokens[0].to_string(),
                logic: tokens[1].to_string(),
                input: u64::from_str(tokens[2]).context(codeloc!())?,
                output: u64::from_str(tokens[3]).context(codeloc!())?,
            })
        }
    }

    return Ok(csv_lines);
}
