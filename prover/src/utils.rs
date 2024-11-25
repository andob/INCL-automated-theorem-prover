use anyhow::Result;
use std::collections::BTreeMap;
use std::{panic, process, thread};
use std::thread::JoinHandle;
use anyhow::Error;
use crossbeam_channel::{Receiver, Sender};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::logic::{LogicFactory, LogicName};
use crate::problem::json::ProblemJSON;

#[macro_export]
macro_rules! codeloc
{
    () => { format!("{}:{}", file!(), line!()) }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, EnumIter)]
enum Difficulty { VeryHigh, High, Medium, Low }

pub fn parallel_for_each_problem(problems : Vec<ProblemJSON>, consumer : fn(ProblemJSON) -> Result<()>) -> Result<()>
{
    let logics = LogicFactory::get_logic_theories();

    let problems = problems.into_iter().map(|problem|
    {
        let logic_name = logics.iter()
            .map(|logic| logic.get_name())
            .find(|logic_name| logic_name.to_string() == problem.logic)
            .unwrap_or(LogicName::of(""));

        match (logic_name.is_intuitionistic_logic(), logic_name.is_first_order_logic())
        {
            (true, true) => { (Difficulty::VeryHigh, problem) }
            (true, false) => { (Difficulty::High, problem) }
            (false, true) => { (Difficulty::Medium, problem) }
            (false, false) => { (Difficulty::Low, problem) }
        }
    }).collect::<Vec<(Difficulty, ProblemJSON)>>();

    let channels = Difficulty::iter()
        .map(|difficulty| (difficulty, crossbeam_channel::unbounded::<ProblemJSON>()))
        .collect::<BTreeMap<Difficulty, (Sender<ProblemJSON>, Receiver<ProblemJSON>)>>();

    for (problem_difficulty, problem) in problems
    {
        for (channel_difficulty, (problem_sender, _problem_receiver)) in &channels
        {
            if problem_difficulty == *channel_difficulty
            {
                problem_sender.send(problem.clone()).unwrap();
            }
        }
    }

    setup_panicking_from_all_future_threads();

    let number_of_cpus = num_cpus::get();
    let mut join_handles: Vec<JoinHandle<Result<(), Error>>> = vec![];
    for _cpu_index in 0..number_of_cpus
    {
        let mut problem_receivers: Vec<Receiver<ProblemJSON>> = vec![];
        for (_channel_difficulty, (_problem_sender, problem_receiver)) in &channels
        {
            problem_receivers.push(problem_receiver.clone());
        }

        join_handles.push(thread::spawn(move ||
        {
            for problem_receiver in problem_receivers
            {
                while let Ok(problem) = problem_receiver.try_recv()
                {
                    let result = consumer(problem);
                    if result.is_err() { return result };
                }
            }

            return Ok(());
        }));
    }

    for join_handle in join_handles
    {
        let result = join_handle.join().unwrap();
        if result.is_err() { return result };
    }

    return Ok(());
}

fn setup_panicking_from_all_future_threads()
{
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info|
    {
        original_hook(panic_info);
        process::exit(1);
    }));
}
