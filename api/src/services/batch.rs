use protobuf::{Message, RepeatedField};
use sawtooth_sdk::messages::batch::{Batch, BatchHeader};
use sawtooth_sdk::messages::transaction::Transaction;
use sawtooth_sdk::signing::Signer;

use super::transaction;

pub fn make_batch<'a>(
    payload_bytes: &Vec<u8>,
    inputs: Vec<String>,
    outputs: Vec<String>,
    txn_signer: &'a Signer,
    batch_signer: &'a Signer,
) -> Batch {
    // create txn header
    let txn_header =
        transaction::make_header(&payload_bytes, inputs, outputs, txn_signer, batch_signer);
    let txn_header_bytes = txn_header
        .write_to_bytes()
        .expect("Error converting transaction header to bytes");

    // create txn
    let txn_header_signature = txn_signer
        .sign(&txn_header_bytes)
        .expect("Error signing transaction header bytes");
    let transaction =
        transaction::make_transaction(&payload_bytes, &txn_header_bytes, &txn_header_signature);
    let transactions = vec![&transaction];

    // create batch header
    let batch_header = make_header(&transactions, batch_signer);
    let batch_header_bytes = batch_header
        .write_to_bytes()
        .expect("Error converting batch header to bytes");

    let batch_header_signature = batch_signer
        .sign(&batch_header_bytes)
        .expect("Error signing batch header bytes");

    let mut batch = Batch::new();
    batch.set_header(batch_header_bytes.to_vec());
    batch.set_header_signature(batch_header_signature);
    batch.set_transactions(RepeatedField::from_vec(vec![transaction]));

    batch
}

pub fn make_header(transactions: &[&Transaction], signer: &Signer) -> BatchHeader {
    let mut batch_header = BatchHeader::new();
    let txn_ids = transactions
        .iter()
        .map(|txn| String::from(txn.get_header_signature()))
        .collect();

    batch_header.set_signer_public_key(
        signer
            .get_public_key()
            .expect("Error retrieving the public key")
            .as_hex(),
    );

    batch_header.set_transaction_ids(RepeatedField::from_vec(txn_ids));

    batch_header
}
