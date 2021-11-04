use crate::exit_code::ErrorCondition;
use crate::file_info::FileInfo;

use std::fs;
use std::path::PathBuf;

pub struct FolderInfo {
    pub path: PathBuf,
    pub files: Vec<FileInfo>,
    pub folders: Vec<PathBuf>,
}

impl FolderInfo {
    pub fn create(path: PathBuf) -> Result<Self, ErrorCondition> {
        if !path.is_dir() {
            return Err(ErrorCondition::invalid_path());
        }

        let mut files: Vec<FileInfo> = vec![];
        let mut folders: Vec<PathBuf> = vec![];
        match fs::read_dir(&path) {
            Ok(entries) => {
                for result in entries {
                    match result {
                        Ok(e) => {
                            let ep = e.path();
                            if ep.is_file() {
                                if let Ok(fi) = FileInfo::create(ep) {
                                    files.push(fi);
                                }
                            } else if ep.is_dir() {
                                folders.push(ep)
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => return Err(ErrorCondition::list_directory()),
        }

        Ok(Self {
            path,
            files,
            folders,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::exit_code::ErrorCondition;
    use crate::folder_info::FolderInfo;
    use std::path::PathBuf;

    #[test]
    fn given_folder_when_create_then_returns_contents() {
        let path_str = "../";
        let path = PathBuf::from(&path_str);
        let fi = FolderInfo::create(path).unwrap();
        assert!(fi.path.display().to_string().contains(&path_str));
        assert!(fi.files.len() > 0);
        assert!(fi.folders.len() > 0);
    }

    #[test]
    fn given_folder_with_only_files_when_create_then_returns_contents() {
        let path_str = "./src/";
        let path = PathBuf::from(&path_str);
        let fi = FolderInfo::create(path).unwrap();
        assert!(fi.path.display().to_string().contains(&path_str));
        assert!(fi.files.len() > 0);
        assert_eq!(fi.folders.len(), 0);
    }
}
