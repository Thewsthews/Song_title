#! usr/bin/env cargo-script

use id3::{Tag, TagLike};

use std::env;
use std::path::Path;

fn main()-> Result<(), Box<dyn std::error::Error>>{
    //This part gets the file path frm the command-line argument
let args: Vec<String> = env::args().collect();
if args.len() != 2 {
    eprintln!("Usage: {} <path to mp3 file>", args[0]);
    std::process::exit(1);
}
let file_path = &args[1];
let path = Path::new(file_path);

//Checks if the target file exists
if !path.exists(){eprintln!("File does not exist: {}", file_path);
    std::process::exit(1);}


//This part reads the ID3 tag 
let tag = Tag::read_from_path(path)?;

//The song title is gotten and the feedback is displayed

match tag.title() {
        Some(title) => println!("Song Title: {}", title),
        None => println!("No song title found in the file"),
    }
Ok(())
}