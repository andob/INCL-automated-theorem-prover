use crate::formula::Formula::{And, Atomic, BiImply, Comment, Imply, Non, Or};
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PossibleWorld, PredicateArguments, Sign};
use box_macro::bx;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;

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
            let name = random_string::generate(1, SMALL_LETTER_CHARSET);
            let extras = AtomicFormulaExtras
            {
                predicate_args: PredicateArguments::empty(),
                possible_world: PossibleWorld::zero(),
                is_hidden: false, sign: Sign::Plus,
            };

            return Atomic(name, extras);
        }

        let extras = FormulaExtras
        {
            possible_world: PossibleWorld::zero(),
            is_hidden: false, sign: Sign::Plus,
        };

        let next = || Formula::random_impl(seed / 2.0);
        match uniform_distribution.sample(&mut random_number_generator) % 4
        {
            0 => And(bx!(next()), bx!(next()), extras),
            1 => Or(bx!(next()), bx!(next()), extras),
            2 => Imply(bx!(next()), bx!(next()), extras),
            3 => BiImply(bx!(next()), bx!(next()), extras),
            _ => Comment(random_string::generate(1, SMALL_LETTER_CHARSET)),
        }
    }
}
