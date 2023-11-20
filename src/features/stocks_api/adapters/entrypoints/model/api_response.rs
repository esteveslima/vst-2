use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct APIResponse<'a, T> {
    message: Option<&'a str>,
    result: Option<T>,
}

impl<'a, T> APIResponse<'a, T> {
    pub fn success(result: T) -> APIResponse<'a, T> {
        APIResponse {
            message: None,
            result: Some(result),
        }
    }

    pub fn error(message: &'a str) -> APIResponse<'a, T> {
        APIResponse {
            message: Some(message),
            result: None,
        }
    }

    // pub fn new(result: T, message: &'a str) -> APIResponse<'a, T> {
    //     APIResponse {
    //         message: Some(message),
    //         result: Some(result),
    //     }
    // }
}
