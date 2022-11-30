pub mod message_traits {
    use crate::messages::messages_types_structs::message_structs::Message;

    pub trait MessageBehaviourTrait {
        fn new_from_buffer(buf: Vec<u8>) -> Self;

        fn new_from_string(str: &str) -> Self;

        fn display_item_type<T>(_item: T);

        fn display_message(message: &Message);

        fn to_json(&self) -> String;

        fn from_str(str: &str) -> Self;
    }
}