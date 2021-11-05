use crate::error_condition::ErrorCondition;
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

pub struct FileInfo {
    pub path: PathBuf,
    pub size: usize,
}

impl FileInfo {
    pub fn create(path: PathBuf) -> Result<Self, ErrorCondition> {
        if !path.exists() {
            return Err(ErrorCondition::invalid_path(&path));
        }

        match path.metadata() {
            Ok(md) => Ok(Self {
                path,
                size: md.len() as usize,
            }),
            _ => Err(ErrorCondition::file_meta_data(&path)),
        }
    }
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} bytes)", self.path.display(), self.size)
    }
}

#[cfg(test)]
mod tests {
    use crate::error_condition::ErrorCondition;
    use crate::file_info::FileInfo;
    use std::path::{Path, PathBuf};

    #[test]
    fn when_read_file_has_non_zero_size() {
        let path_str = "./src/file_info.rs";
        let path = PathBuf::from(&path_str);
        let fi = FileInfo::create(path).unwrap();
        assert!(fi.path.display().to_string().contains(&path_str));
        assert!(fi.size > 0);
    }

    #[test]
    fn given_invalid_path_when_create_then_returns_invalid_path() {
        let path_str = "./src/no_such_file.rs";
        let path = PathBuf::from(&path_str);
        let result = FileInfo::create(path);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ErrorCondition::invalid_path(&path));
    }
}
