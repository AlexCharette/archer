use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use log::{error, info};
use protobuf::{parse_from_bytes, Message};
use regex::Regex;
use sawtooth_sdk::messages::events::Event;
use sawtooth_sdk::messages::transaction_receipt::{StateChange, StateChangeList};
use std::cell::RefCell;
use std::rc::Rc;

use archer::{Account, ArcherStructs, ArcherTypes, Merchant, NAME as NAMESPACE};
use archer_protobuf::deserialize_data;
use database::models::{Block, NewAccount, NewMerchant};
use database::*;
use database::PgPool;

const MAX_BLOCK_NUMBER: i64 = i64::MAX;

lazy_static::lazy_static! {
    static ref NAMESPACE_REGEX: Regex = Regex::new(format!(r"^{}", NAMESPACE).as_str()).expect("Error creating regex for namespace");
}

pub fn get_events_handler(pool: PgPool) -> Box<dyn Fn(Vec<Event>)> {
    let connection = Rc::new(RefCell::new(
        pool.get()
            .expect("Error establishing a connection with the database"),
    ));
    Box::new(move |events| handle_events(events, &Rc::clone(&connection).borrow()))
}

pub fn handle_events(events: Vec<Event>, connection: &PgConnection) {
    let (block_num, block_id): (i64, String) =
        parse_new_block(&events).expect("Error parsing new block");
    if !resolve_if_forked(block_num, &block_id, connection) {
        apply_state_changes(events.as_slice(), block_num, &block_id, connection);
    }
    let transaction_result = commit(connection);
    match transaction_result {
        Ok(_) => {}
        Err(err) => {
            error!("Unable to handle event: {}", err);
            rollback(connection).expect("Error carrying out rollback");
        }
    }
}

pub fn apply_state_changes(
    events: &[Event],
    block_num: i64,
    block_id: &str,
    connection: &PgConnection,
) {
    let changes = parse_state_changes(&events);
    for change in changes.iter() {
        let (data_type, mut resources): (ArcherTypes, Vec<ArcherStructs>) =
            deserialize_data(change.get_address(), change.get_value().to_vec());
        insert_block(block_num, block_id, connection).expect("Error inserting block");
        match data_type {
            ArcherTypes::Account => {
                let accounts = resources
                    .drain(..)
                    .map(|resource| {
                        resource
                            .account()
                            .expect("Error converting resource to account")
                    })
                    .collect();
                apply_account_change(block_num, accounts, connection);
            }
            ArcherTypes::Merchant => {
                let merchants = resources
                    .drain(..)
                    .map(|resource| {
                        resource
                            .merchant()
                            .expect("Error converting resource to merchant")
                    })
                    .collect();
                apply_merchant_change(block_num, merchants, connection);
            }
        }
    }
}

pub fn parse_state_changes(events: &[Event]) -> Vec<StateChange> {
    let state_event: Option<&Event> = events
        .iter()
        .find(|event| event.event_type == "sawtooth/state-delta");
    match state_event {
        Some(event) => {
            let event_bytes: Vec<u8> = event
                .write_to_bytes()
                .expect("Error writing event to bytes");
            let state_change_list: StateChangeList =
                parse_from_bytes(&event_bytes).expect("Error parsing state change list from bytes");
            state_change_list
                .get_state_changes()
                .iter()
                .filter(|change| NAMESPACE_REGEX.is_match(change.get_address()))
                .cloned()
                .collect()
        }
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
            public_key: &merchant.public_key,
            name: &merchant.name,
            created: &NaiveDateTime::from_timestamp(merchant.timestamp, 0),
            start_block_num: Some(block_num),
            end_block_num: Some(MAX_BLOCK_NUMBER),
        };
        insert_merchant(new_merchant, connection).expect("Error inserting new merchant");
    }
}

pub fn parse_new_block(events: &[Event]) -> Option<(i64, String)> {
    let block_event: Option<&Event> = events
        .iter()
        .find(|event| event.event_type == "sawtooth/block-commit");
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
            let block_num = block_num
                .parse::<i64>()
                .expect("Error parsing block num from string");
            Some((block_num, block_id))
        }
        None => None,
    }
}

pub fn resolve_if_forked(block_num: i64, block_id: &str, connection: &PgConnection) -> bool {
    let existing_block: Option<Block> =
        fetch_block(block_num, connection).expect("Error fetching block");
    match existing_block {
        Some(block) => {
            if block.block_id == block_id {
                return true;
            }
            info!(
                "Fork detected: replacing {:?} ({:?}) with {:?} ({:?})",
                &block.block_id[..8],
                block.block_num,
                &block_id[..8],
                block_num
            );
            drop_fork(block_num, connection).expect("Error dropping fork");
            false
        }
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
