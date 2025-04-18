use serde::{Deserialize, Serialize};

//
// Chat Request structs
//

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String, // should be "json_schema"
    pub json_schema: JsonSchemaFormat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSchemaFormat {
    pub schema: serde_json::Value, // Accept raw JSON schema as a serde_json::Value
    pub name: String,
    pub strict: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    Simple {
        role: String,
        content: String, // For system messages
    },
    WithContentArray {
        role: String,
        content: Vec<Content>, // For user messages with docs/texts
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Content {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "document_url")]
    DocumentUrl { document_url: String },
}

pub struct Messages {
    pub messages: Vec<Message>,
}

//
// Chat Request Builder
//

pub struct ChatRequestBuilder {
    model: String,
    messages: Vec<Message>,
    response_format: Option<ResponseFormat>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

impl ChatRequestBuilder {
    pub fn new<S: Into<String>>(model: S, system_message: S, temparatur: f32) -> Self {
        ChatRequestBuilder {
            model: model.into(),
            messages: vec![Message::Simple {
                role: "system".to_string(),
                content: system_message.into(),
            }],
            response_format: None,
            max_tokens: None,
            temperature: Some(temparatur),
        }
    }

    pub fn add_user_message<S: Into<String>>(mut self, text: S) -> Self {
        self.messages.push(Message::WithContentArray {
            role: "user".to_string(),
            content: vec![Content::Text { text: text.into() }],
        });
        self
    }

    pub fn add_document_message<S: Into<String>>(mut self, text: S, document_url: S) -> Self {
        self.messages.push(Message::WithContentArray {
            role: "user".to_string(),
            content: vec![
                Content::Text { text: text.into() },
                Content::DocumentUrl {
                    document_url: document_url.into(),
                },
            ],
        });
        self
    }

    pub fn response_format_from_json<S: Into<String>>(mut self, schema_json: S, name: S, strict: bool) -> Self {
        let schema_value: serde_json::Value = serde_json::from_str(&schema_json.into()).expect("Invalid JSON schema");
        self.response_format = Some(ResponseFormat {
            format_type: "json_schema".to_string(),
            json_schema: JsonSchemaFormat {
                schema: schema_value,
                name: name.into(),
                strict,
            },
        });
        self
    }

    pub fn max_tokens(mut self, value: u32) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn temperature(mut self, value: f32) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn build(self) -> ChatRequest {
        ChatRequest {
            model: self.model,
            messages: self.messages,
            response_format: self.response_format,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
        }
    }
}