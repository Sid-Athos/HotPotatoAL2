use std::io::Write;
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream) => {
            let message = "Hello";
            let buf = message.as_bytes();
            let n = buf.len() as u32;
            let buf_n = n.to_be_bytes();
            stream.write(&buf_n).unwrap();
            stream.write(&buf).unwrap();
        }
        Err(err) => {
            panic!("Impossible de se connecter : {err}")
        }
    }
}