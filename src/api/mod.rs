mod dtos;
mod mailgun;

pub use dtos::{
    Attachments, DeliveryStatus, EmailParams, Envelope, Events, Flags, Headers, Item, Message,
    Paging, ReceiveMessage, Storage, Tag,
};
pub use mailgun::MailgunApi;
