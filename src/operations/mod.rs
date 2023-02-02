pub mod find;

use clap::Subcommand;
use find::Find;

#[derive(Subcommand, Debug)]
pub enum Operation {
    /// Set of operations to find values that meet a requirement.
    #[clap(subcommand)]
    Find(Find),
}

impl Operation {
    pub fn run(&self) {
        match self {
            Self::Find(find) => find.run(),
        }
    }
}
