use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::common::PublicPlayer;
use crate::MessageType;

pub struct Client {
    pub server_connected_ip: String,
    pub player: PublicPlayer,
}

impl Client {
    pub fn connect(&mut self, server_ip: String) {
        let stream = TcpStream::connect(&server_ip);
        match stream {
            Ok(mut stream) => {
                self.server_connected_ip = server_ip;
                self.send_message(&mut stream, MessageType::Hello);
                let listener = TcpListener::bind(&self.server_connected_ip);
                let listener = self.check_listener(listener);
                let message = self.wait_server_message(&listener);
            }
            Err(err) => {
                panic!("Impossible de se connecter : {err}")
            }
        }
    }

    pub fn send_message(&self, stream: &mut TcpStream, message: MessageType) {
        let message = message.serialize_to_json();
        let buf = message.as_bytes();
        let n = buf.len() as u32;
        let buf_n = n.to_be_bytes();
        let res = stream.write(&buf_n);
        match res {
            Ok(ok) => {
                println!("Size of message send to server");
            }
            Err(err) => {
                panic!("Size of message not send to server");
            }
        }
        let res = stream.write(&buf);
        match res {
            Ok(ok) => {
                println!("Message send to server");
            }
            Err(err) => {
                panic!("Message not send to server");
            }
        }
    }

    pub fn wait_server_message(&self, listener: &TcpListener) {
        for stream in listener.incoming() {
            println!("Incoming stream");
            let mut stream = self.check_stream(stream);
            let message = self.read_messages(&mut stream);
            let message_type = MessageType::deserialize_from_string(&message);
            self.interpret_message(&message_type);
        }
    }

    pub fn check_listener(&self, listener: io::Result<TcpListener>) -> TcpListener {
        match listener {
            Ok(ok) => { return ok; }
            Err(_) => { panic!("Error when receive message of server"); }
        }
    }

    pub fn check_stream(&self, stream: io::Result<TcpStream>) -> TcpStream {
        match stream {
            Ok(ok) => { return ok; }
            Err(_) => { panic!("Error when trying to listen {}", self.server_connected_ip); }
        }
    }

    pub fn read_message_size(&self, stream: &mut TcpStream) -> u32 {
        let mut buf_n = [0u8; 4];
        let res = stream.read_exact(&mut buf_n);
        match res {
            Ok(_) => {}
            Err(_) => {panic!("Error when read message size message of server")}
        }
        return u32::from_be_bytes(buf_n);
    }

    pub fn read_messages(&self, stream: &mut TcpStream) -> String {
        let message_size = self.read_message_size(stream);
        let message = self.read_message_content(stream, message_size);
        return message;
    }

    pub fn read_message_content(&self, stream: &mut TcpStream, message_size: u32) -> String {
        let mut buf = Vec::<u8>::new();
        buf.resize(message_size as usize, 0);
        stream.read(&mut buf);
        return String::from_utf8_lossy(&buf).to_string();
    }
    
    pub fn interpret_message(&self, message_type: &MessageType) {
        match message_type {
            MessageType::Hello => {}
            MessageType::Welcome { .. } => {}
            MessageType::Subscribe { .. } => {}
            MessageType::SubscribeResult(subscribe_result) => {}
            MessageType::PublicLeaderBoard(players) => {}
            MessageType::Challenge(challenge) => {}
            MessageType::ChallengeResult { .. } => {}
            MessageType::ChallengeTimeout { .. } => {}
            MessageType::RoundSummary { .. } => {}
            MessageType::EndOfGame { .. } => {}
        }
    }
}