pub mod messages;

use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::chat::messages::Messages;
use crate::MistralClient;


//
// Chat Response structs.
//

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    index: u32,
    pub message: MessageContent,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContent {
    role: String,
    tool_calls: Option<serde_json::Value>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    prompt_tokens: u32,
    total_tokens: u32,
    completion_tokens: u32,
}




pub struct ChatClient<'a> {
    mistral_client: &'a MistralClient,
    chat_path: String,
    model: String,
    temperature: f64,
}

impl<'a> ChatClient<'a> {
    pub fn new(mistral_client: &'a MistralClient, model: &str, temperature: f64) -> Self {
        ChatClient {
            mistral_client,
            chat_path: "chat/completions".to_string(),
            model: model.to_string(),
            temperature,
        }
    }


    pub async fn chat_complete(&self, messages: Messages) -> Result<ChatResponse, Box<dyn Error>> {
        let request_body = serde_json::json!({
            "model": self.model,
            "messages": messages.messages,
            "temperature": self.temperature,
        });

        let response = self
            .mistral_client
            .client
            .post(&format!("{}/{}", self.mistral_client.base_url, self.chat_path))
            .bearer_auth(&self.mistral_client.api_key)
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        // println!("Response body: {}", response_text);

        let chat_response: ChatResponse = serde_json::from_str(&response_text)?;
        Ok(chat_response)
    }
}