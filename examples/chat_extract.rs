use std::env;
use dotenvy::dotenv;
use std::error::Error;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use fx_mistral::chat::ChatResponse;
use fx_mistral::chat::chat_request::{ChatRequestBuilder, Messages};
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
    let system_message = r#"Du bist ein KI-Assistent mit Dokumentenverständnis für Rechnungen und Belege über URLs.
        Du erhältst URLs von an Dich hochgeladenen Beleg-Dokumenten. Extrahiere die erforderlichen Informationen
        mit Hilfe von OCR und gib sie im JSON-Format zurück wie im JSON-Schema definiert."#.to_string();

    let user_message = r#"Extrahiere die Daten aus dem Dokument in das invoice_info."#.to_string();

    let schema = schema_for!(InvoiceInfo);
    let schema_str = serde_json::to_string_pretty(&schema).unwrap();

    let chat_client = mistral_client.chat_client("mistral-small-latest", 0.0);
    let request = chat_client.request_builder(system_message.clone())
        .add_document_message(user_message, signed_url.url)
        .response_format_from_json(schema_str, "invoice_info".into(), false)
        .build();



    // Call the chat_complete function
    let chat_response: ChatResponse = chat_client
        .chat_complete(&request)
        .await?;

    println!("Chat response: {:?}", chat_response);

    let completion = chat_response.choices[0].message.content.clone();
    let invoice_info = extract_invoice_info(&completion)?;
    println!("Invoice Info: {:?}", invoice_info);

    Ok(())
}

fn example_invoice_date() -> String { "2025-02-18".to_string() }

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct InvoiceInfo {
    #[schemars(description = "Rechnungsnummer, Belegnummer")]
    invoice_number: String,
    #[schemars(description = "Normalerweise als Summe, Total, Brutto, Zahlbetrag bezeichnet, der den tatsächlich  in Rechnung gestellten Betrag einschließlich Steuern enthält.")]
    total_amount: String,
    #[schemars(description = "Mehrwertsteuer oder VAT als Betrag in der Währung.")]
    vat: String,
    #[schemars(description = "Internationale Kennung der Währung, z.B. 'EUR', 'USD', 'CHF', 'GBP', etc.")]
    currency: String,
    #[schemars(description = "Das Rechnungsdatum. Nur das Datum, ohne Uhrzeit, im ISO-Format YYYY-MM-DD.", example = "example_invoice_date")]
    invoice_date: String,
    #[schemars(description = "Das Unternehmen, die Organisation oder die Person, das die Rechnung ausstellt.")]
    issuer_name: String,
}


fn extract_invoice_info(completion: &str) -> Result<InvoiceInfo, Box<dyn Error>> {
    // Remove the backticks and code block indicator
    let json_str = completion.trim().trim_start_matches("```json").trim_end_matches("```").trim();

    // Deserialize the JSON string
    let invoice_info: InvoiceInfo = serde_json::from_str(json_str)?;
    Ok(invoice_info)
}