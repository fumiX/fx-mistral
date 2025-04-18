use std::env;
use dotenvy::dotenv;
use std::error::Error;
use serde::{Deserialize, Serialize};
use fx_mistral::chat::ChatResponse;
use fx_mistral::chat::chat_request::Messages;
use fx_mistral::MistralClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;
    let base_url = "https://api.mistral.ai/v1";
    let file_path = "./examples/Beleg1.pdf";

    let mistral_client = MistralClient::new(&api_key, base_url);

    // Open the file
    let data = tokio::fs::read(file_path).await?;
    println!("Read PDF data {} with size {} bytes.", file_path, data.len());

    // Upload the file
    let file_client = mistral_client.file_client();
    let file_data = file_client.upload_file(data).await?;
    println!("Uploaded file to mistral, received file data result: {:?}", file_data);

    let signed_url = file_client.get_signed_url(&file_data.id).await?;
    println!("Signed URL retrieved from mistral: {:?}", signed_url);

    // Create messages with the signed URL
    let ocr_client = mistral_client.ocr_client("mistral-ocr-latest");
    let ocr_response = ocr_client.get_ocr_results(signed_url.url.as_str()).await?;
    println!("OCR response: {:?}", ocr_response);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct InvoiceInfo {
    invoice_number: String,
    total_amount: String,
    currency: String,
    invoice_date: String,
    issuer_name: String,
}

fn extract_invoice_info(completion: &str) -> Result<InvoiceInfo, Box<dyn Error>> {
    // Remove the backticks and code block indicator
    let json_str = completion.trim().trim_start_matches("```json").trim_end_matches("```").trim();

    // Deserialize the JSON string
    let invoice_info: InvoiceInfo = serde_json::from_str(json_str)?;
    Ok(invoice_info)
}