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
pub enum PayloadType {
    JSON(serde_json::Value),
    Simple(String),
    No
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GETResponse {
    code: u32,
    msg: String,
    payload: PayloadType
}

impl Default for Response {
    fn default() -> Response {
        Response { code: 200, msg: "Success".to_string() }
    }
}

impl GETResponse {
    pub fn resp_200(redis_result: serde_json::Value) -> GETResponse {
        GETResponse { code: 200, msg: "Success".to_string(), payload: PayloadType::JSON(redis_result) }
    }

    pub fn resp_200_string(redis_result: String) -> GETResponse {
        GETResponse { code: 200, msg: "Success".to_string(), payload: PayloadType::Simple(redis_result) }
    }

    pub fn no_payload(msg: String) -> GETResponse {
        GETResponse { code: 200, msg: msg, payload: PayloadType::No }
    }

    pub fn error() -> GETResponse {
        GETResponse { code: 500, msg: "Internal Server Error".to_string(), payload: PayloadType::No }
    }
}