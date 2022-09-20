use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    payload: String
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