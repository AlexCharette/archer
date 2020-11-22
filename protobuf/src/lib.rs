use archer::{get_address_type, Account, ArcherStructs, ArcherTypes, Merchant};
use protobuf::{parse_from_bytes, Message};

pub mod account;
pub mod merchant;
pub mod payload;

use account::{Account as AccountPB, AccountContainer};
use merchant::{Merchant as MerchantPB, MerchantContainer};

enum Containers {
    AccountContainer(Box<dyn Message>),
    MerchantContainer(Box<dyn Message>),
}

pub fn deserialize_data(address: &str, data: Vec<u8>) -> (ArcherTypes, Vec<ArcherStructs>) {
    let data_type = get_address_type(address).expect("Invalid address infix");

    let resources = match data_type {
        ArcherTypes::Account => {
            let entries = parse_accounts_from_proto(data);
            entries
                .iter()
                .map(|entry| {
                    convert_proto_to_account(data_type, entry).expect("Invalid struct found")
                })
                .collect()
        }
        ArcherTypes::Merchant => {
            let entries = parse_merchants_from_proto(data);
            entries
                .iter()
                .map(|entry| {
                    convert_proto_to_merchant(data_type, entry).expect("Invalid struct found")
                })
                .collect()
        }
    };

    (data_type, resources)
}

pub fn parse_accounts_from_proto(data: Vec<u8>) -> Vec<AccountPB> {
    let deserialized = parse_from_bytes::<AccountContainer>(&data)
        .ok()
        .expect("Error parsing account from bytes");
    deserialized.get_entries().to_vec()
}

pub fn parse_merchants_from_proto(data: Vec<u8>) -> Vec<MerchantPB> {
    let deserialized = parse_from_bytes::<MerchantContainer>(&data)
        .ok()
        .expect("Error parsing merchant from bytes");
    deserialized.get_entries().to_vec()
}

pub fn convert_proto_to_account(
    data_type: ArcherTypes,
    entry: &AccountPB,
) -> Option<ArcherStructs> {
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
        }
        ArcherTypes::Merchant => None,
    }
}

pub fn convert_proto_to_merchant(
    data_type: ArcherTypes,
    entry: &MerchantPB,
) -> Option<ArcherStructs> {
    match data_type {
        ArcherTypes::Account => None,
        ArcherTypes::Merchant => {
            let merchant = Merchant {
                public_key: String::from(entry.get_public_key()),
                name: String::from(entry.get_name()),
                timestamp: entry.get_timestamp(),
                start_block_num: None,
                end_block_num: None,
            };
            Some(ArcherStructs::Merchant(merchant))
        }
    }
}
