use clap::Parser;
use suugaku::operations::Operation;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    operation: Operation,
}

fn main() {
    let args = Args::parse();
    args.operation.run();
}
