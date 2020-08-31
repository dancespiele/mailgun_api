//! # Mailgun API
//!
//! Send emails through mailgun in rust
//!
//! ## How install it
//! 1. add the dependency in the Cargo.toml file of the project:
//!
//! ```toml
//! mailgun_api = "0.2"
//! reqwest = {version = "0.10", features = ["json"]}
//! serde = { version = "1.0", features = ["derive"]}
//! serde_derive = "1.0"
//! serde_json = "1.0"
//! tokio = { version = "0.2", features = ["macros"]}
//! ```
//!
//! ## Example
//! ```rust
//! #[tokio::test]
//! async fn shoul_send_email_with_text() {
//!     use super::EmailParams;
//!     use dotenv::dotenv;
//!     use std::env;
//!
//!     dotenv().ok();
//!
//!     let mailgun_secret = env::var("MAILGUN_SECRET").expect("MAILGUN_SECRET must be set");
//!     let mailgun_domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set");
//!     let mailgun_endpoint = env::var("MAILGUN_ENDPOINT").expect("MAILGUN_ENDPOINT must be set");
//!     let sender = env::var("EMAIL_FROM").expect("MAIL_FROM must be set");
//!     let receiver = env::var("EMAIL_RECEIVER_TEST").expect("EMAIL_RECEIVER_TEST must be set");
//!
//!     let params = EmailParams {
//!         from: sender,
//!         to: receiver,
//!         subject: "test mailgung api".to_string(),
//!         text: Some("hello this is a test".to_string()),
//!         html: None,
//!     };
//!
//!     let mut mailgun = MailungApi::new(&mailgun_secret, &mailgun_endpoint, &mailgun_domain);
//!
//!     let response = mailgun.send_email::<HashMap<String, String>>(params).await;
//!
//!     assert_eq!(response.is_ok(), true)
//! }
//! ```
//! ## How test it
//!
//! 1. Create a .env file and add the next parameters:
//!
//! ```ignore
//! MAILGUN_DOMAIN=[YOUR DOMAIN]
//! MAILGUN_SECRET=[YOUR MAILGUN API SECRET]
//! MAILGUN_ENDPOINT=[MAILGUN ENDPOINT] // There is two endpoint currently: api.mailgun.net and api.eu.mailgun.net
//! EMAIL_RECEIVER_TEST=[EMAIL RECEIVER TEST]
//! EMAIL_FROM=[SENDER TEST]
//! ```
//!
//! 2. Execute the tests `cargo test`
//!
//! **Warning:** when you are running the tests you are sending a email to the receiver that you set
//!
//! ## About Dtos
//!
//! The dtos are structures used to transform the json retrieved from Mailung API in data accesible by the rust aplication.
//! It wasn't possible to see any Mailgun documentation which specify which parameters are nullable then maybe is possible
//! to get an error in the response because the Dto property is not typed as `Option` (please open an issue if that happens),
//! for this reason the methods accept Generics which you need to pass the Dtos provided or yours

pub mod api;

pub use api::MailgunApi;
