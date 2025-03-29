use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::MistralClient;

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
            files_path: "files".to_string()
        }
    }

    pub async fn upload_file(&self, file_data: Vec<u8>) -> Result<FileData, Box<dyn Error>> {
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
            .await?;

        let response_text = response.text().await?;
        // println!("Response body: {}", response_text);

        let file_data: FileData = serde_json::from_str(&response_text)?;
        Ok(file_data)
    }

    pub async fn get_signed_url(&self, file_id: &str) -> Result<SignedUrl, Box<dyn Error>> {
        let call_url = format!("{}/{}/{}/url?expiry=24", self.mistral_client.base_url, self.files_path, file_id);
        // println!("Calling {}", call_url);
        let response = self
            .mistral_client
            .client
            .get(call_url)
            .bearer_auth(&self.mistral_client.api_key)
            .send()
            .await?;

        let response_text = response.text().await?;
        // println!("Response body for url call: {}", response_text);

        let signed_url: SignedUrl = serde_json::from_str(&response_text)?;
        Ok(signed_url)
    }
}
