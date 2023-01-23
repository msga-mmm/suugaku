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
    pub fn handle(find: Self) {
        match find {
            Find::Sum(sum) => {
                Sum::handle(sum);
            }
        }
    }
}
