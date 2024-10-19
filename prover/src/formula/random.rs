use box_macro::bx;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{Rng, RngCore};
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PossibleWorld, PredicateArguments, Sign};
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Conditional, Imply, Necessary, Non, Or, Possible, StrictImply};

const SMALL_LETTER_CHARSET : &str = "qwertyuiopasdfghjklzxcvbnm";

impl Distribution<Formula> for Standard
{
    fn sample<R : Rng + ?Sized>(&self, random : &mut R) -> Formula
    {
        let probability = random.gen();
        if random.gen_bool(probability)
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

        return match random.next_u32() % 5
        {
            0 => Non(bx!(random.gen()), extras),
            1 => And(bx!(random.gen()), bx!(random.gen()), extras),
            2 => Or(bx!(random.gen()), bx!(random.gen()), extras),
            3 => Imply(bx!(random.gen()), bx!(random.gen()), extras),
            4 => BiImply(bx!(random.gen()), bx!(random.gen()), extras),
            _ => Comment(random_string::generate(1, SMALL_LETTER_CHARSET)),
        };
    }
}
