pub mod message_structs {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub message: String,
    }
    pub struct OtherMessage {
        pub message: String,
        pub some_other_value: u32
    }
    pub enum MessageEnum {
        BaseMessage(Message),
        OtherMessage(OtherMessage, u32, String)
    }
}
