use anyhow::{Context, Result};
use crate::codeloc;
use crate::proof::problem_json::BookChapter;

pub fn get_demo_problem_catalog() -> Result<Vec<BookChapter>>
{
    let json = include_str!("../../../book.json").to_string();
    return BookChapter::parse_from_json(&json).context(codeloc!());
}
