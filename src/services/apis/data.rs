use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub key: String,
    pub payload: Option<serde_json::Value>,
    pub ttl: Option<u32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub code: u32,
    msg: String
}

#[derive(Debug, Serialize, Deserialize)]
enum PayloadType {
    JSON(serde_json::Value),
    Simple(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GETResponse {
    pub code: u32,
    msg: String,
    payload: Option<PayloadType>
}

impl Default for Response {
    fn default() -> Response {
        Response { code: 201, msg: "Success".to_string() }
    }
}

impl Response {
    pub fn resp_generic(msg: String) -> Response {
        Response { code: 200, msg: msg }
    }

    pub fn resp_no_body() -> Response {
        Response { code: 204, msg: "No content to persist.".to_string() }
    }

    pub fn resp_no_ttl() -> Response {
        Response { code: 206, msg: "No ttl set.".to_string() }
    }

    pub fn error() -> Response {
        Response { code: 500, msg: "Internal Server Error".to_string() }
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