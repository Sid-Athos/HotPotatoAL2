pub trait MessageInterface {
    /// Create a message from the specific input
    fn new(buf: Vec<u8>) -> Self;

    // return the string JSON format of this object
    fn to_json(&self) -> String;

    // Create message based on JSON format
    fn from_str(str: &String) -> Self;
}

