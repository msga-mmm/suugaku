use clap::Parser;

/// A struct to find the pairs of numbers whose sum is equal to a given target.
#[derive(Parser, Debug)]
pub struct Pairs {
    /// List of numbers where to find the pairs of numbers.
    #[clap(long, value_delimiter = ',')]
    pub(super) from: Vec<i32>,

    /// File with a list of numbers where to find the pairs of numbers.
    ///
    /// The list of numbers must be a single line and each number is separate
    /// by a comma.
    ///
    /// This argument conflicts with `from` argument, you should use one of
    /// them but not both.
    #[clap(long, conflicts_with = "from")]
    pub(super) from_file: Option<String>,

    /// The sum of each pair of numbers must equal this target number.
    #[clap(long)]
    pub(super) target: i32,
}
