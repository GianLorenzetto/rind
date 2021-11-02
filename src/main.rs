use std::env;
use std::fs;
use std::path::Path;

enum ReturnCode {
    Error = 1,
}

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

fn print_usage() {
    println!("Usage: rind <path>");
}

fn main() -> Result<(), i32> {
    let path_arg = match env::args().nth(1) {
        Some(path) => path,
        _ => String::new(),
    };

    let path = Path::new(&path_arg);
    match path.is_dir() {
        true => {
            print_folder_contents(&path);
            Ok(())
        }
        _ => {
            print_usage();
            Err(ReturnCode::Error as i32)
        }
    }
}
