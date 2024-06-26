use anyhow::Result;

use prover::{test, test2};

fn main() -> Result<()>
{
    test()?;
    test2()?;

    return Ok(());
}
