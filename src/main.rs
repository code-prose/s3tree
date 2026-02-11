use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    bucket: String,
    interactive: bool
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
