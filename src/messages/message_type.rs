use std::fmt;
use serde::{Deserialize, Serialize};
use crate::challenges::{Challenge, ChallengeAnswer, ReportedChallengeResult};
use crate::common::PublicPlayer;
use crate::messages::SubscribeResult;


#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum MessageType {
    Hello,
    Welcome {version: u8},
    Subscribe {name: String},
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult {answer: ChallengeAnswer, next_target: String},
    ChallengeTimeout {message: String},
    RoundSummary {challenge: String, chain: Vec<ReportedChallengeResult>},
    EndOfGame {leader_board: Vec<PublicPlayer>}
}

impl MessageType {
    pub fn serialize_to_json(&self) -> String {
        let result =  serde_json::to_string(self);
        match result {
            Ok(ok) => {
                return ok;
            }
            Err(_) => {
                panic!("Le json fournit n'est pas celui d'un message type");
            }
        }
    }

    pub fn deserialize_from_buffer(buf: &Vec<u8>) -> MessageType {
        let message = String::from_utf8_lossy(&buf).to_string();
        return MessageType::deserialize_from_string(&message);
    }

    pub fn deserialize_from_string(str: &String) -> MessageType {
        let deserialized = serde_json::from_str::<MessageType>(&str);
        match deserialized {
            Ok(ok) => {
                return ok;
            }
            Err(_) => {
                panic!("Le json fournit n'est pas celui d'un message type");
            }
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::challenges::ChallengeValue;
    use crate::challenges::hash_cash::{MD5HashCashInput, MD5HashCashOutput};
    use crate::messages::SubscribeError;
    use super::*;
    use crate::messages::SubscribeResult;

    #[test]
    fn serialize_deserialize_hello_message() {
        let message = MessageType::Hello;
        let serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "\"Hello\"");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_welcome_message() {
        let message = MessageType::Welcome { version: 1 };
        let serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "{\
                        \"Welcome\":{\"version\":1}\
                   }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_subscribe_message() {
        let message = MessageType::Subscribe { name: "free_patato".to_string() };
        let mut serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "{\
                        \"Subscribe\":{\"name\":\"free_patato\"}\
                   }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_subscribe_result_message() {
        let message = MessageType::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName));
        let serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "{\
                        \"SubscribeResult\":{\"Err\":\"InvalidName\"}\
                   }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_public_leader_board_message() {
        let players = vec![
            PublicPlayer { name: "free_patato".to_string(), stream_id: "127.0.0.1".to_string(), score: 10, steps: 20, is_active: true, total_used_time: 1.234 },
            PublicPlayer { name: "dark_salad".to_string(), stream_id: "127.0.0.1".to_string(), score: 6, steps: 200, is_active: true, total_used_time: 0.1234 }
        ];
        let message = MessageType::PublicLeaderBoard(players);
        let serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "{\
                        \"PublicLeaderBoard\":[\
                            {\"name\":\"free_patato\",\"stream_id\":\"127.0.0.1\",\"score\":10,\"steps\":20,\"is_active\":true,\"total_used_time\":1.234},\
                            {\"name\":\"dark_salad\",\"stream_id\":\"127.0.0.1\",\"score\":6,\"steps\":200,\"is_active\":true,\"total_used_time\":0.1234}\
                        ]\
                   }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_challenge_message() {
        let message = MessageType::Challenge(
            Challenge::MD5HashCash(
                MD5HashCashInput {
                    complexity: 5,
                    message: "Hello".to_string()
                }
            )
        );
        let serialized = message.serialize_to_json();
        assert_eq!(serialized, "{\
                                    \"Challenge\":{\"MD5HashCash\":{\"complexity\":5,\"message\":\"Hello\"}}\
                                }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_challenge_result_message() {
        let message = MessageType::ChallengeResult {
            answer: ChallengeAnswer::MD5HashCash(MD5HashCashOutput { seed: 12345678, hashcode: "68B329DA9893E34099C7D8AD5CB9C940".to_string() }),
            next_target: "dark_salad".to_string()
        };
        let serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "{\
                       \"ChallengeResult\":{\
                            \"answer\":{\"MD5HashCash\":{\"seed\":12345678,\"hashcode\":\"68B329DA9893E34099C7D8AD5CB9C940\"}},\
                            \"next_target\":\"dark_salad\"\
                       }\
                   }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_challenge_timeout_message() {
        let message = MessageType::ChallengeTimeout {
            message: "You've been fired!".to_string()
        };
        let serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "{\
                        \"ChallengeTimeout\":{\"message\":\"You've been fired!\"}\
                   }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_round_summary_message() {
        let message = MessageType::RoundSummary {
            challenge: "MD5HashCash".to_string(),
            chain: vec![
                ReportedChallengeResult { name: "free_patato".to_string(), value: ChallengeValue::Ok { used_time: 0.1, next_target: "dark_salad".to_string() } },
                ReportedChallengeResult { name: "dark_salad".to_string(), value: ChallengeValue::Unreachable },
            ]
        };
        let serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "{\
                       \"RoundSummary\":{\
                            \"challenge\":\"MD5HashCash\",\
                            \"chain\":[\
                                {\"name\":\"free_patato\",\"value\":{\"Ok\":{\"used_time\":0.1,\"next_target\":\"dark_salad\"}}},\
                                {\"name\":\"dark_salad\",\"value\":\"Unreachable\"}\
                            ]\
                       }\
                   }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }

    #[test]
    fn serialize_deserialize_end_of_game_message() {
        let message = MessageType::EndOfGame {leader_board: vec![
            PublicPlayer { name: "free_patato".to_string(), stream_id: "127.0.0.1".to_string(), score: 10, steps: 20, is_active: true, total_used_time: 1.234 },
            PublicPlayer { name: "dark_salad".to_string(), stream_id: "127.0.0.1".to_string(), score: 6, steps: 200, is_active: true, total_used_time: 0.1234 }
        ]};
        let serialized = message.serialize_to_json();
        assert_eq!(serialized,
                   "{\
                        \"EndOfGame\":{\
                            \"leader_board\":[\
                                {\"name\":\"free_patato\",\"stream_id\":\"127.0.0.1\",\"score\":10,\"steps\":20,\"is_active\":true,\"total_used_time\":1.234},\
                                {\"name\":\"dark_salad\",\"stream_id\":\"127.0.0.1\",\"score\":6,\"steps\":200,\"is_active\":true,\"total_used_time\":0.1234}\
                            ]\
                        }\
                   }");
        let deserialized = MessageType::deserialize_from_string(&serialized);
        assert_eq!(deserialized, message);
    }
}


