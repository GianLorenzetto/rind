use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn print_folder_contents(path: &Path) {
    let contents = fs::read_dir(path).expect("Unable to read path");
    for c in contents {
        let item = c.unwrap().path();
        match item.is_file() {
            true => print_file(&item),
            _ => match item.is_dir() {
                true => print_folder_contents(&item),
                _ => (),
            },
        };
    }
}

fn print_file(path: &Path) {
    let size: i64 = match path.is_file() {
        true => path.metadata().unwrap().len() as i64,
        _ => -1,
    };
    println!("F: {} ({} bytes)", path.display(), size);
}

fn main() {
    let arg1 = env::args().nth(1).expect("Usage: fview PATH");
    let path = Path::new(&arg1);
    match path.is_dir() {
        true => print_folder_contents(&path),
        _ => print_file(&path),
    };
}
