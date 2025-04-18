pub mod chat_request;

use crate::chat::chat_request::ChatRequest;
use crate::{MistralApiError, MistralClient, MistralError};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::{info, trace};

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
    temperature: f32,
}

impl<'a> ChatClient<'a> {
    pub fn new(mistral_client: &'a MistralClient, model: &str, temperature: f32) -> Self {
        ChatClient {
            mistral_client,
            chat_path: "chat/completions".to_string(),
            model: model.to_string(),
            temperature,
        }
    }

    pub fn request_builder<S: Into<String>>(&self, system_prompt: S) -> chat_request::ChatRequestBuilder {
        chat_request::ChatRequestBuilder::new(self.model.clone(), system_prompt.into(), self.temperature)
            .temperature(self.temperature)
    }

    pub async fn chat_complete(&self, request: &ChatRequest) -> Result<ChatResponse, MistralError> {
        info!("Chat request to {:?}", request.model);
        trace!("Request: {}", serde_json::to_string_pretty(request).unwrap_or("Can't serialize request".to_string()));

        let response = self
            .mistral_client
            .client
            .post(&format!("{}/{}", self.mistral_client.base_url, self.chat_path))
            .bearer_auth(&self.mistral_client.api_key)
            .json(request)
            .send()
            .await
            .map_err(MistralError::Network)?;

        let status = response.status();
        let text = response.text().await.map_err(MistralError::Network)?;
        trace!("Response: {}", text);

        if !status.is_success() {
            // Try to parse API error JSON
            let api_error: Result<MistralApiError, _> = serde_json::from_str(&text);
            return match api_error {
                Ok(err) => Err(MistralError::Api(err)),
                Err(_) => Err(MistralError::Http(status)),
            };
        }

        // Try to parse success response
        serde_json::from_str(&text).map_err(MistralError::Parse)
    }

    pub async fn chat_complete_struct<T> (
        &self,
        request: &ChatRequest,
    ) -> Result<T, MistralError>
    where
        T: DeserializeOwned,
    {
        let response = self.chat_complete(request).await?;
        let content = response.choices[0].message.content.clone();
        extract_struct_from_chat_response::<T>(&content)
    }

}


pub fn extract_struct_from_chat_response<T>(content: &str) -> Result<T, MistralError>
where
    T: DeserializeOwned,
{
    let json_str = content
        .trim()
        .trim_start_matches("```json")
        .trim_end_matches("```")
        .trim();

    serde_json::from_str(json_str).map_err(MistralError::Parse)
}