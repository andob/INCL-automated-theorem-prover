use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use prover::{codeloc, test};

#[derive(Serialize, Deserialize)]
struct RootJSONNode
{
    chapters : Vec<BookChapterJSON>
}

#[derive(Serialize, Deserialize)]
struct BookChapterJSON
{
    name : String,
    problems : Vec<ProblemJSON>,
}

#[derive(Serialize, Deserialize)]
struct ProblemJSON
{
    id : String,
    logic : String,
    expected : String,
    premises : Vec<String>,
    conclusion : String,
}

fn main() -> Result<()>
{
    let raw_json = include_str!("../../problems.json");
    let parsed_json : RootJSONNode = serde_json::from_str(raw_json).context(codeloc!())?;

    for chapter in &parsed_json.chapters
    {
        for problem in &chapter.problems
        {
            for premise in &problem.premises
            {
                test(premise.clone())?;
            }

            test(problem.conclusion.clone())?;
        }
    }

    return Ok(());
}
