//! Write a function that given a directory, recursively finds all files with a given file
//! extension in that directory and all sub-directories, and counts the number of lines
//! in the file and prints it to stdout.
use std::fs::File;
use std::io::{self, BufRead, Result as IoResult};
use std::{env, path::{Path, PathBuf}};

/// Given an absolute path to a filename, read the file and count the number of lines
fn read_lines(file_path: &Path) -> IoResult<usize> {
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines().count())
}

/// Recursively finds all files with a given file extension in a given directory
fn traverse_dir(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut paths = vec![];
    for entry in dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            paths.extend(traverse_dir(&path, ext));
        } else {
            match path.extension() {
                Some(extension) => {
                    if extension == ext {
                        paths.push(path);
                    }
                }
                None => {}
            }
        }
    }
    paths
}

/// Main entry point of the program
pub fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let file_path = Path::new(&args[1]);
            if file_path.is_dir() {
                println!("Not a file: {:?}", file_path);
                return;
            }

            match read_lines(file_path) {
                Ok(lines) => println!("File {:?} {}", file_path.file_name().unwrap(), lines),
                Err(e) => println!("Error: {}", e),
            }
        },
        3 => {
            // let dir_path = pathdiff::diff_paths(Path::new(&args[1]), cur_dir).unwrap();
            let dir_path = Path::new(&args[1]);

            println!("Looking for files with extension `{}` in directory `{}`", args[2], dir_path.display());

            if !dir_path.is_dir() {
                match dir_path.extension() {
                    Some(ext) => {
                        if ext.to_str().unwrap() == args[2] {
                            let count = read_lines(dir_path);
                            println!("Total number of lines: {}", count.unwrap());
                        }
                        println!("Not a valid extension");
                    }
                    None => {
                        println!("No file with extension {} found", args[2]);
                    }
                }
                return;
            }
            let extension = &args[2];

            let mut count = 0;
        
            for path in traverse_dir(&dir_path, extension) {
                let len = read_lines(path.as_path()).unwrap();
                count += len;
                println!("Reading file: {:?} {}", path.file_name().unwrap(), len);
            }
            println!("Total number of lines: {}", count);
        },
        _ => println!("Usage: \ncargo run <absolute_dir_path> <file_extension> \n OR \ncargo run <absolute_file_path> \n")
    }
} 