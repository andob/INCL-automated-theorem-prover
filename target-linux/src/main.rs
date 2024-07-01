use std::fs::File;
use std::{env, fs, io};
use std::ffi::CString;
use std::io::Write;
use std::path::Path;
use anyhow::{Context, Result};
use libc::c_char;
use mustache::{MapBuilder, Template};
use prover::codeloc;
use prover::formula::notations::OperatorNotations;
use prover::logic::{Logic, LogicFactory};
use prover::logic::propositional_logic::PropositionalLogic;
use prover::parser::algorithm::LogicalExpressionParser;
use prover::problem::catalog::get_demo_problem_catalog;
use prover::problem::Problem;

fn main() -> Result<()>
{
    let output_dir_path = "./target/html";
    fs::create_dir_all(output_dir_path).context(codeloc!())?;

    let template = mustache::compile_str(include_str!("template.html")).context(codeloc!())?;

    let args = env::args().collect::<Vec<String>>();
    if args.contains(&String::from("solve-book"))
    {
        prove_problems_from_the_book(template, output_dir_path).context(codeloc!())?;

        open_firefox(format!("{}/index.html", output_dir_path)).context(codeloc!())?;
    }
    else if args.len() == 2
    {
        let logic : Box<dyn Logic> = Box::new(PropositionalLogic{});
        let statement = LogicalExpressionParser::parse(&logic, &args[1]).context(codeloc!())?;
        let problem = Problem { id:String::from("Problem"), logic, premises:vec![], conclusion:statement };

        let proof_file_path = format!("{}/proof.html", output_dir_path);
        prove_problem(template, &proof_file_path, problem).context(codeloc!())?;

        open_firefox(proof_file_path).context(codeloc!())?;
    }
    else if args.len() == 3
    {
        let logic = LogicFactory::get_logic_by_name(&args[1]).context(codeloc!())?;
        let statement = LogicalExpressionParser::parse(&logic, &args[2]).context(codeloc!())?;
        let problem = Problem { id:String::from("Problem"), logic, premises:vec![], conclusion:statement };

        let proof_file_path = format!("{}/proof.html", output_dir_path);
        prove_problem(template, &proof_file_path, problem).context(codeloc!())?;

        open_firefox(proof_file_path).context(codeloc!())?;
    }
    else
    {
        print!("\nGraham Priest Introduction to Non-Classical Logic Automated Theorem Prover");
        print!("\n\nUsage: incl solve-book to solve all problems from the book!");
        print!("\nUsage: incl <logic> <problem> to solve a problem given as input!");
        print!("\nUsage: incl <problem> to solve a propositional logic problem given as input!");
        print!("\n\n∃ ∀ ◇ □ ¬ ∧ ∨ → ≡\n");
    }

    return Ok(());
}

fn prove_problems_from_the_book(template : Template, output_dir_path : &str) -> Result<()>
{
    let output_index_file_path = format!("{}/index.html", output_dir_path);
    let mut output_index_file = File::create(output_index_file_path).context(codeloc!())?;
    let mut output_index_html = String::from("<html><head><title>Index</title></head><body>");

    let book_chapters = get_demo_problem_catalog().context(codeloc!())?;
    for book_chapter in &book_chapters
    {
        for problem_json in &book_chapter.problems
        {
            let problem_id = &problem_json.id;
            let problem = problem_json.to_problem().context(codeloc!())?;
            let proof_tree = problem.prove();

            let proof_file_path = format!("{}/{}.html", output_dir_path, problem_id);
            let mut proof_file = File::create(proof_file_path).context(codeloc!())?;

            let proof_tree_json = proof_tree.to_json(OperatorNotations::BookNotations).context(codeloc!())?;
            let template_data = MapBuilder::new().insert_str("json", proof_tree_json.as_str()).build();
            template.render_data(&mut proof_file, &template_data).context(codeloc!())?;

            output_index_html.push_str(format!("<a href=\"{}.html\" target=\"blank\">{}</a><br/>", problem_id, problem_id).as_str());
        }
    }

    output_index_html.push_str("</body></html>");
    output_index_file.write_all(output_index_html.as_bytes()).context(codeloc!())?;

    return Ok(());
}

fn prove_problem(template : Template, proof_file_path : &String, problem : Problem) -> Result<()>
{
    let mut proof_file = File::create(proof_file_path).context(codeloc!())?;

    let proof_tree = problem.prove();
    let proof_tree_json = proof_tree.to_json(OperatorNotations::ComputerScienceNotations).context(codeloc!())?;

    let template_data = MapBuilder::new().insert_str("json", proof_tree_json.as_str()).build();
    template.render_data(&mut proof_file, &template_data).context(codeloc!())?;

    return Ok(());
}

fn open_firefox(file_path : String) -> Result<()>
{
    unsafe
    {
        let command_string = format!("firefox {}", file_path);
        let command_cstring = CString::new(command_string).context(codeloc!())?;
        libc::system(command_cstring.as_ptr() as *const c_char);

        return Ok(());
    }
}
