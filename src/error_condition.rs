use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct ErrorCondition {
    code: i32,
    desc: String,
    details: String,
}

impl ErrorCondition {
    pub fn invalid_path(path: &PathBuf) -> Self {
        Self {
            code: 1,
            desc: String::from("Path specified is invalid"),
            details: ErrorCondition::to_string_details(path),
        }
    }
    pub fn invalid_argument(details: &str) -> Self {
        Self {
            code: 2,
            desc: String::from("Specified argument is invalid"),
            details: details.to_string(),
        }
    }
    pub fn file_meta_data(path: &PathBuf) -> Self {
        Self {
            code: 3,
            desc: String::from("Unable to read file metadata"),
            details: ErrorCondition::to_string_details(path),
        }
    }
    pub fn list_directory(path: &PathBuf) -> Self {
        Self {
            code: 4,
            desc: String::from("Unable to list directory contents"),
            details: ErrorCondition::to_string_details(path),
        }
    }

    fn to_string_details(path: &PathBuf) -> String {
        match path.to_str() {
            Some(p) => p.to_string(),
            _ => String::from("Unknown"),
        }
    }
}

impl Into<i32> for ErrorCondition {
    fn into(self) -> i32 {
        self.code
    }
}

impl fmt::Display for ErrorCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error ({}) - {}: '{}'",
            self.code, self.desc, self.details
        )
    }
}
