pub mod sum;

use clap::Subcommand;
use sum::Sum;

#[derive(Subcommand, Debug)]
pub enum Find {
    /// Find sums that meet a requirement.
    #[clap(subcommand)]
    Sum(Sum),
}

impl Find {
    pub fn run(&self) {
        match self {
            Self::Sum(sum) => sum.run(),
        }
    }
}
