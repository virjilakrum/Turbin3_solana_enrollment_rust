use base64::encode;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::fs::File;
use std::io::Write;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{system_instruction::transfer, transaction::Transaction};
    use std::str::FromStr;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        let keypair_bytes = kp.to_bytes();
        let json = encode(&keypair_bytes);
        let mut file = File::create("dev-wallet.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("<your WBA public key>").unwrap();
        let client = RpcClient::new(RPC_URL);
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
