use anyhow::Result;
use std::collections::BTreeMap;
use std::{panic, process, thread};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::thread::JoinHandle;
use anyhow::Error;
use crossbeam_channel::{Receiver, Sender};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::logic::{LogicFactory, LogicName};
use crate::problem::json::ProblemJSON;

#[macro_export]
macro_rules! codeloc { () => { format!("{}:{}", file!(), line!()) } }

pub const CONFIG_KEY_MIN_COUNTERMODEL_GRAPH_NODES : &str = "min_countermodel_graph_nodes";
pub const CONFIG_KEY_MAX_COUNTERMODEL_GRAPH_NODES : &str = "max_countermodel_graph_nodes";
pub const CONFIG_KEY_BENCHMARK : &str = "benchmark";

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub fn get_config_value<R>(key : &str) -> Option<R> where R : FromStr, R : Default, R : Display
{
    let key_with_delimiter = format!("{}:", key);

    return std::env::args().into_iter()
        .find(|arg| arg.starts_with(key_with_delimiter.as_str()))
        .map(|arg| arg.trim_start_matches(key_with_delimiter.as_str()).to_string())
        .map(|arg| R::from_str(arg.as_str()).ok())
        .unwrap_or_default();
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub fn get_config_value<R>(key : &str) -> Option<R> where R : FromStr, R : Default, R : Display
{
    let window = web_sys::window()?;
    let url_args_raw = window.location().search().ok()?;
    let url_args = web_sys::UrlSearchParams::new_with_str(url_args_raw.as_str()).ok()?;
    let url_arg = url_args.get(key)?;
    return R::from_str(url_arg.as_str()).ok();
}

#[inline(always)]
pub fn measure_total_number_of_allocated_bytes<F>(function : F) -> f64 where F : FnOnce() -> ()
{
    if get_config_value::<bool>(CONFIG_KEY_BENCHMARK).unwrap_or_default()
    {
        let allocation_info = allocation_counter::measure(function);
        return allocation_info.bytes_total as f64;
    }
    else
    {
        function();
        return 0.0;
    }
}

pub fn setup_panicking_from_all_future_threads()
{
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info|
    {
        original_hook(panic_info);
        process::exit(1);
    }));
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
