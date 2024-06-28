use std::time::Instant;
use prover::problem::json::ProblemJSON;

#[test]
fn test_full_output()
{
    let input_json = include_str!("./full_output_test/input.json").to_string();
    let expected_output = include_str!("./full_output_test/output.txt").to_string();
    let mut actual_output = String::new();

    let start = Instant::now();
    let problems_json = serde_json::from_str::<Vec<ProblemJSON>>(input_json.as_str()).unwrap();
    for problem_json in problems_json
    {
        let problem = problem_json.to_problem().unwrap();
        let proof_tree = problem.prove();

        actual_output.push_str(format!("{}\n", proof_tree).as_str());
    }

    let elapsed = start.elapsed();
    println!("{}", elapsed.as_millis());

    assert_eq!(expected_output.trim(), actual_output.trim());
}
