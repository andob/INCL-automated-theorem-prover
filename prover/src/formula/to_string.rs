use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use itertools::Itertools;
use crate::formula::{Formula, FuzzyTag, FuzzyTags, PossibleWorld, PredicateArgument, PredicateArguments, Sign};
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Conditional, DefinitelyExists, Equals, Exists, ForAll, GreaterOrEqualThan, Imply, InFuture, InPast, LessThan, Necessary, Non, Or, Possible, StrictImply};
use crate::formula::notations::OperatorNotations;
use crate::logic::Logic;
use crate::parser::token_types::TokenTypeID;

impl Display for Formula
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        let options = FormulaFormatOptions::default();
        return write!(f, "{}", self.to_string_with_options(&options));
    }
}

impl Debug for Formula
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
    should_show_possible_worlds : bool,
    should_show_sign : bool,
    should_show_fuzzy_tags : bool,
}

impl FormulaFormatOptions
{
    thread_local!
    {
        pub static DEFAULT_NOTATIONS : Rc<RefCell<OperatorNotations>> =
            Rc::new(RefCell::new(OperatorNotations::BookNotations));
    }

    pub fn default() -> FormulaFormatOptions
    {
        return Self::DEFAULT_NOTATIONS.with(|default_notations|
        {
            return FormulaFormatOptions
            {
                notations: *default_notations.borrow(),
                should_show_possible_worlds: false,
                should_show_sign: false,
                should_show_fuzzy_tags: false,
            }
        })
    }

    pub fn recommended_for(logic : &Rc<dyn Logic>) -> FormulaFormatOptions
    {
        let mut formula_format_options = FormulaFormatOptions::default();
        formula_format_options.should_show_possible_worlds = logic.get_name().is_modal_logic();

        let number_of_truth_values = logic.get_semantics().number_of_truth_values();
        formula_format_options.should_show_sign = number_of_truth_values > 2;
        formula_format_options.should_show_fuzzy_tags = number_of_truth_values == u8::MAX;

        return formula_format_options;
    }
}

impl Formula
{
    pub fn to_string_with_options(&self, options : &FormulaFormatOptions) -> String
    {
        let mut formula_string = self.to_string_impl(options, 0);

        let is_comment = matches!(self, Comment(..));
        let is_inequality = matches!(self, LessThan(..) | GreaterOrEqualThan(..));

        if options.should_show_possible_worlds && !is_comment
        {
            formula_string = format!("{} {}", formula_string, self.get_possible_world());
        }

        if options.should_show_sign && !is_comment && !is_inequality
        {
            formula_string = format!("{} {}", formula_string, self.get_sign());
        }

        if options.should_show_fuzzy_tags && !is_comment && !is_inequality
        {
            formula_string = format!("{{ {} }} {}", self.get_fuzzy_tags(), formula_string);
        }

        if self.is_hidden()
        {
            formula_string = format!("[HIDDEN] {}", formula_string);
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
            Atomic(p, args) =>
            {
                if args.predicate_args.is_empty() { return p.to_string() };
                return format!("{}[{}]", p, args.predicate_args);
            }

            Non(p, _) =>
            {
                let non = options.notations.get_operator_character(TokenTypeID::Non);
                return format!("{}{}", non, p.to_string_impl(options, index+1));
            }

            And(p, q, _) =>
            {
                let and = options.notations.get_operator_character(TokenTypeID::And);
                return format_binary_formula(p, and, q);
            }

            Or(p, q, _) =>
            {
                let or = options.notations.get_operator_character(TokenTypeID::Or);
                return format_binary_formula(p, or, q);
            }

            Imply(p, q, _) =>
            {
                let imply = options.notations.get_operator_character(TokenTypeID::Imply);
                return format_binary_formula(p, imply, q);
            }

            BiImply(p, q, _) =>
            {
                let bi_imply = options.notations.get_operator_character(TokenTypeID::BiImply);
                return format_binary_formula(p, bi_imply, q);
            }

            StrictImply(p, q, _) =>
            {
                return format_binary_formula(p, '⥽', q);
            }

            Conditional(p, q, _) =>
            {
                return format_binary_formula(p, 'ᐅ', q);
            }

            Exists(x, p, _) =>
            {
                return format!("∃{}({})", x, p.to_string_impl(options, index+1));
            }

            ForAll(x, p, _) =>
            {
                return format!("∀{}({})", x, p.to_string_impl(options, index+1));
            }

            Equals(x, y, _) =>
            {
                return if index == 0 { format!("{} = {}", x, y) }
                else { format!("({} = {})", x, y) };
            }

            DefinitelyExists(x, _) =>
            {
                return format!("𝔈{}", x);
            }

            Possible(p, _) =>
            {
                return format!("◇{}", p.to_string_impl(options, index+1));
            }

            Necessary(p, _) =>
            {
                return format!("□{}", p.to_string_impl(options, index+1));
            }

            InPast(p, _) =>
            {
                return format!("ᵖ{}", p.to_string_impl(options, index+1));
            }

            InFuture(p, _) =>
            {
                return format!("ᶠ{}", p.to_string_impl(options, index+1));
            }

            LessThan(x, y, _) =>
            {
                return if index == 0 { format!("{} < {}", x, y) }
                else { format!("({} < {})", x, y) };
            }

            GreaterOrEqualThan(x, y, _) =>
            {
                return if index == 0 { format!("{} ≥ {}", x, y) }
                else { format!("({} ≥ {})", x, y) };
            }

            Comment(payload) =>
            {
                return payload.to_string();
            }
        }
    }
}

impl Display for PredicateArguments
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        let args_as_string = self.args.iter()
            .map(|arg| arg.to_string())
            .intersperse(String::from(","))
            .collect::<String>();

        return write!(f, "{}", args_as_string);
    }
}

impl Display for PredicateArgument
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return if self.is_instantiated()
            { write!(f, "{}:{}", self.object_name, self.variable_name) }
        else { write!(f, "{}", self.variable_name) };
    }
}

impl Display for PossibleWorld
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "w{}", self.index);
    }
}

impl Debug for PossibleWorld
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "w{}", self.index);
    }
}

impl Display for Sign
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", match self
        {
            Sign::Plus => { '+' }
            Sign::Minus => { '-' }
        })
    }
}

impl Display for FuzzyTags
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        if self.is_empty() { return write!(f, "0") };

        let mut tags_as_string = self.tags.iter()
            .map(|tag| tag.to_string())
            .intersperse(String::from(" "))
            .collect::<String>();

        //trim trailing plus prefix (eg: "+ a + b + c" => "a + b + c")
        tags_as_string = tags_as_string.trim_start_matches("+ ").to_string();

        return write!(f, "{}", tags_as_string);
    }
}

impl Display for FuzzyTag
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{} {}", self.sign, self.object_name);
    }
}
