use std::io;
use std::io::{Read, stdin, Write};
use std::net::{TcpListener, TcpStream};
use crate::challenges::{Challenge, ChallengeAnswer, ChallengeTrait, ReportedChallengeResult};
use crate::challenges::hash_cash::{MD5HashCashChallenge, MD5HashCashInput, MD5HashCashOutput};
use crate::challenges::monstrous_maze::MonstrousMazeChallenge;
use crate::challenges::nonogram::NonogramChallenge;
use crate::challenges::recover_secret::RecoverSecretChallenge;
use crate::common::PublicPlayer;
use crate::error::ClientConnexionError;
use crate::messages::{SubscribeError, SubscribeResult};
use crate::MessageType;

pub struct Client {
    pub pseudo: String,
    pub server_connected_ip: String,
    pub game_version: u8,
    pub leader_board: Option<Vec<PublicPlayer>>,
    pub is_connected: bool,
    pub stream: Option<TcpStream>,
    pub listener: Option<TcpListener>
}

impl Client {
    pub fn connect(&mut self, server_ip: &String) -> Result<(), ClientConnexionError> {
        self.server_connected_ip = server_ip.clone();
        let stream = TcpStream::connect(server_ip.to_string());
        match self.check_stream(stream) {
            Ok(_) => {

                println!("You are connected to the server");

                self.send_message(MessageType::Hello)?;

                while self.is_connected {
                    self.wait_server_message()?;
                }

                Ok(())
            }
            Err(_) => {
                Err(ClientConnexionError)
            }
        }
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

        let res = self.stream.as_mut().ok_or(ClientConnexionError)?.write(buffer);
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

    fn wait_server_message(&mut self) -> Result<(), ClientConnexionError> {
        let message = self.read_messages();
        let message_type = MessageType::deserialize_from_string(&message?);
        self.interpret_message(message_type);

        Ok(())
    }

    fn check_stream(&mut self, stream: io::Result<TcpStream>) -> Result<(), ClientConnexionError> {
        return match stream {
            Ok(ok) => {
                self.stream = Option::from(ok);
                self.is_connected = true;
                println!("You are connected on '{0}' server", self.server_connected_ip);
                Ok(())
            }
            Err(_) => {
                println!("Error when trying to connect on '{0}'", self.server_connected_ip);
                Err(ClientConnexionError)
            }
        }
    }

    fn read_messages(&mut self) -> Result<String, ClientConnexionError> {
        let message_size = self.read_message_size();
        let message = self.read_message_content(message_size?);
        message
    }

    fn read_message_size(&mut self) -> Result<u32, ClientConnexionError> {
        let mut buf_n = [0u8; 4];
        let res = self.stream.as_mut().ok_or(ClientConnexionError)?.read_exact(&mut buf_n);
        match res {
            Ok(_) => {
                Ok(u32::from_be_bytes(buf_n))
            }
            Err(_) => {
                println!("Error when read message 'size message' of server");
                Err(ClientConnexionError)
            }
        }
    }

    fn read_message_content(&mut self, message_size: u32) -> Result<String, ClientConnexionError> {
        let mut buf = Vec::<u8>::new();
        buf.resize(message_size as usize, 0);
        let res = self.stream.as_mut().ok_or(ClientConnexionError)?.read_exact(&mut buf);
        match res {
            Ok(_) => {
                Ok(String::from_utf8_lossy(&buf).to_string())
            }
            Err(_) => {
                println!("Error when read message 'content message' of server");
                Err(ClientConnexionError)
            }
        }
    }

    pub fn interpret_message(&mut self, message_type: MessageType) {
        let res;
        match message_type {
            MessageType::Welcome { version } => {
                res = self.on_receive_welcome_message(version);
            }
            MessageType::SubscribeResult(subscribe_result) => {
                res = self.on_receive_subscribe_result_message(subscribe_result);
            }
            MessageType::PublicLeaderBoard(players) => {
                res = self.on_receive_public_leader_board_message(players);
            }
            MessageType::Challenge(challenge) => {
                res = self.on_receive_challenge_message(challenge);
            }
            MessageType::ChallengeTimeout { message } => {
                res = self.on_receive_challenge_timeout_message(message);
            }
            MessageType::RoundSummary { challenge, chain } => {
                res = self.on_receive_round_summary_message(challenge, chain);
            }
            MessageType::EndOfGame { leader_board } => {
                res = self.on_receive_end_of_game_message(leader_board);
                self.is_connected = false;
            }
            _ => {
                println!("Unknown received message");
            }
        }

    }

    fn on_receive_welcome_message(&mut self, version: u8) -> Result<(), ClientConnexionError> {
        self.game_version = version;
        println!("Game version : {}", self.game_version.to_string());

        self.send_message(MessageType::Subscribe {name: self.pseudo.clone()})
    }

    fn on_receive_subscribe_result_message(&mut self, subscribe_result: SubscribeResult) -> Result<(), ClientConnexionError> {
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
                        //self.on_receive_welcome_message(&self.game_version.clone());
                    }
                }
            }
        }
        Ok(())
    }

    fn on_receive_public_leader_board_message(&mut self, players: Vec<PublicPlayer>) -> Result<(), ClientConnexionError>{
        self.leader_board = Option::from(players);
        Ok(())
    }

    fn on_receive_challenge_message(&mut self, challenge: Challenge) -> Result<(), ClientConnexionError>{
        println!("{}", challenge.to_string());
        let mut challenge_res: ChallengeAnswer;
        let mut is_valid_res: bool = false;
        match challenge {
            Challenge::MD5HashCash(input) => {
                let challenge = MD5HashCashChallenge::new(input);
                let output = challenge.solve();
                is_valid_res = challenge.verify(&output);
                challenge_res = ChallengeAnswer::MD5HashCash(output);
            }
            Challenge::MonstrousMaze(input) => {
                let challenge = MonstrousMazeChallenge::new(input);
                let output = challenge.solve();
                is_valid_res = challenge.verify(&output);
                challenge_res = ChallengeAnswer::MonstrousMaze(output);
            }
            Challenge::RecoverSecret(input) => {
                let challenge = RecoverSecretChallenge::new(input);
                let output = challenge.solve();
                is_valid_res = challenge.verify(&output);
                challenge_res = ChallengeAnswer::RecoverSecret(output);
            }
            Challenge::NonogramSolver(input) => {
                let challenge = NonogramChallenge::new(input);
                let output = challenge.solve();
                is_valid_res = challenge.verify(&output);
                challenge_res = ChallengeAnswer::NonogramSolver(output);
            }
        }
        if is_valid_res {
            let players = self.leader_board.take().ok_or(ClientConnexionError);
            match players {
                Ok(players) => {
                    let principal_target = players.get(0);
                    let secondary_target = players.get(1);
                    let mut target: &PublicPlayer;
                    match principal_target {
                        None => {
                            return Err(ClientConnexionError)
                        }
                        Some(ok_target) => {
                            if ok_target.name != self.pseudo {
                                target = ok_target;
                            }
                            else {
                                match secondary_target {
                                    None => {
                                        return Err(ClientConnexionError)
                                    }
                                    Some(ok_target) => {
                                        target = ok_target;
                                    }
                                }
                            }
                        }
                    }
                    let res = self.send_message(MessageType::ChallengeResult {
                        answer: challenge_res,
                        next_target: target.name.clone(),
                    });
                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(err)
                        }
                    }
                }
                Err(err) => {
                    return Err(err)
                }
            }
        }
        Ok(())
    }

    fn on_receive_challenge_timeout_message(&mut self, message: String) -> Result<(), ClientConnexionError> {
        println!("{}", message);
        Ok(())
    }

    fn on_receive_round_summary_message(&mut self, challenge: String, chain: Vec<ReportedChallengeResult>) -> Result<(), ClientConnexionError> {
        println!("{}", challenge);
        for elem in chain {
            println!("{}", elem.name);
        }
        Ok(())
    }

    fn on_receive_end_of_game_message(&mut self, leader_board: Vec<PublicPlayer>) -> Result<(), ClientConnexionError> {
        for player in leader_board {
            println!("{}", player.name);
        }
        Ok(())
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
            pseudo: "Barlords".to_string(),
            server_connected_ip: "".to_string(),
            game_version: 0,
            leader_board: None,
            is_connected: false,
            stream: None,
            listener: None,
        };
        let message = MessageType::Welcome {version: 1};
        client.interpret_message(message);
    }

    #[test]
    fn interpret_subscribe_result_message() {
        let mut client = Client{
            pseudo: "Barlords".to_string(),
            server_connected_ip: "".to_string(),
            game_version: 0,
            leader_board: None,
            is_connected: false,
            stream: None,
            listener: None,
        };
        let message = MessageType::SubscribeResult(SubscribeResult::Ok);
        client.interpret_message(message);

        let message = MessageType::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered));
        client.interpret_message(message);

        let message = MessageType::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName));
        client.interpret_message(message);
    }

    #[test]
    fn interpret_public_leader_board_message() {
        let mut client = Client{
            pseudo: "Barlords".to_string(),
            server_connected_ip: "".to_string(),
            game_version: 0,
            leader_board: None,
            is_connected: false,
            stream: None,
            listener: None,
        };
    }

    #[test]
    fn interpret_challenge_message() {
        let mut client = Client{
            pseudo: "Barlords".to_string(),
            server_connected_ip: "".to_string(),
            game_version: 0,
            leader_board: None,
            is_connected: false,
            stream: None,
            listener: None,
        };
    }

    #[test]
    fn interpret_challenge_timeout_message() {
        let mut client = Client{
            pseudo: "Barlords".to_string(),
            server_connected_ip: "".to_string(),
            game_version: 0,
            leader_board: None,
            is_connected: false,
            stream: None,
            listener: None,
        };
    }

    #[test]
    fn interpret_round_summary_message() {
        let mut client = Client{
            pseudo: "Barlords".to_string(),
            server_connected_ip: "".to_string(),
            game_version: 0,
            leader_board: None,
            is_connected: false,
            stream: None,
            listener: None,
        };
    }

    #[test]
    fn interpret_end_of_game_message() {
        let mut client = Client{
            pseudo: "Barlords".to_string(),
            server_connected_ip: "".to_string(),
            game_version: 0,
            leader_board: None,
            is_connected: false,
            stream: None,
            listener: None,
        };
    }


}