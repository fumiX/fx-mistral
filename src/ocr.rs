use crate::MistralClient;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

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
    pages: Vec<Page>,
    model: String,
    usage_info: UsageInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    index: u32,
    markdown: String,
    images: Vec<Image>,
    dimensions: Dimensions,
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    id: String,
    top_left_x: u32,
    top_left_y: u32,
    bottom_right_x: u32,
    bottom_right_y: u32,
    image_base64: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dimensions {
    dpi: u32,
    height: u32,
    width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct UsageInfo {
    pages_processed: u32,
    doc_size_bytes: u64,
}

#[derive(Debug)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse invoice information")
    }
}

impl Error for ParseError {}

impl<'a> OcrClient<'a> {
    pub fn new(mistral_client: &'a MistralClient, model: &str) -> Self {
        OcrClient {
            mistral_client,
            ocr_path: "ocr".to_string(),
            model: model.to_string()
        }
    }

    // Add methods for OCR operations here
    pub async fn get_ocr_results(&self, signed_url: &str) -> Result<OcrResponse, Box<dyn Error>> {
        let ocr_request = OcrRequest {
            model: self.model.clone(),
            document: Document {
                document_type: "document_url".to_string(),
                document_url: signed_url.to_string(),
            },
            include_image_base64: false,
        };

        // Serialize the request body to JSON for debugging
        let request_body = serde_json::to_string(&ocr_request)?;
        println!("OCR Request body: {}", request_body);

        // Build the request
        let request = self
            .mistral_client
            .client
            .post(&format!("{}/{}", self.mistral_client.base_url, self.ocr_path))
            .bearer_auth(&self.mistral_client.api_key)
            .json(&ocr_request)
            .build()?;

        // Send the request
        let response = self.mistral_client.client.execute(request).await?;

        println!("Response status: {}", response.status());

        let response_text = response.text().await?;
        println!("Response body: {}", response_text);

        let ocr_response: OcrResponse = serde_json::from_str(&response_text)?;
        Ok(ocr_response)
    }

}