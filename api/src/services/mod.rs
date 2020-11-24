use protobuf::{Message, RepeatedField};
// use sawtooth_sdk::messages::transaction::Transaction;
use sawtooth_sdk::messages::batch::{Batch, BatchList};
use sawtooth_sdk::signing::Signer;

use archer::{calculate_account_address, calculate_merchant_address};
use archer_protobuf::payload::{Payload as PayloadPB, Payload_Action};

pub mod batch;
pub mod transaction;

pub fn make_deposit_txn<'a>(
    txn_signer: &'a Signer,
    batch_signer: &'a Signer,
    name: String,
    number: u32,
    amount: i32,
) -> (Vec<u8>, String) {
    let address = calculate_account_address(&name);

    let mut payload_pb = PayloadPB::new();
    payload_pb.set_name(name);
    payload_pb.set_number(number);
    payload_pb.set_amount(amount);
    payload_pb.set_action(Payload_Action::DEPOSIT);

    let payload_bytes = payload_pb
        .write_to_bytes()
        .expect("Error converting protobuf payload to bytes");
    let inputs = vec![String::from(&address)];
    let outputs = vec![String::from(&address)];

    let batch = batch::make_batch(&payload_bytes, inputs, outputs, txn_signer, batch_signer);

    let batch_header_signature = batch.get_header_signature();

    let encoded_batches: Vec<u8> = encode_batches(batch.clone());

    (encoded_batches, batch_header_signature.to_string())
}

pub fn make_withdraw_txn<'a>(
    txn_signer: &'a Signer,
    batch_signer: &'a Signer,
    name: String,
    number: u32,
    amount: i32,
) -> (Vec<u8>, String) {
    let address = calculate_account_address(&name);

    let mut payload_pb = PayloadPB::new();
    payload_pb.set_name(name);
    payload_pb.set_number(number);
    payload_pb.set_amount(amount);
    payload_pb.set_action(Payload_Action::WITHDRAW);

    let payload_bytes = payload_pb
        .write_to_bytes()
        .expect("Error converting protobuf payload to bytes");
    let inputs = vec![String::from(&address)];
    let outputs = vec![String::from(&address)];

    let batch = batch::make_batch(&payload_bytes, inputs, outputs, txn_signer, batch_signer);

    let batch_header_signature = batch.get_header_signature();

    let encoded_batches: Vec<u8> = encode_batches(batch.clone());

    (encoded_batches, batch_header_signature.to_string())
}

pub fn make_add_account_txn<'a>(
    txn_signer: &'a Signer,
    batch_signer: &'a Signer,
    name: String,
    number: u32,
) -> (Vec<u8>, String) {
    let address = calculate_account_address(&name);

    let mut payload_pb = PayloadPB::new();
    payload_pb.set_name(name);
    payload_pb.set_number(number);
    payload_pb.set_action(Payload_Action::ADD_ACCOUNT);

    let payload_bytes = payload_pb
        .write_to_bytes()
        .expect("Error converting protobuf payload to bytes");
    let inputs = vec![String::from(&address)];
    let outputs = vec![String::from(&address)];

    let batch = batch::make_batch(&payload_bytes, inputs, outputs, txn_signer, batch_signer);

    let batch_header_signature = batch.get_header_signature();

    let encoded_batches: Vec<u8> = encode_batches(batch.clone());

    (encoded_batches, batch_header_signature.to_string())
}

pub fn make_add_merchant_txn<'a>(
    txn_signer: &'a Signer,
    batch_signer: &'a Signer,
    name: String,
    timestamp: i64,
) -> (Vec<u8>, String) {
    let address = calculate_merchant_address(
        &txn_signer
            .get_public_key()
            .expect("Could not get public key from transaction signer")
            .as_hex(),
    );

    let mut payload_pb = PayloadPB::new();
    payload_pb.set_name(name);
    payload_pb.set_timestamp(timestamp);
    payload_pb.set_action(Payload_Action::ADD_MERCHANT);

    let payload_bytes = payload_pb
        .write_to_bytes()
        .expect("Error converting protobuf payload to bytes");
    let inputs = vec![String::from(&address)];
    let outputs = vec![String::from(&address)];

    let batch = batch::make_batch(&payload_bytes, inputs, outputs, txn_signer, batch_signer);

    let batch_header_signature = batch.get_header_signature();

    let encoded_batches: Vec<u8> = encode_batches(batch.clone());

    (encoded_batches, batch_header_signature.to_string())
}

fn encode_batches<'a>(batch: Batch) -> Vec<u8> {
    let mut batch_list = BatchList::new();
    batch_list.set_batches(RepeatedField::from_vec(vec![batch]));
    batch_list
        .write_to_bytes()
        .expect("Error converting batch list to bytes")
}
