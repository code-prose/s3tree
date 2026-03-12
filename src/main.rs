use clap::Parser;
use std::io;
use aws_sdk_s3 as s3;

struct Root {
    children: Vec<Box<Directory>>,
    name: String
}

enum Parent {
    Directory(Box<Directory>),
    Parent(Root),
}

struct Directory {
    children: Vec<Box<Directory>>,
    parent: Parent,
    name: String
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    bucket: String,

    // Do I even care about making this non-interactive?
    #[arg(short, long, default_value_t = false)]
    interactive: bool
}

// What am I going to use this for?
// Do I really need this or am I constructing this for the sake of using language features?
// If I do use it, do I need to create a impl from_string?
#[derive(Parser, Debug)]
enum Commands {
    Copy,
    Move,
    List, // is it possible to add additional flags like -l?
    ChangeDirectory,
    Tree,
}

async fn arg_loop(client: &aws_sdk_s3::Client, bucket: &str, root: Root) -> Result<(), s3::Error> {
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
    let directories: Root = create_directories(&args.bucket);
    // TODO:
    // Will need to handle some type of exceptions here... Might want to handle in the arg loop
    let result = arg_loop(&client, &args.bucket, directories).await;

    todo!("Need to get this to work with both LocalStack and normal AWS pathing");

    println!("{result:?}");
    todo!("Use this result to handle exceptions");

    Ok(())
}

fn create_directories(bucket: &str) -> Root {
    let children: Vec<Box<Directory>> = Vec::new();
    let root = Root{ children: children, name: bucket.to_string() };

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
