pub enum MessageEnum {
    BaseMessage(Message),
    OtherMessage(OtherMessage, u32, String)
}