use prover::formula::to_string::FormulaFormatOptions;
use prover::problem::catalog::get_demo_problem_catalog;
use prover::problem::json::ProblemJSON;

const EXPECTED_TIMEOUT : &str = "timeout";
const EXPECTED_PROVED : &str = "proved";
const EXPECTED_DISPROVED : &str = "disproved";

#[test]
fn test_full_output()
{
    let input_json = include_str!("full_output_test/input.json").to_string();
    let expected_output = include_str!("full_output_test/output.txt").to_string();
    let mut actual_output = String::new();

    let problems_json = serde_json::from_str::<Vec<ProblemJSON>>(input_json.as_str()).unwrap();
    for problem_json in problems_json
    {
        let problem = problem_json.to_problem().unwrap();
        let logic = problem.logic.clone();

        let mut formula_format_options = FormulaFormatOptions::default();
        formula_format_options.should_show_possible_worlds = logic.get_name().is_modal_logic();
        formula_format_options.should_show_sign = logic.get_semantics().number_of_truth_values()>2;

        let proof_tree = problem.prove();
        let proof_tree_string = proof_tree.to_string_with_options(&formula_format_options);

        actual_output.push_str(format!("{}\n", proof_tree_string).as_str());
        println!("{}\n", proof_tree_string);
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

            if problem_json.expected == EXPECTED_TIMEOUT && !proof_tree.has_timeout
            {
                eprintln!("Expected problem {} to timeout but it did not!", problem_id);
                assert!(proof_tree.has_timeout);
            }

            if problem_json.expected == EXPECTED_PROVED && proof_tree.has_timeout
            {
                eprintln!("Expected problem {} to be proved but it timed out!", problem_id);
                assert!(!proof_tree.has_timeout);
            }

            if problem_json.expected == EXPECTED_DISPROVED && proof_tree.has_timeout
            {
                eprintln!("Expected problem {} to be disproved but it timed out!", problem_id);
                assert!(!proof_tree.has_timeout);
            }

            if problem_json.expected == EXPECTED_PROVED && !proof_tree.is_proof_correct
            {
                eprintln!("Expected problem {} to be proved but it was disproved!", problem_id);
                assert!(proof_tree.is_proof_correct);
            }

            if problem_json.expected == EXPECTED_DISPROVED && proof_tree.is_proof_correct
            {
                eprintln!("Expected problem {} to be disproved but it was proved!", problem_id);
                assert!(!proof_tree.is_proof_correct);
            }
        }
    }
}
