use reqwest::Client;
use crate::chat::ChatClient;
use crate::ocr::OcrClient;

mod ocr;
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
