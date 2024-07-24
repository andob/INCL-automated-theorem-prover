use wasm_bindgen::prelude::*;
use web_sys::window;
use prover::problem::catalog::get_demo_problem_catalog;
use crate::utils::set_panic_hook;

mod utils;

#[wasm_bindgen]
extern "C"
{
    fn alert(message : &str);
}

#[wasm_bindgen]
pub fn test_solve_all_problems()
{
    set_panic_hook();

    let start_time = window().unwrap().performance().unwrap().now();

    let book_chapters = get_demo_problem_catalog().unwrap();
    for book_chapter in &book_chapters
    {
        for problem_json in &book_chapter.problems
        {
            let problem = problem_json.to_problem().unwrap();
            problem.prove();
        }
    }

    let stop_time = window().unwrap().performance().unwrap().now();
    let delta_time = (stop_time - start_time) as u64;

    let message = format!("Took {}ms to solve all problems!", delta_time);
    alert(message.as_str());
}
