// use regex::Regex;
use protobuf::{parse_from_bytes, Message, ProtobufError};
use sawtooth_sdk::processor::handler::ApplyError;
// use serde::{Serialize, Deserialize};

use archer_protobuf::payload::{Payload as PayloadPB, Payload_Action};

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct ArcherPayload {
    payload: PayloadPB,
    action: Payload_Action,
}

impl ArcherPayload {
    pub fn new(payload_data: &[u8]) -> Result<ArcherPayload, ApplyError> {
        let payload: Result<PayloadPB, ProtobufError> = parse_from_bytes(payload_data);

        let payload: PayloadPB = match payload {
            Ok(pl) => pl,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Invalid payload serialization",
                )))
            }
        };

        let action = payload.get_action();

        Ok(ArcherPayload {
            payload: payload,
            action: action,
        })

        // let items: Vec<&str> = payload_string.split(",").collect();

        // if items.len() != 3 {
        //     return Err(ApplyError::InvalidTransaction(String::from("Payload must have exactly 2 commas")));
        // }

        // let (name, action) = (items[0], items[1]);
        // let amount = items[2].parse::<i32>().expect("Error parsing i32 from string data");

        // if name.is_empty() {
        //     return Err(ApplyError::InvalidTransaction(String::from("Name is required")));
        // }

        // if action.is_empty() {
        //     return Err(ApplyError::InvalidTransaction(String::from("Action is required")));
        // }

        // if name.contains("|") {
        //     return Err(ApplyError::InvalidTransaction(String::from("Name cannot contain |")));
        // }

        // let name_pattern: Regex = Regex::new(r"^\[a-z]{1,9}\d{7}$").expect("Error creating regex from str");
        // if !(name_pattern.is_match(name)) {
        //     return Err(ApplyError::InvalidTransaction(String::from("Name must have between 1 and 9 characters and 7 numbers")));
        // }

        // if amount < 0 {
        //     return Err(ApplyError::InvalidTransaction(String::from("Amount canot be negative")));
        // }

        // match action {
        //     "deposit" | "withdraw" | "get_balance" | "update_account" => (),
        //     _ => {
        //         return Err(ApplyError::InvalidTransaction(String::from(
        //             format!("Invalid action: {}", action).as_str()
        //         )));
        //     }
        // };
    }

    pub fn data(&self) -> Result<Vec<u8>, ApplyError> {
        match self.payload.get_action() {
            Payload_Action::DEPOSIT => {
                if self
                    .payload
                    .descriptor()
                    .get_field_by_name("amount")
                    .is_some()
                {
                    Ok(self
                        .payload
                        .write_to_bytes()
                        .expect("Error converting action message to bytes"))
                } else {
                    Err(ApplyError::InvalidTransaction(String::from(
                        "Action does not match payload data",
                    )))
                }
            }
            Payload_Action::WITHDRAW => {
                if self
                    .payload
                    .descriptor()
                    .get_field_by_name("amount")
                    .is_some()
                {
                    Ok(self
                        .payload
                        .write_to_bytes()
                        .expect("Error converting action message to bytes"))
                } else {
                    Err(ApplyError::InvalidTransaction(String::from(
                        "Action does not match payload data",
                    )))
                }
            }
            Payload_Action::UPDATE_NUMBER => {
                if self
                    .payload
                    .descriptor()
                    .get_field_by_name("new_number")
                    .is_some()
                {
                    Ok(self
                        .payload
                        .write_to_bytes()
                        .expect("Error converting action message to bytes"))
                } else {
                    Err(ApplyError::InvalidTransaction(String::from(
                        "Action does not match payload data",
                    )))
                }
            }
            Payload_Action::ADD_ACCOUNT => Ok(self
                .payload
                .write_to_bytes()
                .expect("Error converting action message to bytes")),
            Payload_Action::ADD_MERCHANT => Ok(self
                .payload
                .write_to_bytes()
                .expect("Error converting action message to bytes")),
        }
    }

    pub fn action(&self) -> Payload_Action {
        self.action
    }

    pub fn name(&self) -> String {
        String::from(self.payload.get_name())
    }

    pub fn number(&self) -> u32 {
        self.payload.get_number()
    }
}
