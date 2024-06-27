use anyhow::{Context, Result};
use crate::codeloc;
use crate::problem::json::BookChapterJSON;

pub fn get_demo_problem_catalog() -> Result<Vec<BookChapterJSON>>
{
    let json = include_str!("../../../book.json");
    return Ok(serde_json::from_str(json).context(codeloc!())?);
}
