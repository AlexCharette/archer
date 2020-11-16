
use crypto::sha2::Sha512;
use crypto::digest::Digest; 

pub mod error;

pub const NAME: &str = "archer";

pub struct Account { 
    pub name: String, 
    pub number: u32, 
    pub balance: i32, 
    pub start_block_num: Option<i64>, 
    pub end_block_num: Option<i64>, 
}

pub struct Merchant {
    pub name: String,
}

#[derive(Eq, Hash, PartialEq)]
pub enum ArcherModules {
    RestApi,
    Processor,
    Subscriber,
    Sawtooth,
}

pub enum ArcherStructs {
    Account(Account),
    Merchant(Merchant),
}

impl ArcherStructs {
    pub fn account(self) -> Option<Account> {
        match self {
            ArcherStructs::Account(account) => Some(account),
            ArcherStructs::Merchant(account) => None,    
        }
    }

    pub fn merchant(self) -> Option<Merchant> {
        match self {
            ArcherStructs::Merchant(merchant) => Some(merchant),
            ArcherStructs::Account(merchant) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArcherTypes {
    Account,
    Merchant,
}

pub fn get_archer_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str(NAME);
    sha.result_str()[..6].to_string()
}

pub fn get_type_prefix(archer_type: &ArcherTypes) -> Option<String> {
    match archer_type {
        ArcherTypes::Account => Some(String::from("00")),
        ArcherTypes::Merchant => Some(String::from("01")),
    }
}

pub fn calculate_account_address(name: &str) -> String {
    let mut sha = Sha512::new();
    sha.input_str(name);
    let mut prefix = get_archer_prefix();
    prefix.push_str(&get_type_prefix(&ArcherTypes::Account).expect("Invalid archer type"));
    prefix + &sha.result_str()[..62].to_string()
}

pub fn calculate_merchant_address(public_key: &str) -> String {
    let mut sha = Sha512::new();
    sha.input_str(public_key);
    let mut prefix = get_archer_prefix();
    prefix.push_str(&get_type_prefix(&ArcherTypes::Merchant).expect("Invalid archer type"));
    prefix + &sha.result_str()[..62].to_string()
}

pub fn get_address_type(address: &str) -> Option<ArcherTypes> {
    match &address[6..8] {
        "00" => Some(ArcherTypes::Account),
        "01" => Some(ArcherTypes::Merchant),
        _ => None,
    }
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strings: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    strings.join("")
}

pub unsafe fn any_as_u8_slice<T: Sized>(param: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (param as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

#[cfg(test)]
mod test {
    use crate::archer::*;
    
    #[test]
    fn test_archer_prefix() {}

    #[test]
    fn test_type_prefix() {
        assert_eq!(get_type_prefix(&ArcherTypes::Account).unwrap(), String::from("00"));
    }

    #[test]
    fn test_account_address() {
        // create public key
    }

    #[test]
    fn test_address_type() {
        assert_eq!(get_address_type("12345600").unwrap(), ArcherTypes::Account);
        assert_eq!(get_address_type("address1234"), None);
    }

    #[test]
    fn test_to_hex_string() {}

    #[test]
    fn test_as_u8_slice() {}
}