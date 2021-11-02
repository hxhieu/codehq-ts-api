use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub error: Option<String>,
    pub data: Option<T>,
}
