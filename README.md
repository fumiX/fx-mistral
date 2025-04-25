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

The following example demonstrates how to use the `fx-mistral` library to extract structured data like invoice number, 
total amount, tax, currency, invoice date, and issuer name from German invoices.

It uses 'mistral-small-latest' as the model for chat completion, which leads to correct results with the example invoice. 
The large model did not work well for this example.

### System and User Prompts

The library uses both a system prompt and a user prompt for extraction:

- System prompt:
  ```text
  Du bist ein KI-Assistent mit Dokumentenverständnis für Rechnungen und Belege über URLs.
        Du erhältst URLs von an Dich hochgeladenen Beleg-Dokumenten. Extrahiere die erforderlichen Informationen
        mit Hilfe von OCR und gib sie im JSON-Format zurück wie im JSON-Schema definiert.
  ```

- User prompt:
  ```text
  Extrahiere die Daten aus dem Dokument in das invoice_info.
  ```

### JSON Schema for Structured Responses

A key feature of the library is the use of JSON Schema to enforce structured responses.
The library uses the response_format argument to send a JSON schema as string to 
mistral to instruct it to produce a chat response in the required format.

The library itself makes no assumption nor has dependencies to some JSON schema library, but the
chat_extract example shows the use with `schemars` to demonstrate how to use annotations to a rust struct
to produce a JSON schema with the instructions of how to fill the fields of the struct.

### .env

Since the app calls the Mistral API, you need to provide your API key in a `.env` file.

```dotenv
API_KEY=<your mistral api key>
```

