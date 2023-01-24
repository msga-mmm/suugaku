mod implementation;

use std::fs;

use clap::Parser;
use implementation::find_sum;

/// A struct to find the pairs of numbers whose sum is equal to a given target.
#[derive(Parser, Debug)]
pub struct Pairs {
    /// List of numbers where to find the pairs of numbers.
    #[clap(long, value_delimiter = ',')]
    from: Vec<i32>,

    /// File with a list of numbers where to find the pairs of numbers.
    ///
    /// The list of numbers must be a single line and each number is separate
    /// by a comma.
    ///
    /// This argument conflicts with `from` argument, you should use one of
    /// them but not both.
    #[clap(long, conflicts_with = "from")]
    from_file: Option<String>,

    /// The sum of each pair of numbers must equal this target number.
    #[clap(long)]
    target: i32,
}

struct ParsedInput {
    from: Vec<i32>,
    target: i32,
}

impl Pairs {
    fn parse(&self) -> ParsedInput {
        let numbers = if let Some(file) = &self.from_file {
            let numbers = fs::read_to_string(file).expect("file should exist");
            let numbers: Vec<i32> = numbers
                .trim()
                .split(',')
                .map(|ch| {
                    ch.parse().unwrap_or_else(|_| {
                        panic!(
                            "each number should be a valid number, found: {ch}"
                        )
                    })
                })
                .collect();

            numbers
        } else {
            self.from.clone()
        };

        ParsedInput {
            from: numbers,
            target: self.target,
        }
    }

    pub fn run(&self) {
        let input = self.parse();
        println!("from: {:?}", &input.from);
        println!("target: {:?}", input.target);
        let result = find_sum(&input.from, input.target);
        let result: Vec<(i32, i32)> = result
            .iter()
            .map(|unordered_pair| unordered_pair.into_ordered_tuple())
            .collect();
        println!("result: {result:?}");
    }
}
