use archer::{get_address_type, Account, ArcherStructs, ArcherTypes, Merchant};
use protobuf::{parse_from_bytes, Message};

pub mod account;
pub mod merchant;
pub mod payload;

use account::{Account as AccountPB, AccountContainer};
use merchant::{Merchant as MerchantPB, MerchantContainer};

// TODO ! what do I do with this?
enum _Containers {
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
        .expect("Error parsing account from bytes");
    deserialized.get_entries().to_vec()
}

pub fn parse_merchants_from_proto(data: Vec<u8>) -> Vec<MerchantPB> {
    let deserialized = parse_from_bytes::<MerchantContainer>(&data)
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

#[cfg(test)]
mod tests {
    use super::*;
    use archer::{get_address_type, Account, ArcherStructs, ArcherTypes, Merchant};

    #[test]
    fn proto_to_account() {
        let data_type = ArcherTypes::Account;
        let name = "John Doe";
        let number: u32 = 12345;
        let mut entry = AccountPB::default();
        entry.set_name(name.to_string());
        entry.set_number(number);
        entry.set_balance(1000);
        entry.compute_size();
        let account = convert_proto_to_account(data_type, &entry).unwrap();
        let result = account.account().unwrap();
        assert_eq!(&result.name, name);
    }

    #[test]
    fn proto_to_merchant() {
        let data_type = ArcherTypes::Merchant;
        let name = "Bob's Poutine";
        let mut entry = MerchantPB::default();
        entry.set_public_key("abcdefghijklmnopqrstuvwxyz1234567890".to_string());
        entry.set_name(name.to_string());
        entry.set_timestamp(10003456);
        entry.compute_size();
        let merchant = convert_proto_to_merchant(data_type, &entry).unwrap();
        let result = merchant.merchant().unwrap();
        assert_eq!(&result.name, name);
    }
}
