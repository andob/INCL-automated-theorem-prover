use crate::formula::Formula::{And, Atomic, BiImply, Comment, Imply, Or};
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, FuzzyTags, PossibleWorld, PredicateArguments, Sign};
use box_macro::bx;
use itertools::Itertools;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

const UNIFORM_DISTRIBUTION_MAX : u32 = 1_000_000;
const SMALL_LETTER_CHARSET : &str = "qwertyuiopasdfghjklzxcvbnm";

impl Formula
{
    pub fn random(number_of_operators : usize) -> Formula
    {
        loop
        {
            let seed = if number_of_operators == 0 { 0 }
                else if number_of_operators < 10 { 1 }
                else { (number_of_operators / 10) * 2 };

            let formula = Formula::random_impl(seed as f64);
            if formula.count_number_of_operators() == number_of_operators
            {
                return formula;
            }
        }
    }

    pub fn random_impl(seed : f64) -> Formula
    {
        let mut random_number_generator = rand::thread_rng();
        let uniform_distribution = Uniform::from(0..UNIFORM_DISTRIBUTION_MAX);

        let random_number = uniform_distribution.sample(&mut random_number_generator);
        if (random_number as f64) / (UNIFORM_DISTRIBUTION_MAX as f64) >= seed
        {
            let charset = SMALL_LETTER_CHARSET.chars().collect_vec();
            let character = charset[(random_number as usize) % charset.len()];

            let extras = AtomicFormulaExtras
            {
                predicate_args: PredicateArguments::empty(),
                possible_world: PossibleWorld::zero(),
                sign: Sign::Plus,
                fuzzy_tags: FuzzyTags::empty(),
                is_hidden: false,
            };

            return Atomic(character.to_string(), extras);
        }

        let extras = FormulaExtras
        {
            possible_world: PossibleWorld::zero(),
            sign: Sign::Plus,
            fuzzy_tags: FuzzyTags::empty(),
            is_hidden: false,
        };

        let next = || Formula::random_impl(seed / 2.0);
        match uniform_distribution.sample(&mut random_number_generator) % 4
        {
            0 => And(bx!(next()), bx!(next()), extras),
            1 => Or(bx!(next()), bx!(next()), extras),
            2 => Imply(bx!(next()), bx!(next()), extras),
            3 => BiImply(bx!(next()), bx!(next()), extras),
            _ => Comment(String::new()),
        }
    }
}
