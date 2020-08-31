mod dtos;
mod mailgun;

pub use dtos::{Attachments, EmailParams, Events, ReceiveMessage};
pub use mailgun::MailungApi;
