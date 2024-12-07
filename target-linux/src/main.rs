use std::fs::File;
use std::{env, fs, io, thread};
use std::any::Any;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::thread::JoinHandle;
use std::time::Instant;
use anyhow::{anyhow, Context, Error, Result};
use libc::c_char;
use mustache::{MapBuilder, Template};
use prover::{codeloc, logic, problem};
use prover::formula::notations::OperatorNotations;
use prover::formula::to_string::FormulaFormatOptions;
use prover::logic::{Logic, LogicFactory, LogicName};
use prover::logic::propositional_logic::PropositionalLogic;
use prover::parser::algorithm::LogicalExpressionParser;
use prover::problem::catalog::get_demo_problem_catalog;
use prover::problem::json::ProblemJSON;
use prover::problem::{Problem, ProblemFlags};
use prover::utils::{parallel_for_each_problem, setup_panicking_from_all_future_threads};

const OUTPUT_DIR_PATH : &str = "./target/html";
const INDEX_FILE_PATH : &str = "./target/html/index.html";
const PROOF_FILE_PATH : &str = "./target/html/proof.html";

const TEMPLATE : &str = include_str!("template.html");

fn main() -> Result<()>
{
    setup_panicking_from_all_future_threads();
    fs::create_dir_all(OUTPUT_DIR_PATH).context(codeloc!())?;

    let args = env::args().collect::<Vec<String>>();
    if args.contains(&String::from("solve-book"))
    {
        prove_problems_from_the_book().context(codeloc!())?;
        create_proof_index_html_file().context(codeloc!())?;

        open_browser(INDEX_FILE_PATH).context(codeloc!())?;
    }
    else if args.len() == 2
    {
        FormulaFormatOptions::DEFAULT_NOTATIONS.with(|default_notations|
            { *(default_notations.borrow_mut()) = OperatorNotations::SoftwareDevelopmentNotations });

        let logic : Rc<dyn Logic> = Rc::new(PropositionalLogic{});
        let statement = LogicalExpressionParser::parse(&logic, &args[1]).context(codeloc!())?;
        let problem = Problem { id:String::from("Problem"), logic, premises:vec![], conclusion:statement, flags:ProblemFlags::default() };

        prove_problem(PROOF_FILE_PATH, problem).context(codeloc!())?;

        open_browser(PROOF_FILE_PATH).context(codeloc!())?;
    }
    else if args.len() == 3
    {
        let logic = LogicFactory::get_logic_by_name(&args[1]).context(codeloc!())?;
        let statement = LogicalExpressionParser::parse(&logic, &args[2]).context(codeloc!())?;
        let problem = Problem { id:String::from("Problem"), logic, premises:vec![], conclusion:statement, flags:ProblemFlags::default() };

        prove_problem(PROOF_FILE_PATH, problem).context(codeloc!())?;

        open_browser(PROOF_FILE_PATH).context(codeloc!())?;
    }
    else
    {
        println!("Graham Priest Introduction to Non-Classical Logic Automated Theorem Prover\n");
        println!("Usage: incl solve-book to solve all problems from the book!");
        println!("Usage: incl <logic> <problem> to solve a problem given as input!");
        println!("Usage: incl <problem> to solve a propositional logic problem given as input!\n");
    }

    return Ok(());
}

fn prove_problems_from_the_book() -> Result<()>
{
    let problems = get_demo_problem_catalog()?.into_iter()
        .flat_map(|book_chapter| book_chapter.problems)
        .collect::<Vec<ProblemJSON>>();

    return parallel_for_each_problem(problems, |problem_json|
    {
        let problem = problem_json.to_problem().context(codeloc!())?;
        let (problem_id, logic) = (problem.id.clone(), problem.logic.clone());
        println!("Solving {}…", problem_id);

        let proof_file_path = format!("{}/{}.html", OUTPUT_DIR_PATH, problem_json.id);
        let mut proof_file = File::create(proof_file_path).context(codeloc!())?;

        let formula_format_options = FormulaFormatOptions::recommended_for(&logic);

        let proof_tree = problem.prove();
        let proof_tree_json = proof_tree.to_json(&formula_format_options).context(codeloc!())?;

        let template = mustache::compile_str(TEMPLATE).context(codeloc!())?;
        let template_data = MapBuilder::new().insert_str("json", proof_tree_json.as_str()).build();
        template.render_data(&mut proof_file, &template_data).context(codeloc!())?;

        return Ok(());
    });
}

fn create_proof_index_html_file() -> Result<()>
{
    let mut output_index_file = File::create(INDEX_FILE_PATH).context(codeloc!())?;
    let mut output_index_html = String::from("<html><head><title>Index</title></head><body>");

    let book_chapters = get_demo_problem_catalog().context(codeloc!())?;
    for book_chapter in &book_chapters
    {
        output_index_html.push_str(format!("<p>{}</p>", book_chapter.name).as_str());
        for problem in &book_chapter.problems
        {
            output_index_html.push_str(format!("<a href=\"{}.html\" target=\"blank\">{}</a><br/>", problem.id, problem.id).as_str());
        }
    }

    output_index_html.push_str("</body></html>");
    output_index_file.write_all(output_index_html.as_bytes()).context(codeloc!())?;

    return Ok(());
}

fn prove_problem(proof_file_path : &str, problem : Problem) -> Result<()>
{
    let mut proof_file = File::create(proof_file_path).context(codeloc!())?;

    let formula_format_options = FormulaFormatOptions::recommended_for(&problem.logic);

    let proof_tree = problem.prove();
    let proof_tree_json = proof_tree.to_json(&formula_format_options).context(codeloc!())?;

    let template = mustache::compile_str(TEMPLATE).context(codeloc!())?;
    let template_data = MapBuilder::new().insert_str("json", proof_tree_json.as_str()).build();
    template.render_data(&mut proof_file, &template_data).context(codeloc!())?;

    return Ok(());
}

fn open_browser(file_path : &str) -> Result<()>
{
    unsafe
    {
        let command_string = format!("chromium {}", file_path);
        let command_cstring = CString::new(command_string).context(codeloc!())?;
        libc::system(command_cstring.as_ptr() as *const c_char);

        return Ok(());
    }
}
