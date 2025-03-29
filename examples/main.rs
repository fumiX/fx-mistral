use std::env;
use dotenvy::dotenv;
use std::error::Error;
use serde::{Deserialize, Serialize};
use fx_mistral::chat::ChatResponse;
use fx_mistral::chat::messages::Messages;
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
    let system_message = "Du bist ein KI-Assistent mit Dokumentenverständnis für Rechnungen und Belege über URLs. Du erhältst URLs von an Dich hochgeladenen Beleg-Dokumenten. Extrahiere die erforderlichen Informationen mit Hilfe von OCR und gib sie im folgenden JSON-Format zurück. Währungsbeträge mit Dezimalkomma und Datum im Iso-Format: {\"invoice_number\": \"\", \"total_amount\": \"\", \"tax\": \"\", \"currency\": \"\", \"invoice_date\": \"\", \"issuer_name\": \"\"}".to_string();
    let user_message = "Extrahiere die folgenden Daten als Felder: * Rechnungsnummer: Rechnungsnummer, Belegnummer. * Gesamtbetrag/Summe: normalerweise als Summe, Total, Brutto, Zahlbetrag bezeichnet, der den tatsächlich in Rechnung gestellten Betrag einschließlich Steuern enthält. * Steuer: Mehrwertsteuer oder VAT als Betrag in der Währung. * Währung: als 'EUR' oder 'USD'. * Rechnungsdatum. Nur das Datum, ohne Uhrzeit. * Ausstellername: Das Unternehmen, das die Rechnung ausstellt. Das Rechnungsdokument wird als URL bereitgestellt. Beispiel einer Rechnung:\n\nRechnungsnummer: 12345\nSumme: €100,00\nRechnungsdatum: 18.02.2025\nAusstellername: Beispiel GmbH".to_string();
    let messages = Messages::builder(&system_message)
        .add_document_message(&user_message, &signed_url.url)
        .build();

    for message in &messages.messages {
        println!("Message: {:?}", message);
    }

    // Call the chat_complete function
    let chat_client = mistral_client.chat_client("mistral-small-latest", 0.0);
    let chat_response: ChatResponse = chat_client
        .chat_complete(messages)
        .await?;

    println!("Chat response: {:?}", chat_response);

    let completion = chat_response.choices[0].message.content.clone();
    let invoice_info = extract_invoice_info(&completion)?;
    println!("Invoice Info: {:?}", invoice_info);

    // let ocr_results = mistral_client.get_ocr_results(&signed_url.url).await?;
    // println!("OCR Results: {:?}", ocr_results);
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