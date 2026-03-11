use clap::Parser;
use std::io;
use std::process::Command;
use aws_sdk_s3 as s3;

// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// use ratatui::{
//     buffer::Buffer,
//     layout::Rect,
//     style::Stylize,
//     symbols::border,
//     text::{Line, Text},
//     widgets::{Block, Paragraph, Widget},
//     DefaultTerminal, Frame,
// };
//
struct Root {
    children: Vec<Box<Directory>>
}

enum Parent {
    Directory(Box<Directory>),
    Parent(Root),
}

struct Directory {
    children: Vec<Box<Directory>>,
    parent: Parent,
}



// #[derive(Debug, Default)]
// pub struct App {
//     curr_dir: String,
//     exit: bool
// }
//
// impl App {
//     pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
//         while !self.exit {
//             terminal.draw(|frame| self.draw(frame))?;
//             self.handle_events()?;
//         }
//         Ok(())
//     }
//
//     fn draw(&self, frame: &mut Frame) {
//         frame.render_widget(self, frame.area());
//     }
//
//     fn handle_events(&mut self) -> io::Result<()> {
//         match event::read()? {
//             // it's important to check that the event is a key press event as
//             // crossterm also emits key release and repeat events on Windows.
//             Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
//                 self.handle_key_event(key_event)
//             }
//             _ => {}
//         };
//         Ok(())
//     }
//
//     fn handle_key_event(&mut self, key_event: KeyEvent) {
//         match key_event.code {
//             KeyCode::Char('q') => self.exit(),
//             KeyCode::Char('j') => todo!(),
//             KeyCode::Char('k') => todo!(),
//             KeyCode::Enter => {
//                 todo!()
//                 // do I have to handle something here regarding what is hovered?
//             }
//             // I want to handle j - k - up -down for navigation between buckets
//             // I also want to handle enter... this could depend on whether the person is hovering
//             // ../ or a directory... or even an item... Could I make it so a person could read?
//             // Would I do this by doing aws s3 cp into some buffer and then displaying it? Then
//             // what if the person wants to make changes? Could I also enable this by writing the
//             // buffer back out to s3?
//             _ => {}
//         }
//     }
//
//     fn exit(&mut self) {
//         self.exit = true;
//     }
//
//     fn current_directory(&mut self, new: String) {
//         self.curr_dir = new;
//     }
// }
//
// impl Widget for &App {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let title = Line::from(" Counter App Tutorial ".bold());
//         let instructions = Line::from(vec![
//             " Down ".into(),
//             "<j / down>".blue().bold(),
//             " Up ".into(),
//             "<k / up>".blue().bold(),
//             " Enter directory ".into(),
//             "<Enter>".blue().bold(),
//             " Quit ".into(),
//             "<Q> ".blue().bold(),
//         ]);
//         // let block = Block::bordered()
//         //     .title(title.centered())
//         //     .title_bottom(instructions.centered())
//         //     .border_set(border::THICK);
//         //
//         // let counter_text = Text::from(vec![Line::from(vec![
//         //     "Value: ".into(),
//         //     self.counter.to_string().yellow(),
//         // ])]);
//         //
//         // Paragraph::new(counter_text)
//         //     .centered()
//         //     .block(block)
//         //     .render(area, buf);
//     }
// }
//
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

async fn arg_loop(client: &aws_sdk_s3::Client, bucket: &str) -> Result<(), s3::Error> {
    loop {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Failed to parse command");
        let cmd_vec: Vec<_> = cmd.split_whitespace().collect(); 
        match cmd_vec[0] {
            "exit" => break,
            "ls" => {
                list_bucket(client, bucket).await?;
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
    Ok(())
}

#[::tokio::main]
async fn main() -> Result<(), s3::Error> {
    let args = Args::parse();
    println!("{args:?}");
    let config = aws_config::load_from_env().await;
    let force_path_style = std::env::var("AWS_ENDPOINT_URL").is_ok();
    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .force_path_style(force_path_style)
        .build();
    let client = aws_sdk_s3::Client::from_conf(s3_config);
    // TODO:
    // Will need to handle some type of exceptions here... Might want to handle in the arg loop
    let result = arg_loop(&client, &args.bucket).await;

    todo!("Need to get this to work with both LocalStack and normal AWS pathing");
    Ok(())
}

fn create_directories() -> Root {
    let children: Vec<Box<Directory>> = Vec::new();
    let root = Root{ children: children };

    root
}



async fn list_bucket(
    client: &aws_sdk_s3::Client,
    bucket: &str,
) -> Result<(), s3::Error> {
    // List the buckets in this account
    let mut objects = client
        .list_objects_v2()
        .bucket(bucket)
        .into_paginator()
        .send();

    println!("key\tetag\tlast_modified\tstorage_class");
    while let Some(result) = objects.next().await {
        match result {
            Ok(object) => {
                for item in object.contents() {
                    println!(
                        "{}\t{}\t{}\t{}",
                        item.key().unwrap_or_default(),
                        item.e_tag().unwrap_or_default(),
                        item.last_modified()
                            .map(|lm| format!("{lm}"))
                            .unwrap_or_default(),
                        item.storage_class()
                            .map(|sc| format!("{sc}"))
                            .unwrap_or_default()
                    );
                }
            },
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
    // Prepare a ByteStream around the file, and upload the object using that ByteStream.
    // let body = aws_sdk_s3::primitives::ByteStream::from_path(filepath)
    //     .await
    //     .map_err(|err| {
    //         S3ExampleError::new(format!(
    //             "Failed to create bytestream for {filepath:?} ({err:?})"
    //         ))
    //     })?;
    // let resp = client
    //     .put_object()
    //     .bucket(bucket)
    //     .key(key)
    //     .body(body)
    //     .send()
    //     .await?;


    // Retrieve the just-uploaded object.
    // let resp = client.get_object().bucket(bucket).key(key).send().await?;
    // println!("etag: {}", resp.e_tag().unwrap_or("(missing)"));
    // println!("version: {}", resp.version_id().unwrap_or("(missing)"));

    Ok(())
}

// fn main() -> io::Result<()> {
//     ratatui::run(|terminal| App::default().run(terminal))
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ratatui::style::Style;
//
//     #[test]
//     fn render() {
//         let app = App::default();
//         let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
//
//         app.render(buf.area, &mut buf);
//
//         let mut expected = Buffer::with_lines(vec![
//             "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
//             "┃                    Value: 0                    ┃",
//             "┃                                                ┃",
//             "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
//         ]);
//         let title_style = Style::new().bold();
//         let counter_style = Style::new().yellow();
//         let key_style = Style::new().blue().bold();
//         expected.set_style(Rect::new(14, 0, 22, 1), title_style);
//         expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
//         expected.set_style(Rect::new(13, 3, 6, 1), key_style);
//         expected.set_style(Rect::new(30, 3, 7, 1), key_style);
//         expected.set_style(Rect::new(43, 3, 4, 1), key_style);
//
//         assert_eq!(buf, expected);
//     }
//
//     #[test]
//     fn handle_key_event() {
//         let mut app = App::default();
//         app.handle_key_event(KeyCode::Right.into());
//         assert_eq!(app.counter, 1);
//
//         app.handle_key_event(KeyCode::Left.into());
//         assert_eq!(app.counter, 0);
//
//         let mut app = App::default();
//         app.handle_key_event(KeyCode::Char('q').into());
//         assert!(app.exit);
//     }
// }
