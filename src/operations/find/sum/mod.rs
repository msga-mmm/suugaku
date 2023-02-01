pub mod pairs;

use clap::Subcommand;
pub use pairs::model::Pairs;

#[derive(Subcommand, Debug)]
pub enum Sum {
    /// Find pairs of numbers whose sum is equal to a given target.
    Pairs(Pairs),
}

impl Sum {
    pub fn run(&self) {
        match self {
            Self::Pairs(pairs) => {
                pairs.run();
            }
        }
    }
}
