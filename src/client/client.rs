use std::io;
use std::io::{Read, stdin, Write};
use std::net::{TcpListener, TcpStream};
use crate::challenges::{Challenge, ReportedChallengeResult};
use crate::common::PublicPlayer;
use crate::error::ClientConnexionError;
use crate::messages::{SubscribeError, SubscribeResult};
use crate::MessageType;

pub struct Client {
    server_connected_ip: String,
    game_version: u8,
    stream: Option<TcpStream>,
    listener: Option<TcpListener>
}

impl Client {
    pub fn connect(&mut self, server_ip: String) -> Result<(), ClientConnexionError> {
        let stream = TcpStream::connect(&server_ip);
        let listener = TcpListener::bind("127.0.0.1:7878");
        self.listener = Option::from(self.check_listener(listener));
        self.stream = Option::from(self.check_stream(stream));
        self.server_connected_ip = server_ip;
        println!("You are connected to the server");

        self.send_message(MessageType::Hello)?;

        self.server_communication()?;

        Ok(())
    }

    fn send_message(&mut self, message: MessageType) -> Result<(), ClientConnexionError> {
        let message = message.serialize_to_json();
        let message_buffer = message.as_bytes();
        let message_length = message_buffer.len() as u32;
        let message_length_buffer = message_length.to_be_bytes();

        self.write_message(&message_length_buffer)?;
        self.write_message(&message_buffer)?;

        Ok(())
    }

    fn write_message(&mut self, buffer: &[u8]) -> Result<usize, ClientConnexionError> {

        let res = self.stream.as_ref().ok_or(ClientConnexionError)?.write(buffer);
        match res {
            Ok(ok) => {
                println!("Message send to server");
                Ok(ok)
            }
            Err(_) => {
                println!("Message not send to server");
                Err(ClientConnexionError)
            }
        }
    }

    fn server_communication(&mut self) -> Result<(), ClientConnexionError> {
        let listener = self.listener.take().ok_or(ClientConnexionError)?;
        for stream in listener.incoming() {
            println!("Incoming stream");
            let mut stream = self.check_stream(stream);

            let message = self.read_messages(&mut stream);

            let message_type = MessageType::deserialize_from_string(&message);

            self.interpret_message(&message_type);
        }
        Ok(())
    }

    fn check_stream(&mut self, stream: io::Result<TcpStream>) -> TcpStream {
        match stream {
            Ok(ok) => { return ok; }
            Err(_) => { panic!("Error when trying to connect on {}", self.server_connected_ip); }
        }
    }

    fn check_listener(&mut self, listener: io::Result<TcpListener>) -> TcpListener {
        match listener {
            Ok(ok) => { return ok; }
            Err(_) => { panic!("Error when trying to listen on port 7878");}
        }
    }

    fn read_messages(&mut self, stream: &mut TcpStream) -> String {
        let message_size = self.read_message_size(stream);
        let message = self.read_message_content(stream, message_size);
        return message;
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

    fn on_receive_welcome_message(&mut self, version: &u8) -> Result<(), ClientConnexionError> {
        let mut input = String::new();
        self.game_version = *version;
        println!("Game version : {}", self.game_version.to_string());
        println!("Enter your name :");

        //stdin().read_line(&mut input).expect("Did not enter a correct string");

        self.send_message(MessageType::Subscribe {name: input.to_string()})
    }

    fn on_receive_subscribe_result_message(&mut self, subscribe_result: &SubscribeResult) {
        match subscribe_result {
            SubscribeResult::Ok => {
                println!("You are registered, wait the start of game");
            }
            SubscribeResult::Err(subscribe_error) => {
                match subscribe_error {
                    SubscribeError::AlreadyRegistered => {
                        println!("Your are already registered, or your name is already in use");
                    }
                    SubscribeError::InvalidName => {
                        println!("Your name is invalid, please try again");
                        self.on_receive_welcome_message(&self.game_version.clone());
                    }
                }
            }
        }
    }

    fn on_receive_public_leader_board_message(&mut self, players: &Vec<PublicPlayer>) {
        for player in players {
            println!("{}", player.name);
        }
    }

    fn on_receive_challenge_message(&mut self, challenge: &Challenge) {
        println!("{}", challenge.to_string());
    }

    fn on_receive_challenge_timeout_message(&mut self, message: &String) {
        println!("{}", message);
    }

    fn on_receive_round_summary_message(&mut self, challenge: &String, chain: &Vec<ReportedChallengeResult>) {
        println!("{}", challenge);
        for elem in chain {
            println!("{}", elem.name);
        }
    }

    fn on_receive_end_of_game_message(&mut self, leader_board: &Vec<PublicPlayer>) {
        for player in leader_board {
            println!("{}", player.name);
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::client::client::Client;
    use crate::common::PublicPlayer;
    use crate::messages::{MessageType, SubscribeError, SubscribeResult};

    #[test]
    fn interpret_welcome_message() {
        let mut client = Client{
            server_connected_ip: "".to_string(),
            game_version: 0,
            stream: None,
            listener: None,
        };
        let message = MessageType::Welcome {version: 1};
        client.interpret_message(&message);
    }

    #[test]
    fn interpret_subscribe_result_message() {
        let mut client = Client{
            server_connected_ip: "".to_string(),
            game_version: 0,
            stream: None,
            listener: None,
        };
        let message = MessageType::SubscribeResult(SubscribeResult::Ok);
        client.interpret_message(&message);

        let message = MessageType::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered));
        client.interpret_message(&message);

        let message = MessageType::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName));
        client.interpret_message(&message);
    }

    #[test]
    fn interpret_public_leader_board_message() {
        let mut client = Client{
            server_connected_ip: "".to_string(),
            game_version: 0,
            stream: None,
            listener: None,
        };
    }

    #[test]
    fn interpret_challenge_message() {
        let mut client = Client{
            server_connected_ip: "".to_string(),
            game_version: 0,
            stream: None,
            listener: None,
        };
    }

    #[test]
    fn interpret_challenge_timeout_message() {
        let mut client = Client{
            server_connected_ip: "".to_string(),
            game_version: 0,
            stream: None,
            listener: None,
        };
    }

    #[test]
    fn interpret_round_summary_message() {
        let mut client = Client{
            server_connected_ip: "".to_string(),
            game_version: 0,
            stream: None,
            listener: None,
        };
    }

    #[test]
    fn interpret_end_of_game_message() {
        let mut client = Client{
            server_connected_ip: "".to_string(),
            game_version: 0,
            stream: None,
            listener: None,
        };
    }


}