use crate::{MistralApiError, MistralClient, MistralError};

use serde::{Deserialize, Serialize};

pub struct OcrClient<'a> {
    mistral_client: &'a MistralClient,
    ocr_path: String,
    model: String,
}

// OCR Request structs

#[derive(Serialize, Deserialize, Debug)]
struct OcrRequest {
    model: String,
    document: Document,
    include_image_base64: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Document {
    #[serde(rename = "type")]
    document_type: String,
    document_url: String,
}

// OCR Response structs

#[derive(Serialize, Deserialize, Debug)]
pub struct OcrResponse {
    pub pages: Vec<Page>,
    pub model: String,
    pub usage_info: UsageInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub index: u32,
    pub markdown: String,
    pub images: Vec<Image>,
    pub dimensions: Dimensions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: String,
    pub top_left_x: u32,
    pub top_left_y: u32,
    pub bottom_right_x: u32,
    pub bottom_right_y: u32,
    pub image_base64: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dimensions {
    pub dpi: u32,
    pub height: u32,
    pub width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsageInfo {
    pub pages_processed: u32,
    pub doc_size_bytes: u64,
}

impl<'a> OcrClient<'a> {
    pub fn new(mistral_client: &'a MistralClient, model: &str) -> Self {
        OcrClient {
            mistral_client,
            ocr_path: "ocr".to_string(),
            model: model.to_string()
        }
    }


    pub async fn get_ocr_results(&self, signed_url: &str) -> Result<OcrResponse, MistralError> {
        let ocr_request = OcrRequest {
            model: self.model.clone(),
            document: Document {
                document_type: "document_url".to_string(),
                document_url: signed_url.to_string(),
            },
            include_image_base64: false,
        };

        let response = self
            .mistral_client
            .client
            .post(&format!("{}/{}", self.mistral_client.base_url, self.ocr_path))
            .bearer_auth(&self.mistral_client.api_key)
            .json(&ocr_request)
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

        serde_json::from_str::<OcrResponse>(&response_text)
            .map_err(MistralError::Parse)
    }
}

