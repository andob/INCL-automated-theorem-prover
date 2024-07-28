import init, {
    setup_console_error_panic_hook,
    get_operator_notations,
    set_operator_notations,
    get_logics,
    get_operator_symbols,
    get_problem_catalog,
    solve_problem,
} from './target_wasm.js';

(async () =>
{
    await init();

    setup_console_error_panic_hook();

    let operator_notations = get_operator_notations();
    set_operator_notations(operator_notations[0]);
    console.log(operator_notations);

    let logics = get_logics();
    console.log(logics);

    console.log(get_operator_symbols(logics[0]));

    let start_time = window.performance.now();

    let book_chapters = JSON.parse(get_problem_catalog());
    for (let book_chapter of book_chapters)
    {
        for (let problem of book_chapter.problems)
        {
            try
            {
                let problem_json = JSON.stringify(problem);
                let proof_tree_json = solve_problem(problem_json);
                let proof_tree = JSON.parse(proof_tree_json);

            }
            catch (e)
            {
                console.log(e);
            }
        }
    }

    let stop_time = window.performance.now();
    let delta_time = stop_time - start_time;

    alert("Solved all problems in " + delta_time + "ms!");
})();
