use serde::{Deserialize, Serialize};
/// Params to send a email
pub struct EmailParams {
    /// Sender
    pub from: String,
    /// Receiver
    pub to: String,
    /// Title of the email
    pub subject: String,
    /// Text body format of the email. If text is set html should be `None`
    pub text: Option<String>,
    /// Html body format of the email. If html is set text should be `None`
    pub html: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attachments {
    size: String,
    url: String,
    name: String,
    #[serde(rename = "content-type")]
    content_type: String,
}

/// Receive email
#[derive(Debug, Deserialize, Serialize)]
pub struct ReceiveMessage {
    #[serde(rename = "Received")]
    pub received: String,
    #[serde(rename = "stripped-signature")]
    pub stripped_signature: String,
    #[serde(rename = "Message-Id")]
    pub message_id: String,
    pub from: String,
    pub sender: String,
    pub recipients: String,
    #[serde(rename = "Subject")]
    pub subject_message: String,
    #[serde(rename = "Content-Transfer-Encoding")]
    pub content_transfer_encoding: String,
    #[serde(rename = "Sender")]
    pub email_sender: String,
    pub attachments: Vec<Attachments>,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "stripped-html")]
    pub stripped_html: String,
    #[serde(rename = "content-id-map")]
    pub content_id_map: serde_json::Value,
    #[serde(rename = "stripped-text")]
    pub stripped_text: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "From")]
    pub from_email: String,
    #[serde(rename = "message-headers")]
    pub message_headers: Vec<Vec<String>>,
    #[serde(rename = "Mime-Version")]
    pub mime_version: String,
    #[serde(rename = "Content-Type")]
    pub content_type: String,
    #[serde(rename = "body-plain")]
    pub body_plain: String,
    pub subject: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeliveryStatus {
    pub tls: bool,
    pub mx_host: String,
    pub attempt_no: u16,
    pub description: String,
    pub session_seconds: f32,
    pub utf8: bool,
    pub code: u16,
    pub message: String,
    pub certificate_verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    pub url: String,
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Envelope {
    pub transport: String,
    pub sender: String,
    pub sending_ip: Option<String>,
    pub targets: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Flags {
    pub is_routed: Option<bool>,
    pub is_authenticated: Option<bool>,
    pub is_system_test: Option<bool>,
    pub is_test_mode: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub domain: String,
    pub tag: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Headers {
    pub to: String,
    pub message_id: String,
    pub from: String,
    pub subject: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Message {
    pub headers: Headers,
    pub attachments: Option<Vec<Attachments>>,
    pub size: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Item {
    pub tags: Option<Vec<Tag>>,
    pub delivery_status: Option<DeliveryStatus>,
    pub storage: Storage,
    pub envelope: Envelope,
    pub log_level: String,
    pub event: String,
    pub campaigns: Option<serde_json::Value>,
    pub user_variables: serde_json::Value,
    pub flags: Flags,
    pub recipient_domain: String,
    pub timestamp: f32,
    pub message: Message,
    pub recipient: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paging {
    pub previous: String,
    pub first: String,
    pub last: String,
    pub next: String,
}

/// Get all the events
#[derive(Debug, Deserialize, Serialize)]
pub struct Events {
    pub items: Vec<Item>,
    pub paging: Paging,
}
