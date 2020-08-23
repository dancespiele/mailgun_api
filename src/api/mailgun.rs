use super::EmailParams;
use reqwest::Error;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// MailungApi struct where include all the endpoints
pub struct MailungApi {
    /// Mailung api secret
    api_key: String,
    /// There is two endpoint currently: api.mailgun.net and api.eu.mailgun.net
    endpoint: String,
    /// Your email domain
    domain: String,
}

impl MailungApi {
    /// Set the mailung paramenters
    pub fn new(api_key: &str, endpoint: &str, domain: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            endpoint: endpoint.to_string(),
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
