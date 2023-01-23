mod implementation;

use clap::Parser;

/// A struct to find the pairs of numbers whose sum is equal to a given target.
#[derive(Parser, Debug)]
pub struct Pairs {
    /// List of numbers where to find the pairs of numbers.
    #[clap(long, value_delimiter = ',')]
    from: Vec<i32>,

    /// The sum of each pair of numbers must equal this target number.
    #[clap(long)]
    target: i32,
}

use implementation::find_sum;

impl Pairs {
    pub fn run(&self) {
        println!("from: {:?}", &self.from);
        println!("target: {:?}", self.target);
        let result = find_sum(&self.from, self.target);
        let result: Vec<(i32, i32)> = result
            .iter()
            .map(|unordered_pair| unordered_pair.into_ordered_tuple())
            .collect();
        println!("result: {result:?}");
    }
}
