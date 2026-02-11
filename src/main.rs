use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    bucket: String,

    #[arg(short, long, default_value_t = false)]
    interactive: bool
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
