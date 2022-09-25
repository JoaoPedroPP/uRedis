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
enum PayloadType {
    JSON(serde_json::Value),
    Simple(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GETResponse {
    code: u32,
    msg: String,
    payload: Option<PayloadType>
}

impl Default for Response {
    fn default() -> Response {
        Response { code: 200, msg: "Success".to_string() }
    }
}

impl GETResponse {
    pub fn resp_200(redis_result: serde_json::Value) -> GETResponse {
        GETResponse { code: 200, msg: "Success".to_string(), payload: Some(PayloadType::JSON(redis_result)) }
    }

    pub fn resp_200_string(redis_result: String) -> GETResponse {
        GETResponse { code: 200, msg: "Success".to_string(), payload: Some(PayloadType::Simple(redis_result)) }
    }

    pub fn no_payload(msg: String) -> GETResponse {
        GETResponse { code: 200, msg: msg, payload: None }
    }

    pub fn error() -> GETResponse {
        GETResponse { code: 500, msg: "Internal Server Error".to_string(), payload: None }
    }
}