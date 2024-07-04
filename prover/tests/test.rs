use prover::formula::notations::OperatorNotations;
use prover::formula::to_string::FormulaFormatOptions;
use prover::problem::catalog::get_demo_problem_catalog;
use prover::problem::json::ProblemJSON;

#[test]
fn test_full_output()
{
    let input_json = include_str!("./full_output_test/input.json").to_string();
    let expected_output = include_str!("./full_output_test/output.txt").to_string();
    let mut actual_output = String::new();

    let problems_json = serde_json::from_str::<Vec<ProblemJSON>>(input_json.as_str()).unwrap();
    for problem_json in problems_json
    {
        let problem = problem_json.to_problem().unwrap();
        let proof_tree = problem.prove();

        let formula_format_options = FormulaFormatOptions
        {
            notations: OperatorNotations::BookNotations,
            should_show_possible_worlds: false,
        };

        let output_chunk = format!("{}\n", proof_tree.to_string_with_options(&formula_format_options));
        actual_output.push_str(output_chunk.as_str());
        println!("{}\n", output_chunk);
    }

    assert_eq!(actual_output.trim(), expected_output.trim());
}

#[test]
fn test_proof_status()
{
    let book_chapters = get_demo_problem_catalog().unwrap();
    for book_chapter in &book_chapters
    {
        for problem_json in &book_chapter.problems
        {
            let problem_id = &problem_json.id;
            let problem = problem_json.to_problem().unwrap();
            let proof_tree = problem.prove();

            println!("\n{}\n{}\n{}", book_chapter.name, problem_id, proof_tree);

            if problem_json.expected == "proved" && !proof_tree.is_proof_correct
            {
                eprintln!("Expected problem {} to be proved but it was not proved!", problem_id);
                assert!(proof_tree.is_proof_correct);
            }

            if problem_json.expected == "not-proved" && proof_tree.is_proof_correct
            {
                eprintln!("Expected problem {} to be not proved but it was proved!", problem_id);
                assert!(!proof_tree.is_proof_correct);
            }
        }
    }
}
