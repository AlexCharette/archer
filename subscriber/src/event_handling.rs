
use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;
use log::{error, info};
use diesel::pg::PgConnection;
use protobuf::{Message, parse_from_bytes};
use sawtooth_sdk::messages::events::Event;
use sawtooth_sdk::messages::transaction_receipt::{StateChange, StateChangeList};

use crate::archer::{Account, ArcherStructs, ArcherTypes, Merchant, NAME as NAMESPACE};
use crate::database::*;
use crate::database::{PgPool, PgPooledConnection};
use crate::database::models::{Block, NewAccount};
use crate::protobuf::deserialize_data;

const MAX_BLOCK_NUMBER: i64 = i64::MAX;

lazy_static::lazy_static! {
    static ref NAMESPACE_REGEX: Regex = Regex::new(format!(r"^{}", NAMESPACE).as_str()).expect("Error creating regex for namespace");
}

pub fn get_events_handler(pool: PgPool) -> Box<dyn Fn(Vec<Event>)> {
    let connection = Rc::new(RefCell::new(pool.get().expect("Error establishing a connection with the database")));
    Box::new(move |events| handle_events(events, &Rc::clone(&connection).borrow()))
}

pub fn handle_events(events: Vec<Event>, connection: &PgConnection) {
    let (block_num, block_id): (i64, String) = parse_new_block(&events).expect("Error parsing new block");
    if !resolve_if_forked(block_num, &block_id, connection) {
        apply_state_changes(events.as_slice(), block_num, &block_id, connection);
    }
    let transaction_result = commit(connection); 
    match transaction_result {
        Ok(_) => {},
        Err(err) => {
            error!("Unable to handle event: {}", err);
            rollback(connection).expect("Error carrying out rollback");
        }
    }
}

pub fn apply_state_changes(events: &[Event], block_num: i64, block_id: &str, connection: &PgConnection) {
    let changes = parse_state_changes(&events);
    for change in changes.iter() {
        let (data_type, mut resources): (ArcherTypes, Vec<ArcherStructs>) = deserialize_data(change.get_address(), change.get_value().to_vec());
        insert_block(block_num, block_id, connection).expect("Error inserting block");
        match data_type {
            ArcherTypes::Account => {
                let accounts = resources
                    .drain(..)
                    .map(|resource| resource.account().expect("Error converting resource to account")).collect();
                apply_account_change(block_num, accounts, connection);
            },
            ArcherTypes::Merchant => {
                let merchants = resources
                    .drain(..)
                    .map(|resource| resource.account().expect("Error converting resource to merchant")).collect();
                apply_merchant_change(block_num, merchants, connection);
            }
        }
    }
}

pub fn parse_state_changes(events: &[Event]) -> Vec<StateChange> {
    let state_event: Option<&Event> = events.iter()
        .filter(|event| 
            event.event_type == "sawtooth/state-delta"
        ).next();
    match state_event {
        Some(event) => {
            let event_bytes: Vec<u8> = event.write_to_bytes().ok().expect("Error writing event to bytes");
            let state_change_list: StateChangeList = parse_from_bytes(&event_bytes).ok().expect("Error parsing state change list from bytes");
            state_change_list.get_state_changes().iter().filter(|change| 
                NAMESPACE_REGEX.is_match(change.get_address())
            ).cloned().collect()
        },
        None => Vec::<StateChange>::new(),
    }
}

pub fn apply_account_change(block_num: i64, accounts: Vec<Account>, connection: &PgConnection) {
    for account in accounts {
        let new_account = NewAccount {
            name: &account.name,
            number: account.number as i32,
            balance: account.balance,
            start_block_num: Some(block_num),
            end_block_num: Some(MAX_BLOCK_NUMBER),
        };
        insert_account(new_account, connection).expect("Error inserting new account");
    }
}

pub fn apply_merchant_change(block_num: i64, merchants: Vec<Merchant>, connection: &PgConnection) {
    for merchant in merchants {
        let new_merchant = NewMerchant {
            name: &merchant.name,
            number: merchant.number as i32,
            start_block_num: Some(block_num),
            end_block_num: Some(MAX_BLOCK_NUMBER),
        };
        insert_merchant(new_merchant, connection).expect("Error inserting new merchant");
    }
}

pub fn parse_new_block(events: &[Event]) -> Option<(i64, String)> {
    let block_event: Option<&Event> = events.iter()
        .filter(|event| 
            event.event_type == "sawtooth/block-commit"
        ).next();
    match block_event {
        Some(e) => {
            let block_attrs = e.get_attributes().to_vec();
            let mut block_num = String::new();
            let mut block_id = String::new();
            for attr in block_attrs.iter() {
                if attr.get_key() == "block_num" {
                    block_num = String::from(attr.get_value());
                    break;
                } else if attr.get_key() == "block_id" {
                    block_id = String::from(attr.get_value());
                    break;
                }
            }
            let block_num = block_num.parse::<i64>().ok().expect("Error parsing block num from string");
            Some((block_num, String::from(block_id)))
        },
        None => None,
    }
}

pub fn resolve_if_forked(block_num: i64, block_id: &str, connection: &PgConnection) -> bool {
    let existing_block: Option<Block> = fetch_block(block_num, connection).ok().expect("Error fetching block");
    match existing_block {
        Some(block) => {
            if block.block_id == block_id {
                return true;
            }
            info!("Fork detected: replacing {:?} ({:?}) with {:?} ({:?})",
                &block.block_id[..8],
                block.block_num,
                &block_id[..8],
                block_num
            );
            drop_fork(block_num, connection).expect("Error dropping fork");
            false
        },
        None => false,
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_apply_state_changes() {}

    #[test]
    fn test_parse_state_changes() {}

    #[test]
    fn test_apply_account_change() {}

    #[test]
    fn test_parse_new_block() {}

    #[test]
    fn test_resolve_fork() {}
}

