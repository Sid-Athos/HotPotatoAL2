use std::collections::HashMap;
use std::io::Read;
use std::net::TcpListener;
use serde_json::Value;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("Incoming stream");

        let mut stream = stream.unwrap();

        let mut buf_n = [0u8; 4];
        stream.read_exact(&mut buf_n).unwrap();
        let message_size = u32::from_be_bytes(buf_n);

        let mut buf = Vec::<u8>::new();
        buf.resize(message_size as usize, 0);
        let size_of_message_read = stream.read(&mut buf).unwrap();

        let message = String::from_utf8_lossy(&buf);
        let str = "{\"test\":{\"key\":\"salut\"}}".to_string();
        let test: HashMap<String, Value> = serde_json::from_str(&str).unwrap();
        for (key, value) in test {
            let test: HashMap<String, String> = serde_json::from_value(value).unwrap();
            for (key, value) in test {
                println!("{} {}", key, value);
            }

        }

        println!("\
        Receive message : {buf:?} \n\
        With size {size_of_message_read} \n\
        Convert to : {message}");
    }
}