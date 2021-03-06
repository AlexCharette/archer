use protobuf::parse_from_bytes;
use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext, TransactionHandler};

use super::payload::ArcherPayload;
use super::state::ArcherState;
use archer::get_archer_prefix;
use archer_protobuf::payload::{Payload as PayloadPB, Payload_Action};

pub struct ArcherTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

impl ArcherTransactionHandler {
    pub fn new(name: &str) -> ArcherTransactionHandler {
        ArcherTransactionHandler {
            family_name: String::from(name),
            family_versions: vec![String::from("1.0")],
            namespaces: vec![get_archer_prefix()],
        }
    }
}

impl TransactionHandler for ArcherTransactionHandler {
    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut dyn TransactionContext,
    ) -> Result<(), ApplyError> {
        let header = &request.header;
        let _signer = match &header.as_ref() {
            Some(s) => &s.signer_public_key,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Invalid header",
                )))
            }
        };

        let mut state = ArcherState::new(context);

        let payload = ArcherPayload::new(&request.payload)?;

        let data: PayloadPB =
            parse_from_bytes(&(payload.data()?)).expect("Error converting bytes to action");

        match payload.action() {
            Payload_Action::DEPOSIT => {
                state.update_balance(data.get_name(), data.get_number(), data.get_amount())?;
            }
            Payload_Action::WITHDRAW => {
                state.update_balance(data.get_name(), data.get_number(), -data.get_amount())?;
            }
            Payload_Action::UPDATE_NUMBER => {
                state.update_number(data.get_name(), data.get_number(), data.get_new_number())?;
            }
            Payload_Action::ADD_ACCOUNT => {
                state.set_account(data.get_name(), data.get_number())?;
            }
            Payload_Action::ADD_MERCHANT => {
                state.set_merchant(data.get_public_key(), data.get_name(), data.get_timestamp())?;
            }
        };
        Ok(())
    }

    fn family_name(&self) -> String {
        self.family_name.clone()
    }

    fn family_versions(&self) -> Vec<String> {
        self.family_versions.clone()
    }

    fn namespaces(&self) -> Vec<String> {
        self.namespaces.clone()
    }
}
