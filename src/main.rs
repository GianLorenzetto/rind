mod error_condition;
mod file_info;
mod folder_info;

use crate::folder_info::FolderInfo;
use clap::{App, Arg};
use error_condition::ErrorCondition;
use std::path::PathBuf;
use std::time::Instant;

fn process_folders(path: PathBuf) -> Result<Vec<FolderInfo>, ErrorCondition> {
    let mut folders = vec![];
    let mut to_process: Vec<PathBuf> = vec![path];
    loop {
        if let Some(next) = to_process.pop() {
            match FolderInfo::create(next) {
                Ok(fi) => {
                    for folder in fi.folders.iter() {
                        to_process.push(folder.clone());
                    }
                    folders.push(fi);
                }
                Err(ec) => return Err(ec),
            }
        } else {
            break;
        }
    }

    Ok(folders)
}

fn print_folder(fi: &FolderInfo) {
    println!("** Folder path {}", &fi.path.display());
    for folder in fi.folders.iter() {
        println!("  + {}", &folder.display());
    }
    for file_info in fi.files.iter() {
        println!("  - {}", file_info);
    }
}

fn main() -> Result<(), i32> {
    let args = App::new("rind")
        .version("0.1")
        .about("Rust version of find(-lite) command")
        .arg(
            Arg::with_name("path")
                .help("The top-level path to search")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .takes_value(false)
                .help("Do not print results to stdout"),
        )
        .get_matches();

    let path_arg = args.value_of("path").unwrap();
    let is_quiet = args.is_present("quiet");

    let path = PathBuf::from(&path_arg);
    if !path.is_dir() {
        let ec = ErrorCondition::invalid_argument("path");
        eprintln!("{}", ec);
        return Err(ec.into());
    }

    let start = Instant::now();
    match process_folders(path) {
        Err(ec) => {
            eprintln!("{}", ec);
            Err(ec.into())
        }
        Ok(folders) => {
            println!("Processed folder in {}ms", start.elapsed().as_millis());
            for fi in folders.iter() {
                if !is_quiet {
                    print_folder(fi);
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error_condition::ErrorCondition;
    use crate::process_folders;
    use std::path::PathBuf;

    #[test]
    fn given_folder_when_process_then_contents_returned() {
        let path_str = "../";
        let path = PathBuf::from(&path_str);
        assert!(process_folders(path).is_ok());
    }
}
