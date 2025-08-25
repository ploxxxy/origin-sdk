use serde::{Deserialize, Serialize};

use crate::protocol::message::{EventBody, RequestBody, ResponseBody};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "LSX")]
pub struct Lsx {
    #[serde(rename = "$value")]
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Message {
    Request(Request),
    Response(Response),
    Event(Event),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@recipient")]
    pub recipient: String,
    #[serde(rename = "$value")]
    pub body: RequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@sender")]
    pub sender: String,
    #[serde(rename = "$value")]
    pub body: ResponseBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "@sender")]
    pub sender: String,
    #[serde(rename = "$value")]
    pub body: EventBody,
}
