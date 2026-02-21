use clap::Parser;
use std::io;
use std::process::Command;
use ratatui::{DefaultTerminal, Frame};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    bucket: String,

    // Do I even care about making this non-interactive?
    #[arg(short, long, default_value_t = false)]
    interactive: bool
}

// What am I going to use this for?
#[derive(Parser, Debug)]
enum Commands {
    Copy,
    Move,
    List, // is it possible to add additional flags like -l?
    ChangeDirectory,
    Tree,
}

fn arg_loop() {
    loop {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Failed to parse command");
        let cmd_vec: Vec<_> = cmd.split_whitespace().collect(); 
        match cmd_vec[0] {
            "exit" => break,
            "ls" => {
                // -a? -l?
                let res = Command::new("ls").spawn();
                println!("{res:?}");
            },
            "cd" => {
                // cd foo/bar/?
                println!("change dir!");
            },
            "mv" => {
                println!("move!");
            },
            "rm" => {
                // do I really want to take something like -rf?
                println!("remove!");
            }
            "cp" => {
                // how can I differentiate what is s3 and what is local?
                println!("copy!");
            }
            _ => println!("{cmd_vec:?}")
            
        }
    }
}

// fn main() {
//     let args = Args::parse();
//     println!("{args:?}");
// }

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
