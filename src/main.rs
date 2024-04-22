use clap::{arg, command, Parser};
use mp4_merge::join_files;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long)]
    pub source: String,
}

fn get_files_from_source(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    Ok(file_names)
}

// fn merge_files(files: &Vec<String>) -> Result<>:
//     join_files(files, "output.mp4", |progress| {
//         println!("Merging... {:.2}%", progress*100.0);
//     }).unwrap();
//     Ok()

fn main() {
    let args = Arguments::parse();
    let source_dir = args.source;
    let file_names = get_files_from_source(&source_dir).unwrap();
    println!("Files: {:?}", file_names);
}
