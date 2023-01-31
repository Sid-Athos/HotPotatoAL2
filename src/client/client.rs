use core::panicking::panic;
use std::hash::Hasher;
use std::io;
use std::io::{Read, stdin, stdout, Write};
use std::net::{TcpListener, TcpStream};
use crate::challenges::{Challenge, ReportedChallengeResult};
use crate::common::PublicPlayer;
use crate::messages::{SubscribeError, SubscribeResult};
use crate::MessageType;

pub struct Client {
    pub server_connected_ip: String,
    pub player: PublicPlayer,
    pub game_version: u8,
    stream: TcpStream,
    listener: TcpListener
}

impl Client {
    pub fn connect(&mut self, server_ip: String) {
        let stream = TcpStream::connect(&server_ip);
        let listener = TcpListener::bind("127.0.0.1:7878");
        self.listener = self.check_listener(listener);
        self.stream = self.check_stream(stream);
        self.server_connected_ip = server_ip;
        println!("You are connected to the server");

        self.send_message(MessageType::Hello);

        self.server_communication();

    }

    pub fn send_message(&mut self, message: MessageType) {
        let message = message.serialize_to_json();
        let message_buffer = message.as_bytes();
        let message_length = message_buffer.len() as u32;
        let message_length_buffer = message_length.to_be_bytes();

        self.write_message(&message_length_buffer);
        self.write_message(&message_buffer);
    }

    pub fn write_message(&mut self, buffer: &[u8]) -> usize {
        let res = self.stream.write(buffer);
        match res {
            Ok(ok) => {
                println!("Message send to server");
                return ok;
            }
            Err(_) => {
                panic!("Message not send to server");
            }
        }
    }

    pub fn server_communication(&mut self) {

        for stream in self.listener.incoming() {
            println!("Incoming stream");
            let mut stream = self.check_stream(stream);

            let message = self.read_messages(&mut stream);

            let message_type = MessageType::deserialize_from_string(&message);
            self.interpret_message(&message_type);
        }

    }

    pub fn check_stream(&self, stream: io::Result<TcpStream>) -> TcpStream {
        match stream {
            Ok(ok) => { return ok; }
            Err(_) => { panic!("Error when trying to connect on {}", self.server_connected_ip); }
        }
    }

    pub fn check_listener(&self, listener: io::Result<TcpListener>) -> TcpListener {
        match listener {
            Ok(ok) => { return ok; }
            Err(_) => { panic("Error when trying to listen on port 7878");}
        }
    }

    fn read_message_size(&self, stream: &mut TcpStream) -> u32 {
        let mut buf_n = [0u8; 4];
        let res = stream.read_exact(&mut buf_n);
        match res {
            Ok(_) => {}
            Err(_) => {panic!("Error when read message 'size message' of server")}
        }
        return u32::from_be_bytes(buf_n);
    }

    pub fn read_messages(&self, stream: &mut TcpStream) -> String {
        let message_size = self.read_message_size(stream);
        let message = self.read_message_content(stream, message_size);
        return message;
    }

    fn read_message_content(&self, stream: &mut TcpStream, message_size: u32) -> String {
        let mut buf = Vec::<u8>::new();
        buf.resize(message_size as usize, 0);
        let res = stream.read_exact(&mut buf);
        match res {
            Ok(_) => {}
            Err(_) => {panic!("Error when read message 'content message' of server")}
        }
        return String::from_utf8_lossy(&buf).to_string();
    }
    
    pub fn interpret_message(&mut self, message_type: &MessageType) {
        match message_type {
            MessageType::Welcome { version } => {
                self.on_receive_welcome_message(version);
            }
            MessageType::SubscribeResult(subscribe_result) => {
                self.on_receive_subscribe_result_message(subscribe_result);
            }
            MessageType::PublicLeaderBoard(players) => {
                self.on_receive_public_leader_board_message(players);
            }
            MessageType::Challenge(challenge) => {
                self.on_receive_challenge_message(challenge);
            }
            MessageType::ChallengeTimeout { message } => {
                self.on_receive_challenge_timeout_message(message);
            }
            MessageType::RoundSummary { challenge, chain } => {
                self.on_receive_round_summary_message(challenge, chain);
            }
            MessageType::EndOfGame { leader_board } => {
                self.on_receive_end_of_game_message(leader_board);
            }
            _ => {
                println!("Unknown received message");
            }
        }
    }

    fn on_receive_welcome_message(&mut self, version: &u8) {
        let mut input: String;
        self.game_version = *version;
        println!("Game version : {}", self.game_version.to_string());
        println!("Enter your name :");
        stdin().read_line(&mut input).expect("Did not enter a correct string");

        self.send_message(MessageType::Subscribe {name: input.to_string()})
    }

    fn on_receive_subscribe_result_message(&mut self, subscribe_result: &SubscribeResult) {
        match subscribe_result {
            Ok(_) => {
                println!("You are registered, wait the start of game");
            }
            Err(err) => {
                match err {
                    SubscribeError::AlreadyRegistered => {
                        println!("Your are already registered, or your name is already in use");
                    }
                    SubscribeError::InvalidName => {
                        println!("Your name is invalid, please try again");
                        on_receive_welcome_message(self.game_version);
                    }
                }
            }
        }
    }

    fn on_receive_public_leader_board_message(&mut self, players: &Vec<PublicPlayer>) {
        for player in players {
            println!(player);
        }
    }

    fn on_receive_challenge_message(&mut self, challenge: &Challenge) {
        println!(challenge);
    }

    fn on_receive_challenge_timeout_message(&mut self, message: &String) {
        println!(message);
    }

    fn on_receive_round_summary_message(&mut self, challenge: &String, chain: &Vec<ReportedChallengeResult>) {
        println!(challenge);
        println!(chain);
    }

    fn on_receive_end_of_game_message(&mut self, leader_board: &Vec<PublicPlayer>) {
        for player in leader_board {
            println!(player);
        }
    }

}