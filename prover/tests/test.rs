use anyhow::Result;
use prover::problem::catalog::get_demo_problem_catalog;
use prover::problem::json::ProblemJSON;
use prover::utils::{parallel_for_each_problem, setup_panicking_from_all_future_threads};

const EXPECTED_TIMEOUT : &str = "timeout";
const EXPECTED_PROVED : &str = "proved";
const EXPECTED_DISPROVED : &str = "disproved";

#[test]
fn test_proof_status() -> Result<()>
{
    let problems = get_demo_problem_catalog()?.into_iter()
        .flat_map(|book_chapter| book_chapter.problems)
        .collect::<Vec<ProblemJSON>>();

    setup_panicking_from_all_future_threads();
    return parallel_for_each_problem(problems, |problem_json|
    {
        let problem_id = &problem_json.id;
        let problem = problem_json.to_problem().unwrap();
        let proof_tree = problem.prove();

        if problem_json.expected == EXPECTED_TIMEOUT && !proof_tree.has_timeout
        {
            eprintln!("\nExpected problem {} to timeout but it did not!", problem_id);
            eprintln!("\n It was {}proved!", if !proof_tree.is_proof_correct { "dis" } else { "" });
            assert!(proof_tree.has_timeout);
        }

        if problem_json.expected == EXPECTED_PROVED && proof_tree.has_timeout
        {
            eprintln!("\nExpected problem {} to be proved but it timed out!", problem_id);
            assert!(!proof_tree.has_timeout);
        }

        if problem_json.expected == EXPECTED_DISPROVED && proof_tree.has_timeout
        {
            eprintln!("\nExpected problem {} to be disproved but it timed out!", problem_id);
            assert!(!proof_tree.has_timeout);
        }

        if problem_json.expected == EXPECTED_PROVED && !proof_tree.is_proof_correct
        {
            eprintln!("\nExpected problem {} to be proved but it was disproved!", problem_id);
            assert!(proof_tree.is_proof_correct);
        }

        if problem_json.expected == EXPECTED_DISPROVED && proof_tree.is_proof_correct
        {
            eprintln!("\nExpected problem {} to be disproved but it was proved!", problem_id);
            assert!(!proof_tree.is_proof_correct);
        }

        return Ok(());
    });
}
