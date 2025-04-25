use reqwest::Client;
use crate::chat::ChatClient;
use crate::ocr::OcrClient;
use std::fmt;
use std::error::Error;
use serde::Deserialize;

pub mod ocr;
pub mod chat;
pub mod files;

pub struct MistralClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl MistralClient {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        MistralClient {
            client: Client::new(),
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn file_client(&self) -> files::FileClient {
        files::FileClient::new(self)
    }

    pub fn chat_client(&self, model: &str, temperature: f32) -> ChatClient {
        ChatClient::new(self, model, temperature)
    }

    pub fn ocr_client(&self, model: &str) -> OcrClient {
        OcrClient::new(&self, model)
    }
}

#[derive(Debug)]
pub enum MistralError {
    Api(MistralApiError),
    Http(reqwest::StatusCode),
    Network(reqwest::Error),
    Parse(serde_json::Error),
}

impl Error for MistralError {}
impl fmt::Display for MistralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MistralError::Api(err) => write!(
                f,
                "Mistral API error (code {}): {}",
                err.code, err.message
            ),
            MistralError::Http(code) => write!(f, "Unexpected HTTP status: {}", code),
            MistralError::Network(err) => write!(f, "Network error: {}", err),
            MistralError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MistralApiError {
    pub code: u32,
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
    pub object: String,
    pub param: Option<String>,
    #[serde(skip_deserializing)]
    pub description: String,
}

pub(crate) fn error_description(code: u32) -> &'static str {
    match code {
        2210 => "Invalid request filter: The JSON body could not be parsed.",
        401 => "Unauthorized: Check your API key.",
        422 => "Unprocessable Entity: The request was well-formed but unable to be followed due to semantic errors.",
        _ => "An unknown error occurred.",
    }
}