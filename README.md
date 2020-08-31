# Mailgun API

<div align="center">
    <p>
        <a href="https://crates.io/crates/mailgun_api" target="_blank"><img alt="Crate Info" src="https://img.shields.io/badge/Crate-Mailgun%20Api%20-yellow"/></a>
        <a href="https://docs.rs/mailgun_api" target="_blank"><img alt="API Docs" src="https://img.shields.io/badge/Docs-Mailgun%20Api%20-blue"/></a>
        <a href="https://discord.gg/ZHWmUaf" target="_blank"><img alt="Discord Chat" src="https://img.shields.io/badge/Discor-Spielrs%20-yellowgreen"/></a>
        <a href="https://github.com/spielrs/mailgun_api/blob/master/LICENSE" target="_blank"><img alt="Donate by Paypal" src="https://img.shields.io/badge/License-MIT%20-lightgrey"/></a>
        <a href="https://paypal.me/dancespiele?locale.x=en_US" target="_blank"><img alt="Donate by Paypal" src="https://img.shields.io/badge/Donate-PayPal-green.svg"/></a>
    </p>
</div>

Send emails through mailgun in rust asynchronously

## How install it

1. add the dependency in the Cargo.toml file of the project:

```toml
reqwest = {version = "0.10", features = ["json"]}
serde = { version = "1.0", features = ["derive"]}
serde_derive = "1.0"
serde_json = "1.0"
tokio = { version = "0.2", features = ["macros"]}
```

## Example

```rust
use mailgun_api::MailgunApi;
use mailgun_api::api::EmailParams;
use dotenv::dotenv;
use std::env;

// Inside of you tokio implementation
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

let mut mailgun = MailgunApi::new(&mailgun_secret, &mailgun_endpoint, &mailgun_domain);

let response = mailgun.send_email::<HashMap<String, String>>(params).await;

assert_eq!(response.is_ok(), true)
```

## How test it
1. Create a .env file and add the next parameters:

```
MAILGUN_DOMAIN=[YOUR DOMAIN]
MAILGUN_SECRET=[YOUR MAILGUN API SECRET]
MAILGUN_ENDPOINT=[MAILGUN ENDPOINT] // There is two endpoint currently: api.mailgun.net and api.eu.mailgun.net
EMAIL_RECEIVER_TEST=[EMAIL RECEIVER TEST]
EMAIL_FROM=[SENDER TEST]
```

2. Execute the test `cargo test`

**Warning:** when you are running the tests you are sending a email to the receiver that you set

## About Dtos

The dtos are structures used to transform the json retrieved from Mailung API in data accesible by the rust aplication.
It wasn't possible to see any Mailgun documentation which specify which parameters are nullable then maybe is possible
to get an error in the response because the Dto property is not typed as `Option` (please open an issue if that happens),
for this reason the methods accept Generics which you need to pass the Dtos provide or yours 

## Do you like Mailgun API?
If you like Mailgun API, help us supporting the project:
- [Github Sponsors](https://github.com/sponsors/dancespiele)
- [Paypal](https://paypal.me/dancespiele?locale.x=en_US)

## Roadmap

- [x] Send email
- [x] Retrieve Store Message
- [ ] Domains
- [ ] IPs
- [x] Events
- [ ] Stats
- [ ] Tags
- [ ] Suppressions
- [ ] Routes
- [ ] Webhooks
- [ ] Mailing List
- [ ] Templates
- [ ] Email validation
- [ ] Inbox Placement

## License

Mailgun API is [MIT](LICENSE) licensed