use anyhow::Result;
use bs58;
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::VersionedTransaction,
};

pub fn sign_transactions(
    serialized_txs: &Vec<String>,
    signers: &Vec<Keypair>,
) -> Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    for serialized_tx in serialized_txs.iter() {
        // Deserialize transaction
        let tx_buffer = bs58::decode(serialized_tx).into_vec().unwrap();
        let mut transaction = match bincode::deserialize::<VersionedTransaction>(&tx_buffer) {
            Ok(transaction) => transaction,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to deserialize transaction: {:?}",
                    e
                ));
            }
        };

        for (index, account_key) in transaction.message.static_account_keys().iter().enumerate() {
            if transaction.message.is_signer(index) {
                let sign_required = transaction.signatures[index] == Signature::default();
                if sign_required {
                    let signer = signers.iter().find(|kp| kp.pubkey().eq(&account_key));

                    if let Some(signer) = signer {
                        transaction.signatures[index] =
                            signer.sign_message(&transaction.message.serialize());
                    } else {
                        return Err(anyhow::anyhow!(
                            "Signer not found for account key: {:?}",
                            account_key.to_string()
                        ));
                    }
                }
            }
        }
        let tx_serialized = match bincode::serialize(&transaction) {
            Ok(serialized) => serialized,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to serialize transaction: {:?}", e));
            }
        };
        res.push(bs58::encode(tx_serialized).into_string());
    }
    Ok(res)
}
