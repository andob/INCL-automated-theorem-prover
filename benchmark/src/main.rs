#![feature(test)]

mod csv;

extern crate test;

fn main()
{
}

#[cfg(test)]
mod tests
{
    use std::hint::black_box;
    use super::*;
    use test::Bencher;

    #[bench]
    fn generate_csv(bencher : &mut Bencher)
    {
        black_box(csv::generate_csv()).unwrap();
    }
}
