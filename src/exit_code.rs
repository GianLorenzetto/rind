#[derive(Debug, PartialEq)]
pub struct ErrorCondition {
    code: i32,
}

impl ErrorCondition {
    pub fn invalid_path() -> Self {
        Self { code: 1 }
    }
    pub fn invalid_argument() -> Self {
        Self { code: 2 }
    }
    pub fn file_meta_data() -> Self {
        Self { code: 3 }
    }
    pub fn list_directory() -> Self {
        Self { code: 4 }
    }
}

impl Into<i32> for ErrorCondition {
    fn into(self) -> i32 {
        self.code
    }
}
