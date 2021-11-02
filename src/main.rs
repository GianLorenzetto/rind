use std::env;
use std::fs;
use std::path::Path;

enum ReturnCode {
    InvalidArgumentError = 1,
    MetaDataError = 2,
}

struct FileInfo {
    full_path: String,
    size: usize,
}

impl FileInfo {
    fn create(path: &Path) -> Result<Self, ReturnCode> {
        let size = path.metadata().unwrap().len() as usize;
        Ok(Self {
            full_path: path.display().to_string(),
            size,
        })
    }
}

fn collect_files(path: &Path) -> Result<Vec<FileInfo>, ReturnCode> {
    let contents = fs::read_dir(path).expect("Unable to read path");
    let mut files: Vec<FileInfo> = vec![];
    for c in contents {
        let item = c.unwrap().path();
        match item.is_file() {
            true => {
                files.push(FileInfo::create(&item)?);
            }
            _ => match item.is_dir() {
                true => match collect_files(&item) {
                    Ok(mut fs) => files.append(&mut fs),
                    _ => (),
                },
                _ => (),
            },
        };
    }
    Ok(files)
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
            if let Ok(files) = collect_files(&path) {
                println!("File count: {}", files.len());

                for f in files {
                    println!("{} ({})", f.full_path, f.size);
                }
            }

            Ok(())
        }
        _ => {
            print_usage();
            Err(ReturnCode::InvalidArgumentError as i32)
        }
    }
}
