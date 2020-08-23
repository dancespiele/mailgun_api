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
