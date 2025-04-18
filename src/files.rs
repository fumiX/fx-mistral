use reqwest::multipart;
use serde::{Deserialize, Serialize};

use crate::{MistralClient, MistralError, MistralApiError};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub id: String,
    object: String,
    bytes: u64,
    created_at: u64,
    filename: String,
    purpose: String,
    sample_type: String,
    num_lines: Option<u64>,
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedUrl {
    pub url: String,
}

pub struct FileClient<'a> {
    mistral_client: &'a MistralClient,
    files_path: String,
}

impl<'a> FileClient<'a> {
    pub fn new(mistral_client: &'a MistralClient) -> Self {
        FileClient {
            mistral_client,
            files_path: "files".to_string(),
        }
    }

    pub async fn upload_file(&self, file_data: Vec<u8>) -> Result<FileData, MistralError> {
        let part = multipart::Part::bytes(file_data).file_name("uploaded_file.pdf");
        let form = multipart::Form::new()
            .text("purpose", "ocr")
            .part("file", part);

        let response = self
            .mistral_client
            .client
            .post(&format!("{}/{}", self.mistral_client.base_url, self.files_path))
            .bearer_auth(&self.mistral_client.api_key)
            .multipart(form)
            .send()
            .await
            .map_err(MistralError::Network)?;

        let status = response.status();
        let response_text = response.text().await.map_err(MistralError::Network)?;

        if !status.is_success() {
            return match serde_json::from_str::<MistralApiError>(&response_text) {
                Ok(mut err) => {
                    err.description = crate::error_description(err.code).to_string();
                    Err(MistralError::Api(err))
                },
                Err(_) => Err(MistralError::Http(status)),
            };
        }

        serde_json::from_str(&response_text).map_err(MistralError::Parse)
    }

    pub async fn get_signed_url(&self, file_id: &str) -> Result<SignedUrl, MistralError> {
        let call_url = format!(
            "{}/{}/{}/url?expiry=24",
            self.mistral_client.base_url, self.files_path, file_id
        );

        let response = self
            .mistral_client
            .client
            .get(call_url)
            .bearer_auth(&self.mistral_client.api_key)
            .send()
            .await
            .map_err(MistralError::Network)?;

        let status = response.status();
        let response_text = response.text().await.map_err(MistralError::Network)?;

        if !status.is_success() {
            return match serde_json::from_str::<MistralApiError>(&response_text) {
                Ok(mut err) => {
                    err.description = crate::error_description(err.code).to_string();
                    Err(MistralError::Api(err))
                },
                Err(_) => Err(MistralError::Http(status)),
            };
        }

        serde_json::from_str(&response_text).map_err(MistralError::Parse)
    }
}
