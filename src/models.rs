use std::error::Error;
use std::fmt;

pub const API_ERROR_NOT_FOUND_CODE: u32 = 5;
pub const API_ERROR_ALREADY_EXISTS_CODE: u32 = 6;
pub const API_ERROR_UNAUTHENTICATED_CODE: u32 = 16;

#[derive(Debug)]
pub struct ApiError {
    pub code: u32,
    pub message: String,
    pub error: Option<Box<dyn Error>>,
}

unsafe impl Send for ApiError {}
unsafe impl Sync for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl ApiError {
    pub fn not_found(message: String) -> Self {
        ApiError::new(API_ERROR_NOT_FOUND_CODE, message, None)
    }

    pub fn already_exist(message: String) -> Self {
        ApiError::new(API_ERROR_ALREADY_EXISTS_CODE, message, None)
    }

    pub fn unauthenticated(message: String) -> Self {
        ApiError::new(API_ERROR_UNAUTHENTICATED_CODE, message, None)
    }

    pub fn new(code: u32, message: String, error: Option<Box<dyn Error>>) -> Self {
        Self {
            code,
            message,
            error,
        }
    }
}

impl ApiError {
    pub fn get_error_message(&self) -> String {
        String::from(&self.message)
    }

    pub fn get_error_code(&self) -> u32 {
        self.code
    }
}
