use anyhow::anyhow;
use anyhow::Result;
use smol_str::SmolStr;
use crate::formula::Formula;

pub struct FirstOrderLogicParserAssertions {}
impl FirstOrderLogicParserAssertions
{
    pub fn run_assertions(p : &Formula) -> Result<()>
    {
        return Self::run_assertions_impl(p, &Vec::new());
    }

    fn run_assertions_impl(p : &Formula, variable_stack : &Vec<SmolStr>) -> Result<()>
    {
        match p
        {
            Formula::Exists(x, box q, _) |
            Formula::ForAll(x, box q, _) =>
            {
                if !x.is_variable()
                {
                    return Err(anyhow!("Invalid syntax: in {}, {} should be a variable!", p, x))
                }

                if variable_stack.contains(&x.variable_name)
                {
                    return Err(anyhow!("Invalid syntax: duplicate variable {}", x))
                }

                let mut variable_stack = variable_stack.clone();
                variable_stack.push(x.variable_name.clone());
                Self::run_assertions_impl(q, &variable_stack)?;
            }

            Formula::Atomic(_, extras) =>
            {
                for arg in extras.predicate_args.iter()
                {
                    if arg.is_variable() && arg.is_rigid_designator() &&
                        !variable_stack.contains(&arg.variable_name)
                    {
                        return Err(anyhow!("Invalid syntax: in {}, {} should be an object!", p, arg))
                    }
                }
            }

            Formula::Non(box q, _) |
            Formula::Possible(box q, _) |
            Formula::Necessary(box q, _) |
            Formula::InPast(box q, _) |
            Formula::InFuture(box q, _) =>
            {
                Self::run_assertions_impl(q, variable_stack)?;
            }

            Formula::And(box q, box w, _) |
            Formula::Or(box q, box w, _) |
            Formula::Imply(box q, box w, _) |
            Formula::BiImply(box q, box w, _) |
            Formula::StrictImply(box q, box w, _) |
            Formula::Conditional(box q, box w, _) =>
            {
                Self::run_assertions_impl(q, variable_stack)?;
                Self::run_assertions_impl(w, variable_stack)?;
            }

            _ => {}
        }

        return Ok(());
    }
}
