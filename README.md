# Mailgun API

Send emails through mailgun in rust

## How install it

1. add the dependency in the Cargo.toml file of the project:

```toml
mailgun_api = "0.1"
```

## Example

```rust
#[tokio::test]
async fn shoul_send_email_with_text() {
    use std::collections::HashMap;
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

## Do you like Mailgun API?
* If you like Mailgun API, help us supporting the project with [Github Sponsors](https://github.com/sponsors/dancespiele) or with grants in [Gitcoin](https://gitcoin.co/grants/1078/mailgun-api)

## Roadmap

- [x] Send email
- [x] Retrieve Store Message
- [x] Delete Stored Message
- [ ] Domains
- [ ] IPs
- [ ] Events
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