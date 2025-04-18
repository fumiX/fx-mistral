use dotenvy::dotenv;
use fx_mistral::MistralClient;
use std::env;
use std::error::Error;

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
