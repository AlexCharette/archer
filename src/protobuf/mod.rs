
use protobuf::{Message, parse_from_bytes};
use crate::archer::{Account, ArcherStructs, ArcherTypes, get_address_type, Merchant};

pub mod account;
pub mod payload;

use account::{Account as AccountPB, AccountContainer};
use merchant::{Merchant as MerchantPB, MerchantContainer};

enum Containers {
    AccountContainer(Box<dyn Message>),
    MerchantContainer(Box<dyn Message>),
}

pub fn deserialize_data(address: &str, data: Vec<u8>) -> (ArcherTypes, Vec<ArcherStructs>) {
    let data_type = get_address_type(address).expect("Invalid address infix");

    let entries = match data_type {
        ArcherTypes::Account => parse_accounts_from_proto(data),
        ArcherTypes::Merchant => parse_merchants_from_proto(data),
    };

    let resources = entries.iter().map(|entry| {
        convert_proto_to_struct(data_type, entry).expect("Invalid struct found")
    }).collect();

    (data_type, resources)
}

pub fn parse_accounts_from_proto(data: Vec<u8>) -> Vec<AccountPB> {
    let deserialized = parse_from_bytes::<AccountContainer>(&data)
        .ok().expect("Error parsing account from bytes");
    deserialized.get_entries().to_vec()
}

pub fn parse_merchants_from_proto(data: Vec<u8>) -> Vec<MerchantPB> {
    let deserialized= parse_from_bytes::<AccountContainer>(&data)
        .ok().expect("Error parsing account from bytes");
    deserialized.get_entries().to_vec()
}

pub fn convert_proto_to_struct(data_type: ArcherTypes, entry: &AccountPB) -> Option<ArcherStructs> {
    match data_type {
        ArcherTypes::Account => {
            let account = Account {
                name: String::from(entry.get_name()),
                number: entry.get_number(),
                balance: entry.get_balance(),
                start_block_num: None,
                end_block_num: None,
            };
            Some(ArcherStructs::Account(account))
        },
        ArcherTypes::Merchant => {
            let merchant = Merchant {
                name: String::from(entry.get_name()),
                number: entry.get_number(),
                balance: entry.get_balance(),
                start_block_num: None,
                end_block_num: None,
            };
            Some(ArcherStructs::Merchant(merchant))
        },
    }
}