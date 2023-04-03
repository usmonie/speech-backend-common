use std::error::Error;
use std::fmt;

pub mod domain;
pub mod data;

pub const API_ERROR_NOT_FOUND_CODE: u32 = 5;
pub const API_ERROR_ALREADY_EXISTS_CODE: u32 = 6;
pub const API_ERROR_UNAUTHENTICATED_CODE: u32 = 16;

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

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
///
/// See the [module documentation](self) for details.
pub enum ApiResult<R> {
    /// Contains the success value
    Ok(R),

    /// Contains the error value
    Err(ApiError),
}

unsafe impl<R> Send for ApiResult<R> {}

unsafe impl<R> Sync for ApiResult<R> {}

impl<R> ApiResult<R> {
    pub fn is_ok(&self) -> bool {
        matches!(&self, ApiResult::Ok(_))
    }
}