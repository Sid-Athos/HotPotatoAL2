use crate::messages::messages_types_structs::message_structs::{Message, OtherMessage};

pub enum MessageEnum {
    BaseMessage(Message),
    OtherMessage(OtherMessage, u32, String)
}