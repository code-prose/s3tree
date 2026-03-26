use aws_sdk_s3 as s3;
use clap::Parser;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::collections::{HashMap, HashSet};


// Sad to see this go
// struct Root {
//     children: Vec<Box<Directory>>,
//     name: String,
// }
// enum Parent {
//     Directory(Box<Directory>),
//     Parent(Root),
// }
//
// struct Directory {
//     children: Vec<Box<Directory>>,
//     parent: Parent,
//     name: String,
// }

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    bucket: String,

    // Do I even care about making this non-interactive?
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
}

type DirectoryTree = HashMap<String, HashSet<String>>;
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

async fn arg_loop(client: &aws_sdk_s3::Client, bucket: &str, tree: DirectoryTree) -> Result<(), s3::Error> {
    // this is not redundant - I'm just not modifying it yet
    let mut curr_path = bucket.clone();
    loop {
        print!("s3://{curr_path}> ");
        // should I handle this shit?
        let _ = stdout().flush();

        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to parse command");
        let cmd_vec: Vec<_> = cmd.split_whitespace().collect();
        println!("");
        match cmd_vec[0] {
            "exit" => break,
            "ls" => {
                if cmd_vec.len() == 2 {
                    if tree.contains_key(cmd_vec[1]) {
                        let dir_contents = tree.get(cmd_vec[1]).unwrap();
                        for entry in dir_contents {
                            println!("{entry}");
                        }
                    }
                } else if cmd_vec.len() == 1 {
                    let dir_contents = tree.get(curr_path).unwrap();
                    for entry in dir_contents {
                        println!("{entry}");
                    }
                } else {
                    println!("usage: ls [optional directory]");

                }
            }
            "cd" => {
                // cd foo/bar/?
                println!("change dir!");
            }
            "mv" => {
                println!("move!");
            }
            "rm" => {
                println!("remove!");
            }
            "lcp" => {
                // how can I differentiate what is s3 and what is local?
                if cmd_vec.len() == 1 {
                    println!("Usage:                           ");
                    println!("  lcp [fully qualified machine path]  [s3 dst path]:");
                }
                println!("local copy!");

            }
            "cp" => {
                if cmd_vec.len() == 1 {
                    println!("Usage:                           ");
                    println!("  cp [s3 src path] [s3 dst path]:");
                }
                println!("s3 copy!")

            }
            "help" => {
                println!("ls: lists all objects in current directory");
                println!("cd: changes to directory");
                println!("mv: moves an s3 object from src to dst");
                println!("rm: removes an s3 object");
                println!("lcp: copies an object from machine to s3 or s3 to machine");
                println!("cp: copies an s3 object from src to dst");

            }
            _ => println!("{cmd_vec:?}"),
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
    let directories = create_directories(&client, &args.bucket).await?;
    println!("{directories:?}");
    // TODO:
    // Will need to handle some type of exceptions here... Might want to handle in the arg loop
    let result = arg_loop(&client, &args.bucket, directories).await;

    todo!("Need to get this to work with both LocalStack and normal AWS pathing");

    println!("{result:?}");
    todo!("Use this result to handle exceptions");

    Ok(())
}

async fn create_directories(client: &aws_sdk_s3::Client, bucket: &str) -> Result<DirectoryTree, s3::Error> {
    let mut directory_tree = DirectoryTree::new();
    let keys = list_bucket(client, bucket).await?;
    for key in keys {
        let splits: Vec<String> = split_path(key);
        // there is a consideration here to make..
        // but given that "directories" don't really exist in s3 - it shouldn't cause any issues
        if splits.len() == 0 { continue; }
        let mut path = String::from(bucket);
        for i in 0..splits.len() {
            if directory_tree.contains_key(&path) {
                let dir_ref = directory_tree.get_mut(&path);
                let mut entry = splits[i].clone();
                if i < splits.len() - 1 {
                    entry.push('/');
                }
                (*dir_ref.unwrap()).insert(entry);
                let check = directory_tree.get(&path);
                println!("{check:?}");
            }
            else {
                let mut children: HashSet<String> = HashSet::new();
                children.insert(splits[i].clone());
                directory_tree.insert(path.clone(), children);
                println!("{directory_tree:?}");
            }
            println!("{i}: {}", splits[i]);
            path.push_str("/");
            path.push_str(&splits[i]);
            println!("appended: {path}");
        }
    }
    println!("{directory_tree:?}");
    Ok(directory_tree)
}

fn split_path(key: String) -> Vec<String>{
    key.split("/").map(|v| v.to_string()).collect::<Vec<_>>()
}

async fn list_bucket(client: &aws_sdk_s3::Client, bucket: &str) -> Result<Vec<String>, s3::Error> {
    // List the buckets in this account
    let mut objects = client
        .list_objects_v2()
        .bucket(bucket)
        .into_paginator()
        .send();

    println!("key\tetag\tlast_modified\tstorage_class");
    let mut keys: Vec<String> = Vec::new();
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
                    if let Some(key) = item.key() {
                        keys.push(key.to_string());
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
            }
        }
    }

    Ok(keys)
}

// async fn put_bucket(
//     client: &aws_sdk_s3::Client,
//     bucket: &str,
//     src: String,  // How the fuck am I going to get this? do I just want a local file path?
//     dst: String,
// ) -> Result<(), s3::Error> {
//     // Prepare a ByteStream around the file, and upload the object using that ByteStream.
//     let body = aws_sdk_s3::primitives::ByteStream::from_path(filepath)
//         .await
//         .map_err(|err| {
//             S3ExampleError::new(format!(
//                 "Failed to create bytestream for {filepath:?} ({err:?})"
//             ))
//         })?;
//     let resp = client
//         .put_object()
//         .bucket(bucket)
//         .key(dst)
//         .body(body)
//         .send()
//         .await?;
//     
//     // Retrieve the just-uploaded object.
//     let resp = client.get_object().bucket(bucket).key(key).send().await?;
//     println!("etag: {}", resp.e_tag().unwrap_or("(missing)"));
//     println!("version: {}", resp.version_id().unwrap_or("(missing)"));
//     Ok(())
// }
