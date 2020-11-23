use openssl::sha::sha512;
use protobuf::RepeatedField;
use rand::{thread_rng, Rng};
use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::signing::Signer;

use archer::to_hex_string;

pub fn make_transaction(
    payload_bytes: &Vec<u8>,
    header_bytes: &Vec<u8>,
    signature: &str,
) -> Transaction {
    let mut transaction = Transaction::new();
    transaction.set_header(header_bytes.to_vec());
    transaction.set_header_signature(String::from(signature));
    transaction.set_payload(payload_bytes.clone());
    transaction
}

pub fn make_header(
    payload_bytes: &Vec<u8>,
    inputs: Vec<String>,
    outputs: Vec<String>,
    txn_signer: &Signer,
    batch_signer: &Signer,
) -> TransactionHeader {
    let mut header = TransactionHeader::new();
    let mut nonce = [0u8; 16];

    thread_rng()
        .try_fill(&mut nonce[..])
        .expect("Error generating random nonce");

    header.set_family_name(String::from("archer"));
    header.set_family_version(String::from("1.0"));

    header.set_nonce(to_hex_string(&nonce.to_vec()));
    header.set_inputs(RepeatedField::from_vec(inputs));
    header.set_outputs(RepeatedField::from_vec(outputs));

    header.set_signer_public_key(
        txn_signer
            .get_public_key()
            .expect("Error retrieving public key")
            .as_hex(),
    );

    header.set_batcher_public_key(
        batch_signer
            .get_public_key()
            .expect("Error retrieving public key")
            .as_hex(),
    );

    header.set_payload_sha512(to_hex_string(&sha512(payload_bytes).to_vec()));

    header
}
