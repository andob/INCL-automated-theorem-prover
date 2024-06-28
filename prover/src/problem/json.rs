use std::rc::Rc;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::codeloc;
use crate::formula::Formula;
use crate::logic::{Logic, LogicFactory};
use crate::logic::propositional_logic::PropositionalLogic;
use crate::parser::algorithm::LogicalExpressionParser;
use crate::problem::Problem;

#[derive(Serialize, Deserialize)]
pub struct BookChapterJSON
{
    pub name : String,
    pub problems : Vec<ProblemJSON>,
}

#[derive(Serialize, Deserialize)]
pub struct ProblemJSON
{
    pub id : String,
    pub logic : String,
    pub expected : String,
    pub premises : Vec<String>,
    pub conclusion : String,
}

impl ProblemJSON
{
    pub fn from_problem(problem : &Problem) -> ProblemJSON
    {
        return problem.to_json();
    }

    pub fn to_problem(&self) -> Result<Problem>
    {
        return Problem::from_json(self);
    }
}

impl Problem
{
    pub fn to_json(&self) -> ProblemJSON
    {
        return ProblemJSON
        {
            id: self.id.clone(),
            logic: self.logic.get_name().to_string(),
            expected: String::new(),
            premises: self.premises.iter().map(|premise| premise.to_string()).collect(),
            conclusion: self.conclusion.to_string(),
        };
    }

    pub fn from_json(json : &ProblemJSON) -> Result<Problem>
    {
        let logic = LogicFactory::get_logic_by_name(&json.logic).context(codeloc!())?;

        let conclusion = LogicalExpressionParser::parse(&logic, &json.conclusion).context(codeloc!())?;

        let mut premises : Vec<Formula> = vec![];
        for premise_json in &json.premises
        {
            let premise = LogicalExpressionParser::parse(&logic, premise_json).context(codeloc!())?;
            premises.push(premise);
        }

        return Ok(Problem { id: json.id.clone(), logic, premises, conclusion });
    }
}
