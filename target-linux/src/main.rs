use std::fs::File;
use std::{env, fs, io};
use std::any::Any;
use std::ffi::CString;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use anyhow::{Context, Result};
use libc::c_char;
use mustache::{MapBuilder, Template};
use prover::{codeloc, logic};
use prover::formula::notations::OperatorNotations;
use prover::formula::to_string::FormulaFormatOptions;
use prover::logic::{Logic, LogicFactory, LogicName};
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
        create_proof_index_html_file(output_dir_path).context(codeloc!())?;

        open_firefox(format!("{}/index.html", output_dir_path)).context(codeloc!())?;
    }
    else if args.len() == 2
    {
        let logic : Rc<dyn Logic> = Rc::new(PropositionalLogic{});
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
    let book_chapters = get_demo_problem_catalog().context(codeloc!())?;
    for book_chapter in &book_chapters
    {
        for problem_json in &book_chapter.problems
        {
            let problem = problem_json.to_problem().context(codeloc!())?;
            let (problem_id, logic) = (problem.id.clone(), problem.logic.clone());

            let proof_file_path = format!("{}/{}.html", output_dir_path, problem_id);
            let mut proof_file = File::create(proof_file_path).context(codeloc!())?;

            let formula_format_options = FormulaFormatOptions
            {
                notations: OperatorNotations::BookNotations,
                should_show_possible_worlds: logic.get_name().is_modal_logic(),
                should_show_sign: logic.get_name().is_three_valued_logic(),
            };

            let proof_tree = problem.prove();
            let proof_tree_json = proof_tree.to_json(&formula_format_options).context(codeloc!())?;

            let template_data = MapBuilder::new().insert_str("json", proof_tree_json.as_str()).build();
            template.render_data(&mut proof_file, &template_data).context(codeloc!())?;
        }
    }

    return Ok(());
}

fn create_proof_index_html_file(output_dir_path : &str) -> Result<()>
{
    let output_index_file_path = format!("{}/index.html", output_dir_path);
    let mut output_index_file = File::create(output_index_file_path).context(codeloc!())?;
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

fn prove_problem(template : Template, proof_file_path : &String, problem : Problem) -> Result<()>
{
    let mut proof_file = File::create(proof_file_path).context(codeloc!())?;

    let formula_format_options = FormulaFormatOptions
    {
        notations: OperatorNotations::ComputerScienceNotations,
        should_show_possible_worlds: false,
        should_show_sign: false,
    };

    let proof_tree = problem.prove();
    let proof_tree_json = proof_tree.to_json(&formula_format_options).context(codeloc!())?;

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
