use super::{EmailParams, Events, ReceiveMessage};
use reqwest::Error;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// MailungApi struct where include all the endpoints
pub struct MailungApi {
    /// Mailung api secret
    api_key: String,
    /// There is two endpoint currently: api.mailgun.net and api.eu.mailgun.net
    endpoint: String,
    /// Storage endpoint
    storage_endpoint: String,
    /// Your email domain
    domain: String,
}

impl MailungApi {
    /// Set the mailung paramenters
    pub fn new(api_key: &str, endpoint: &str, domain: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            endpoint: endpoint.to_string(),
            storage_endpoint: endpoint.to_string().replace("api", "storage"),
            domain: domain.to_string(),
        }
    }

    /// Send an email
    pub async fn send_email<T>(&mut self, email_params: EmailParams) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let mut form = HashMap::new();
        form.insert("from", email_params.from);
        form.insert("to", email_params.to);
        form.insert("subject", email_params.subject);

        if let Some(html_message) = email_params.html {
            form.insert("html", html_message);
        } else if let Some(text_message) = email_params.text {
            form.insert("text", text_message);
        }

        let response = reqwest::Client::new()
            .post(&format!(
                "https://api:{}@{}/v3/{}/messages",
                self.api_key, self.endpoint, self.domain
            ))
            .form(&form)
            .send()
            .await?
            .json::<T>()
            .await;

        response
    }

    /// Get all the events
    pub async fn get_all_events<T>(&mut self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .get(&format!(
                "https://api:{}@{}/v3/{}/events",
                self.api_key, self.endpoint, self.domain
            ))
            .send()
            .await?
            .json::<T>()
            .await;

        response
    }

    /// Get a message content by Id
    pub async fn get_message_by_id<T>(&mut self, message_id: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .get(&format!(
                "https://api:{}@{}/v3/domains/{}/messages/{}",
                self.api_key, self.storage_endpoint, self.domain, message_id
            ))
            .send()
            .await?
            .json::<T>()
            .await;

        response
    }
}

#[tokio::test]
async fn shoul_send_email_with_text() {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let mailgun_secret = env::var("MAILGUN_SECRET").expect("MAILGUN_SECRET must be set");
    let mailgun_domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set");
    let mailgun_endpoint = env::var("MAILGUN_ENDPOINT").expect("MAILGUN_ENDPOINT must be set");
    let sender = env::var("EMAIL_FROM").expect("MAIL_FROM must be set");
    let receiver = env::var("EMAIL_RECEIVER_TEST").expect("EMAIL_RECEIVER_TEST must be set");

    let params = EmailParams {
        from: sender,
        to: receiver,
        subject: "test mailgung api".to_string(),
        text: Some("hello this is a test".to_string()),
        html: None,
    };

    let mut mailgun = MailungApi::new(&mailgun_secret, &mailgun_endpoint, &mailgun_domain);

    let response = mailgun.send_email::<HashMap<String, String>>(params).await;

    assert_eq!(response.is_ok(), true)
}

#[tokio::test]
async fn should_get_message_by_id() {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let mailgun_secret = env::var("MAILGUN_SECRET").expect("MAILGUN_SECRET must be set");
    let mailgun_domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set");
    let mailgun_endpoint = env::var("MAILGUN_ENDPOINT").expect("MAILGUN_ENDPOINT must be set");

    let mut mailgun = MailungApi::new(&mailgun_secret, &mailgun_endpoint, &mailgun_domain);

    let response = mailgun
        .get_message_by_id::<ReceiveMessage>("id")
        .await
        .unwrap();

    println!("Response: {:#?}", response);
}

#[tokio::test]
async fn should_all_the_events() {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let mailgun_secret = env::var("MAILGUN_SECRET").expect("MAILGUN_SECRET must be set");
    let mailgun_domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set");
    let mailgun_endpoint = env::var("MAILGUN_ENDPOINT").expect("MAILGUN_ENDPOINT must be set");

    let mut mailgun = MailungApi::new(&mailgun_secret, &mailgun_endpoint, &mailgun_domain);

    let response = mailgun.get_all_events::<Events>().await.unwrap();

    println!("Response: {:#?}", response);
}
