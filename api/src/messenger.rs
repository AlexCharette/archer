use super::services::{
    make_add_account_txn, make_add_merchant_txn, make_deposit_txn, make_withdraw_txn,
};
use log::{error, info};
use protobuf::parse_from_bytes;
use reqwest::Client;
use sawtooth_sdk::messages::client_batch_submit::{
    ClientBatchStatusResponse, ClientBatchStatus_InvalidTransaction, ClientBatchStatus_Status,
};
use sawtooth_sdk::signing::{
    create_context, secp256k1, Context, CryptoFactory, PrivateKey, Signer,
};
use std::sync::Arc;

pub struct Messenger<'a> {
    client: Client,
    context: Arc<Box<dyn Context + 'a>>,
    batch_private_key: Arc<Box<dyn PrivateKey + 'a>>,
}

impl Messenger<'_> {
    pub fn new<'a>(algo: &'a str) -> Self {
        let client: Client = Client::new();
        let context = create_context(algo).expect("Error creating the right context");
        let batch_private_key: Box<dyn PrivateKey> = context
            .new_random_private_key()
            .ok()
            .expect("Error creating new random private key");

        Messenger {
            client: client,
            context: Arc::new(context),
            batch_private_key: Arc::new(batch_private_key),
        }
    }

    pub fn get_new_key_pair(&self) -> (String, String) {
        info!("Generating new private and public key pair");

        let private_key = self
            .context
            .new_random_private_key()
            .expect("Error generating a private key");
        let public_key = self
            .context
            .get_public_key(&*private_key)
            .ok()
            .expect("Error getting public key");
        (public_key.as_hex(), private_key.as_hex())
    }

    pub async fn send_deposit_txn(
        &self,
        private_key: &str,
        name: String,
        number: u32,
        amount: i32,
    ) {
        let crypto_factory: CryptoFactory = CryptoFactory::new(&**self.context);

        let secp_private_key = secp256k1::Secp256k1PrivateKey::from_hex(private_key)
            .ok()
            .expect("Error generating secp256k1 private key from hex");
        let transaction_signer: Signer = crypto_factory.new_signer(&secp_private_key);

        let batch_signer: Signer = crypto_factory.new_signer(&**self.batch_private_key);
        let (encoded_batches, batch_header_signature): (Vec<u8>, String) =
            make_deposit_txn(&transaction_signer, &batch_signer, name, number, amount);

        info!("Sending encoded batches");

        self.send_and_wait(encoded_batches, batch_header_signature)
            .await;
    }

    pub async fn send_withdraw_txn(
        &self,
        private_key: &str,
        name: String,
        number: u32,
        amount: i32,
    ) {
        let crypto_factory: CryptoFactory = CryptoFactory::new(&**self.context);

        let secp_private_key = secp256k1::Secp256k1PrivateKey::from_hex(private_key)
            .ok()
            .expect("Error generating secp256k1 private key from hex");
        let transaction_signer: Signer = crypto_factory.new_signer(&secp_private_key);

        let batch_signer: Signer = crypto_factory.new_signer(&**self.batch_private_key);
        let (encoded_batches, batch_header_signature): (Vec<u8>, String) =
            make_withdraw_txn(&transaction_signer, &batch_signer, name, number, amount);

        info!("Sending encoded batches");

        self.send_and_wait(encoded_batches, batch_header_signature)
            .await;
    }

    pub async fn send_add_account_txn(&self, private_key: &str, name: String, number: u32) {
        let crypto_factory: CryptoFactory = CryptoFactory::new(&**self.context);

        let secp_private_key = secp256k1::Secp256k1PrivateKey::from_hex(private_key)
            .ok()
            .expect("Error generating secp256k1 private key from hex");
        let transaction_signer: Signer = crypto_factory.new_signer(&secp_private_key);

        let batch_signer: Signer = crypto_factory.new_signer(&**self.batch_private_key);
        let (encoded_batches, batch_header_signature): (Vec<u8>, String) =
            make_add_account_txn(&transaction_signer, &batch_signer, name, number);

        info!("Sending encoded batches");

        self.send_and_wait(encoded_batches, batch_header_signature)
            .await;
    }

    pub async fn send_add_merchant_txn(&self, private_key: &str, name: String, timestamp: i64) {
        let crypto_factory: CryptoFactory = CryptoFactory::new(&**self.context);

        let secp_private_key = secp256k1::Secp256k1PrivateKey::from_hex(private_key)
            .ok()
            .expect("Error generating secp256k1 private key from hex");
        let transaction_signer: Signer = crypto_factory.new_signer(&secp_private_key);

        let batch_signer: Signer = crypto_factory.new_signer(&**self.batch_private_key);
        let (encoded_batches, batch_header_signature): (Vec<u8>, String) =
            make_add_merchant_txn(&transaction_signer, &batch_signer, name, timestamp);

        info!("Sending encoded batches");

        self.send_and_wait(encoded_batches, batch_header_signature)
            .await;
    }

    pub async fn send_and_wait<'a>(&self, batches: Vec<u8>, batch_id: String) {
        self.client
            .post("http://localhost:8008/batches")
            .header("Content-Type", "application/octet-stream")
            .body(batches)
            .send()
            .await
            .ok();

        // let batch_id_copy: String = batch_id.clone();

        info!("Obtaining validator response");

        let validator_response: Result<reqwest::Response, reqwest::Error> = self
            .client
            .post("http://localhost:8008/batch_statuses")
            .header("Content-Type", "application/octet-stream")
            .body(batch_id)
            .send()
            .await;

        let validator_response: String = validator_response
            .ok()
            .unwrap()
            .text()
            .await
            .ok()
            .expect("Error getting validator response content");

        let status_response: ClientBatchStatusResponse =
            parse_from_bytes(&validator_response.into_bytes())
                .ok()
                .expect("Error converting validator response into bytes");

        let status: ClientBatchStatus_Status = status_response.batch_statuses[0].status;

        match status {
            ClientBatchStatus_Status::STATUS_UNSET => info!("Status has not yet been set"),
            ClientBatchStatus_Status::COMMITTED => info!("Batch committed"),
            ClientBatchStatus_Status::INVALID => {
                let error: &ClientBatchStatus_InvalidTransaction =
                    &status_response.batch_statuses[0].invalid_transactions[0];
                error!("{:?}", error.message)
            }
            ClientBatchStatus_Status::PENDING => info!("Transaction submitted but timed out"),
            ClientBatchStatus_Status::UNKNOWN => error!("Something went wrong, try again later"),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_key_pair_generation() {}

    #[test]
    fn test_send_transaction() {}

    #[test]
    fn test_send_and_wait() {}
}
