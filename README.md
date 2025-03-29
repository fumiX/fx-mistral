# fx-mistral

The `fx-mistral` library leverages the Mistral API to perform Optical Character Recognition (OCR) and extract structured 
data from PDF documents. It uses chat completions to process and interpret the extracted text. The library includes 
functionality to upload PDF files, retrieve signed URLs, and interact with the Mistral chat API to extract specific 
information from documents.

**This is just a play project to evaluate the Mistral API. It is not intended for production use.**

## Features

- Upload PDF files to the Mistral API
- Retrieve signed URLs for uploaded files
- Extract structured data from documents using chat completions
- Example for extracting data from German invoices

## Example

The following example demonstrates how to use the `fx-mistral` library to extract fields like invoice number, 
total amount, tax, currency, invoice date, and issuer name from a German invoice.

It uses 'mistral-small-latest' as model for the chat completion, which leads to a correct result on the given example
invoice. The large model did not work well for this example.

It uses the following prompts for this. The prompts are in German to fit best to the example invoice, which is also
in German.

- System prompt:
  ```text
  Du bist ein KI-Assistent mit Dokumentenverständnis für Rechnungen und Belege über URLs. Du erhältst URLs von an 
  Dich hochgeladenen Beleg-Dokumenten. Extrahiere die erforderlichen Informationen mit Hilfe von OCR und gib sie im 
  folgenden JSON-Format zurück. Währungsbeträge mit Dezimalkomma und Datum im Iso-Format: 
  {
    "invoice_number": "", 
    "total_amount": "", 
    "tax": "", 
    "currency": "", 
    "invoice_date": "", 
    "issuer_name": ""
  }
  ```
  - User prompt (with URL to the uploaded PDF file):
    ```text
    Extrahiere die folgenden Daten als Felder: * Rechnungsnummer: Rechnungsnummer, Belegnummer. * Gesamtbetrag/Summe: 
    normalerweise als Summe, Total, Brutto, Zahlbetrag bezeichnet, der den tatsächlich in Rechnung gestellten Betrag 
    einschließlich Steuern enthält. 
    * Steuer: Mehrwertsteuer oder VAT als Betrag in der Währung. 
    * Währung: als 'EUR' oder 'USD'. 
    * Rechnungsdatum. Nur das Datum, ohne Uhrzeit. 
    * Ausstellername: Das Unternehmen, das die Rechnung ausstellt. Das Rechnungsdokument wird als URL bereitgestellt. 
    Das Rechnungsdokument wird als URL bereitgestellt. Beispiel einer Rechnung:
      Rechnungsnummer: 12345
      Summe: €100,00
      Rechnungsdatum: 18.02.2025
      Ausstellername: Beispiel GmbH
    ```

### .env

Since the app calls the Mistral API, you need to provide your API key in a `.env` file.

```dotenv
API_KEY=<your mistral api key>
```

