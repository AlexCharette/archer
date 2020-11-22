use crate::protobuf::account::{Account as AccountPB, AccountContainer};
use crate::protobuf::merchant::{Merchant as MerchantPB, MerchantContainer};
use protobuf::{parse_from_bytes, Message};
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext};
use std::collections::HashMap;

use crate::archer::{calculate_account_address, calculate_merchant_address};

// TODO!! switch to address map (VERIFY?????)

pub struct ArcherState<'a> {
    context: &'a mut dyn TransactionContext,
    address_map: HashMap<String, Option<String>>,
}

impl<'a> ArcherState<'a> {
    pub fn new(context: &'a mut dyn TransactionContext) -> ArcherState {
        ArcherState {
            context: context,
            address_map: HashMap::new(),
        }
    }

    /*
        fn _store_game(
        &mut self,
        game_name: &str,
        games: HashMap<String, Game>,
    ) -> Result<(), ApplyError> {
        let address = XoState::calculate_address(game_name);
        let state_string = Game::serialize_games(games);
        self.address_map
            .insert(address.clone(), Some(state_string.clone()));
        self.context
            .set_state(&address, &state_string.into_bytes())?;
        Ok(())

     def set_agent(self, public_key, name, timestamp):
        address = addresser.get_agent_address(public_key)
        agent = agent_pb2.Agent(
            public_key=public_key, name=name, timestamp=timestamp)
        container = agent_pb2.AgentContainer()
        state_entries = self._context.get_state(
            addresses=[address], timeout=self._timeout)
        if state_entries:
            container.ParseFromString(state_entries[0].data)

        container.entries.extend([agent])
        data = container.SerializeToString()

        updated_state = {}
        updated_state[address] = data
        self._context.set_state(updated_state, timeout=self._timeout)
    */
    pub fn set_account(&mut self, name: &str, number: u32) -> Result<(), ApplyError> {
        let address: String = calculate_account_address(name);
        let mut account: AccountPB = AccountPB::new();
        account.set_name(String::from(name));
        account.set_number(number);
        account.set_balance(0);

        // TODO set state

        // let container: AccountContainer = parse_from_bytes();

        // let state_string =

        // self.address_map.insert(address.clone(), Some())
        // self.context.set_state_entry(address, data)
        Ok(())
    }

    // TODO implement
    pub fn set_merchant(&mut self, public_key: &str, name: &str) -> Result<(), ApplyError> {
        let address: String = calculate_merchant_address(public_key);
        let mut merchant: MerchantPB = MerchantPB::new();
        merchant.set_name(String::from(name));

        // TODO set state

        // let container: AccountContainer = parse_from_bytes();

        // let state_string =

        // self.address_map.insert(address.clone(), Some())
        // self.context.set_state_entry(address, data)
        Ok(())
    }

    pub fn update_number(
        &mut self,
        name: &str,
        number: u32,
        new_number: u32,
    ) -> Result<u32, ApplyError> {
        let address: String = calculate_account_address(name);
        let state_entries = self
            .context
            .get_state_entries(&vec![String::from(address.clone())]);
        match state_entries {
            Ok(entries) => {
                let container: AccountContainer =
                    parse_from_bytes(&entries[0].1).expect("Error parsing state entries");
                let mut accounts = container.get_entries().to_vec();
                let account: &mut AccountPB = accounts
                    .iter_mut()
                    .filter(|entry| entry.get_name() == name && entry.get_number() == number)
                    .next()
                    .expect("Did not found accounts with that name");
                account.set_number(new_number);
                Ok(account.get_number())
            }
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(format!(
                    "Account not found for {}",
                    address
                ))))
            }
        }
    }

    pub fn get_balance(&mut self, name: &str, number: u32) -> Result<i32, ApplyError> {
        let address: String = calculate_account_address(name);
        let state_entries = self
            .context
            .get_state_entries(&vec![String::from(address.clone())]);
        match state_entries {
            Ok(entries) => {
                let container: AccountContainer =
                    parse_from_bytes(&entries[0].1).expect("Error parsing state entries");
                let account: &AccountPB = container
                    .entries
                    .iter()
                    .filter(|entry| entry.get_name() == name && entry.get_number() == number)
                    .next()
                    .expect("Did not found accounts with that name");
                Ok(account.get_balance())
            }
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(format!(
                    "Account not found for {}",
                    address
                ))))
            }
        }
    }

    pub fn update_balance(
        &mut self,
        name: &str,
        number: u32,
        amount: i32,
    ) -> Result<i32, ApplyError> {
        let address: String = calculate_account_address(name);
        let updated = self._update_balance(&address, name, number, amount);
        match updated {
            Ok(_) => self.get_balance(name, number),
            Err(err) => Err(err),
        }
    }

    fn _update_balance(
        &mut self,
        address: &str,
        name: &str,
        number: u32,
        amount: i32,
    ) -> Result<(), ApplyError> {
        let state_entries = self.context.get_state_entries(&vec![String::from(address)]);
        match state_entries {
            Ok(entries) => {
                let container: AccountContainer =
                    parse_from_bytes(&entries[0].1).expect("Error parsing state entries");
                let mut accounts = container.get_entries().to_vec();
                let account: &mut AccountPB = accounts
                    .iter_mut()
                    .filter(|entry| entry.get_name() == name && entry.get_number() == number)
                    .next()
                    .expect("Did not found accounts with that name");

                let balance = account.get_balance();
                // If the amount is negative, check if balance + amount < 0
                if amount < 0 && balance + amount >= 0 {
                    account.set_balance(balance + amount);
                } else if amount >= 0 {
                    account.set_balance(balance + amount);
                } else {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Invalid withdrawal amount",
                    )));
                }
                // data = container.SerializeToString()
                // updated_state = {}
                // updated_state[address] = data
                // self._context.set_state(updated_state, timeout=self._timeout)
                let data = container
                    .write_to_bytes()
                    .expect("Could not serialize container");
                self.context.set_state_entry(String::from(address), data)?;
                Ok(())
            }
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(format!(
                    "Account not found for {}",
                    address
                ))))
            }
        }
    }

    // fn _load_accounts(&mut self, name: &str) -> Result<HashMap<String, AccountPB>, ApplyError> {
    //     let address: String = calculate_account_address(name);
    //     let mut accounts = HashMap::new();

    //     if self.address_map.contains_key(&address) {
    //         if let Some(ref serialized_games) = self.address_map[&address] {

    //         }
    //     }
    // }

    /*
    def get_agent(self, public_key):
        """Gets the agent associated with the public_key

        Args:
            public_key (str): The public key of the agent

        Returns:
            agent_pb2.Agent: Agent with the provided public_key
        """
        address = addresser.get_agent_address(public_key)
        state_entries = self._context.get_state(
            addresses=[address], timeout=self._timeout)
        if state_entries:
            container = agent_pb2.AgentContainer()
            container.ParseFromString(state_entries[0].data)
            for agent in container.entries:
                if agent.public_key == public_key:
                    return agent

        return None
    */
}
