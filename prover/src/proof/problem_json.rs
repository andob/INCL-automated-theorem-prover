use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::codeloc;
use crate::formula::Formula;
use crate::parser::algorithm::LogicalExpressionParser;
use crate::proof::Problem;

pub struct BookChapter
{
    pub name : String,
    pub problems : Vec<Problem>,
}

#[derive(Serialize, Deserialize)]
struct BookChapterJSON
{
    pub name : String,
    pub problems : Vec<ProblemJSON>,
}

impl BookChapter
{
    pub fn new(name : String, problems : Vec<Problem>) -> BookChapter
    {
        return BookChapter { name, problems };
    }

    pub fn parse_from_json(json : &String) -> Result<Vec<BookChapter>>
    {
        let mut chapters: Vec<BookChapter> = Vec::new();
        let chapters_json : Vec<BookChapterJSON> = serde_json::from_str(json.as_str()).context(codeloc!())?;
        for chapter_json in &chapters_json
        {
            let mut problems: Vec<Problem> = Vec::new();
            for problem_json in &chapter_json.problems
            {
                let problem = Problem::from_json(problem_json).context(codeloc!())?;
                problems.push(problem);
            }

            let chapter = BookChapter::new(chapter_json.name.clone(), problems);
            chapters.push(chapter);
        }

        return Ok(chapters);
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProblemJSON
{
    pub id : String,
    pub logic : String, //todo use this
    pub expected : String, //todo use this
    pub premises : Vec<String>,
    pub conclusion : String,
}

impl Problem
{
    pub fn to_json(&self) -> ProblemJSON
    {
        return ProblemJSON
        {
            id: String::new(), //todo implement
            logic: String::new(), //todo implement
            expected: String::new(), //todo implement
            premises: self.premises.iter().map(|premise| premise.to_string()).collect(),
            conclusion: self.conclusion.to_string(),
        };
    }

    pub fn from_json(json : &ProblemJSON) -> Result<Problem>
    {
        let conclusion = LogicalExpressionParser::parse(&json.conclusion).context(codeloc!())?;

        let mut premises: Vec<Formula> = vec![];
        for premise_json in &json.premises
        {
            let premise = LogicalExpressionParser::parse(premise_json).context(codeloc!())?;
            premises.push(premise);
        }

        return Ok(Problem { premises, conclusion });
    }
}
