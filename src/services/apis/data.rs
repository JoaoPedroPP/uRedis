use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    key: String,
    payload: Option<serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    code: u32,
    msg: String
}

impl Default for Response {
    fn default() -> Response {
        Response { code: 200, msg: "Success".to_string() }
    }
}