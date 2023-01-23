pub mod pairs;

use clap::Subcommand;
pub use pairs::Pairs;

#[derive(Subcommand, Debug)]
pub enum Sum {
    /// Find pairs of numbers whose sum is equal to a given target.
    Pairs(Pairs),
}

impl Sum {
    pub fn handle(sum: Self) {
        match sum {
            Sum::Pairs(pairs) => {
                pairs.run();
            }
        }
    }
}
