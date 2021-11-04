mod exit_code;
mod file_info;
mod folder_info;

use crate::folder_info::FolderInfo;
use exit_code::ErrorCondition;
use std::env;
use std::path::PathBuf;

fn print_usage() {
    println!("Usage: rind <path>");
}

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
                Err(code) => return Err(code.into()),
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
    let path_arg = match env::args().nth(1) {
        Some(path) => path,
        _ => String::new(),
    };

    let path = PathBuf::from(&path_arg);
    if !path.is_dir() {
        print_usage();
        return Err(ErrorCondition::invalid_argument().into());
    }

    match process_folders(path) {
        Err(code) => Err(code.into()),
        Ok(folders) => {
            for fi in folders.iter() {
                print_folder(fi);
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::exit_code::ErrorCondition;
    use crate::process_folders;
    use std::path::PathBuf;

    #[test]
    fn given_folder_when_process_then_contents_returned() {
        let path_str = "../";
        let path = PathBuf::from(&path_str);
        assert!(process_folders(path).is_ok());
    }
}
