use clap::Parser;
use std::io;

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

    loop {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Failed to parse command");
        let cmd_vec: Vec<_> = cmd.split_whitespace().collect(); 
        match cmd_vec[0] {
            "exit" => break,
            _ => println!("{cmd_vec:?}")
            
        }
    }
}
