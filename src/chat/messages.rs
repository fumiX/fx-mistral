use serde::{Deserialize, Serialize};

//
// Chat Request Messages
//

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    content: Vec<Content>,
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


impl Messages {
    pub fn builder<S: Into<String>>(system_message: S) -> MessagesBuilder {
        MessagesBuilder {
            messages: vec![Message {
                role: "system".to_string(),
                content: vec![Content::Text {
                    text: system_message.into(),
                }],
            }],
        }
    }
}

pub struct MessagesBuilder {
    messages: Vec<Message>,
}

impl MessagesBuilder {
    pub fn add_document_message<S: Into<String>>(mut self, text: S, document_url: S) -> Self {        self.messages.push(Message {
            role: "user".to_string(),
            content: vec![
                Content::Text {
                    text: text.into(),
                },
                Content::DocumentUrl {
                    document_url: document_url.into(),
                },
            ],
        });
        self
    }

    pub fn build(self) -> Messages {
        Messages {
            messages: self.messages,
        }
    }
}
