mod args;
use args::Args;

fn main() {
    let arguments = Args::new();

    println!("{:?}", arguments);
}
