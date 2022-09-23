use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub key: String,
    payload: Option<serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    code: u32,
    msg: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GETResponse {
    code: u32,
    msg: String,
    payload: Option<serde_json::Value>
}

impl Default for Response {
    fn default() -> Response {
        Response { code: 200, msg: "Success".to_string() }
    }
}

impl GETResponse {
    pub fn resp_200(redis_result: serde_json::Value) -> GETResponse {
        GETResponse { code: 200, msg: "Success".to_string(), payload: Some(redis_result) }
    }

    pub fn no_payload() -> GETResponse {
        GETResponse { code: 200, msg: "Success".to_string(), payload: None }
    }
}