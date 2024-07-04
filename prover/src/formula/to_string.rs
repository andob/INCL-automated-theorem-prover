use std::fmt::{Display, Formatter};
use crate::formula::{Formula, PossibleWorld, PredicateArgument, PredicateArguments};
use crate::formula::notations::OperatorNotations;
use crate::parser::token_types::TokenTypeID;

impl Display for Formula
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        let options = FormulaFormatOptions::default();
        return write!(f, "{}", self.to_string_with_options(&options));
    }
}

pub struct FormulaFormatOptions
{
    pub notations : OperatorNotations,
    pub should_show_possible_worlds : bool,
}

impl FormulaFormatOptions
{
    pub fn default() -> FormulaFormatOptions
    {
        return FormulaFormatOptions
        {
            notations: OperatorNotations::BookNotations,
            should_show_possible_worlds: true,
        }
    }
}

impl Formula
{
    pub fn to_string_with_options(&self, options : &FormulaFormatOptions) -> String
    {
        let mut formula_string = self.to_string_impl(options, 0);

        if options.should_show_possible_worlds && !matches!(self, Formula::Comment(..))
        {
            formula_string.push(' ');
            formula_string.push_str(self.get_possible_world().to_string().as_str());
        }

        return formula_string;
    }

    fn to_string_impl(&self, options: &FormulaFormatOptions, index : usize) -> String
    {
        let format_binary_formula = |x : &Box<Formula>, operator : char, y : &Box<Formula>|
        if index==0 { format!("{} {} {}", x.to_string_impl(options, index+1), operator, y.to_string_impl(options, index+1)) }
        else { format!("({} {} {})", x.to_string_impl(options, index+1), operator, y.to_string_impl(options, index+1)) };

        return match self
        {
            Formula::Atomic(p, args) =>
            {
                if args.predicate_args.is_empty() { return p.clone() };
                return format!("{}[{}]", p, args.predicate_args);
            }

            Formula::Non(p, _) =>
            {
                let non = options.notations.get_operator_character(TokenTypeID::Non);
                return format!("{}{}", non, p.to_string_impl(options, index+1));
            }

            Formula::And(p, q, _) =>
            {
                let and = options.notations.get_operator_character(TokenTypeID::And);
                return format_binary_formula(p, and, q);
            }

            Formula::Or(p, q, _) =>
            {
                let or = options.notations.get_operator_character(TokenTypeID::Or);
                return format_binary_formula(p, or, q);
            }

            Formula::Imply(p, q, _) =>
            {
                let imply = options.notations.get_operator_character(TokenTypeID::Imply);
                return format_binary_formula(p, imply, q);
            }

            Formula::BiImply(p, q, _) =>
            {
                let bi_imply = options.notations.get_operator_character(TokenTypeID::BiImply);
                return format_binary_formula(p, bi_imply, q);
            }

            Formula::StrictImply(p, q, _) =>
            {
                return format_binary_formula(p, '⥽', q);
            }

            Formula::Exists(x, p, _) =>
            {
                return format!("∃{}({})", x, p.to_string_impl(options, index+1));
            }

            Formula::ForAll(x, p, _) =>
            {
                return format!("∀{}({})", x, p.to_string_impl(options, index+1));
            }

            Formula::Possible(p, _) =>
            {
                return format!("◇{}", p.to_string_impl(options, index+1));
            }

            Formula::Necessary(p, _) =>
            {
                return format!("□{}", p.to_string_impl(options, index+1));
            }

            Formula::Comment(payload) =>
            {
                return payload.clone();
            }
        }
    }
}

impl Display for PredicateArguments
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        let args_as_string = self.args.iter()
            .map(|arg|arg.to_string())
            .intersperse(String::from(",")).collect::<String>();

        return write!(f, "{}", args_as_string);
    }
}

impl Display for PredicateArgument
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return if let Some(instance_name) = &self.instance_name
            { write!(f, "{}:{}", instance_name, self.type_name) }
        else { write!(f, "{}", self.type_name) };
    }
}

impl Display for PossibleWorld
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "w{}", self.index);
    }
}
