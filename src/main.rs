use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    bucket: String,

    // Do I even care about making this non-interactive?
    #[arg(short, long, default_value_t = false)]
    interactive: bool
}

#[derive(Parser, Debug)]
enum Commands {
    Copy,
    Move,
    List, // is it possible to add additional flags like -l?
    ChangeDirectory,
    Tree,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
