use std::collections::HashSet;
use anyhow::{anyhow, Context, Result};
use crate::codeloc;
use crate::formula::notations::OperatorNotations;
use crate::formula::to_string::FormulaFormatOptions;
use crate::problem::json::{BookChapterJSON, ProblemJSON};

pub fn get_demo_problem_catalog() -> Result<Vec<BookChapterJSON>>
{
    return FormulaFormatOptions::DEFAULT_NOTATIONS.with(|operator_notations_ref|
    {
        let json = include_str!("../../../book.json");
        let book_chapters = serde_json::from_str::<Vec<BookChapterJSON>>(json).context(codeloc!())?;

        check_for_duplicate_problem_ids(&book_chapters)?;

        let operator_notations = *operator_notations_ref.borrow();
        if operator_notations == OperatorNotations::BookNotations
        {
            return Ok(book_chapters);
        }

        let book_chapters_with_custom_notations = book_chapters.into_iter()
            .map(|chapter| chapter.with_operator_notations(operator_notations)).collect();

        return Ok(book_chapters_with_custom_notations);
    });
}

fn check_for_duplicate_problem_ids(book_chapters : &Vec<BookChapterJSON>) -> Result<()>
{
    let mut problem_ids : HashSet<&String> = HashSet::new();
    for book_chapter in book_chapters
    {
        for problem in &book_chapter.problems
        {
            if problem_ids.contains(&problem.id)
            {
                return Err(anyhow!("Problem ID {} is duplicated!", problem.id));
            }

            problem_ids.insert(&problem.id);
        }
    }

    return Ok(());
}

impl BookChapterJSON
{
    fn with_operator_notations(self, operator_notations : OperatorNotations) -> BookChapterJSON
    {
        let problems_with_custom_notations = self.problems.into_iter()
            .map(|problem| problem.with_operator_notations(operator_notations)).collect();

        return BookChapterJSON { name: self.name, problems: problems_with_custom_notations };
    }
}

impl ProblemJSON
{
    fn with_operator_notations(self, operator_notations : OperatorNotations) -> ProblemJSON
    {
        let formula_format_options = FormulaFormatOptions
        { notations: operator_notations, should_show_possible_worlds: false, should_show_sign: false };

        return self.to_problem().unwrap().to_json(&formula_format_options);
    }
}
