use anyhow::Result;
use prover::formula::Formula;
use prover::logic::propositional_logic::PropositionalLogic;
use prover::logic::Logic;
use prover::problem::Problem;
use rand::random;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use itertools::Itertools;
use prover::problem::catalog::get_demo_problem_catalog;

#[inline(always)]
fn measure_number_of_cpu_instructions<F>(function : F) -> u64 where F : FnOnce() -> ()
{
    let mut count = 0u64;
    count_instructions::count_instructions(function, |_| count += 1).unwrap();
    return count;
}

#[inline(always)]
fn measure_total_number_of_allocated_bytes<F>(function : F) -> u64 where F : FnOnce() -> ()
{
    let data = allocation_counter::measure(function);
    return data.bytes_total;
}

pub fn generate_csv() -> Result<()>
{
    let target_logic_name : Rc<dyn Logic> = Rc::new(PropositionalLogic {});
    // let problems = generate_random_problems(&target_logic_name);

    let problems = get_demo_problem_catalog().unwrap().into_iter()
        .flat_map(|chapter| chapter.problems)
        .map(|problem| problem.to_problem().unwrap())
        .filter(|problem| problem.logic.get_name().is_intuitionistic_logic())
        .collect_vec();

    let data_file_path = Path::new("data.csv");
    fs::remove_file(data_file_path).unwrap_or_default();

    let mut data_file = OpenOptions::new().create_new(true).write(true).open(data_file_path)?;
    writeln!(data_file, "{}", "Problem ID,Logic,Input,Output (CPU),Output (RAM)")?;

    for problem in problems
    {
        let problem_id = problem.id.clone();
        let logic_name = problem.logic.get_name();
        let input_indicator = calculate_input_indicator(&problem);

        // let mut problem_clone = problem.clone();
        // problem_clone.skip_contradiction_check = true;
        // let lambda = move || { problem_clone.prove(); };
        // let instruction_count = measure_number_of_cpu_instructions(lambda);

        let mut problem_clone = problem.clone();
        problem_clone.skip_contradiction_check = true;
        let lambda = move || { problem_clone.prove(); };
        let allocation_count = measure_total_number_of_allocated_bytes(lambda);

        writeln!(data_file, "{},{},{},{}", problem_id, logic_name, input_indicator, allocation_count)?;
    }

    return Ok(());
}

fn calculate_input_indicator(problem : &Problem) -> u64
{
    fn indicator(formula : &Formula) -> u64
    {
        return 1 + match formula
        {
            Formula::Atomic(_, _) => { 0 }
            Formula::Non(p, _) => { indicator(&*p) }
            Formula::And(p, q, _) => { indicator(&*p) + indicator(&*q) }
            Formula::Or(p, q, _) => { indicator(&*p) + indicator(&*q) }
            Formula::Imply(p, q, _) => { indicator(&*p) + indicator(&*q) }
            Formula::BiImply(p, q, _) => { indicator(&*p) + indicator(&*q) }
            Formula::StrictImply(p, q, _) => { indicator(&*p) + indicator(&*q) }
            Formula::Conditional(p, q, _) => { indicator(&*p) + indicator(&*q) }
            Formula::Exists(_, p, _) => { indicator(&*p) }
            Formula::ForAll(_, p, _) => { indicator(&*p) }
            Formula::Equals(_, _, _) => { 0 }
            Formula::DefinitelyExists(_, _) => { 0 }
            Formula::Possible(p, _) => { indicator(&*p) }
            Formula::Necessary(p, _) => { indicator(&*p) }
            Formula::InPast(p, _) => { indicator(&*p) }
            Formula::InFuture(p, _) => { indicator(&*p) }
            Formula::Comment(_) => { 0 }
        }
    }

    let mut input_indicator = 0u64;
    for premise in &problem.premises
    {
        input_indicator += indicator(premise);
    }

    let logic_semantics = problem.logic.get_semantics();
    let non_conclusion = logic_semantics.reductio_ad_absurdum(&problem.conclusion);
    input_indicator += indicator(&non_conclusion);

    return input_indicator;
}

fn generate_random_problems(logic : &Rc<dyn Logic>) -> Vec<Problem>
{
    let mut problems : Vec<Problem> = vec![];
    while problems.is_empty()
    {
        let formula : Formula = random();
        let problem = Problem
        {
            id: formula.to_string(),
            logic: logic.clone(),
            premises: Vec::new(),
            conclusion: formula,
            skip_contradiction_check: false,
        };

        let indicator = calculate_input_indicator(&problem);
        if indicator >= 25 && indicator <= 30
        {
            problems.push(problem);
        }
    }

    return problems;
}

pub struct ParsedCSVLine
{
    pub problem_id : String,
    pub logic : String,
    pub input : u64,
    pub output : u64,
}

pub fn read_csv() -> Result<Vec<ParsedCSVLine>>
{
    let mut csv_lines: Vec<ParsedCSVLine> = vec![];
    let data_file = File::open("data.csv")?;
    let data_file_reader = BufReader::new(data_file);

    for (position, line_result) in data_file_reader.lines().enumerate()
    {
        if position>0 && line_result.is_ok()
        {
            let line = line_result.unwrap();
            let tokens = line.split(",").collect_vec();
            csv_lines.push(ParsedCSVLine
            {
                problem_id: tokens[0].to_string(),
                logic: tokens[1].to_string(),
                input: u64::from_str(tokens[2]).unwrap(),
                output: u64::from_str(tokens[3]).unwrap(),
            })
        }
    }

    return Ok(csv_lines);
}
