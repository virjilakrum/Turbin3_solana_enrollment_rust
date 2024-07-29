use serde::{Deserialize, Serialize}; // Serde modüllerini ekleyin
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::fs::File;
use std::io::{Read, Write};

mod programs;

#[derive(Serialize, Deserialize, Debug)] // Debug trait'i ekleyin
struct KeypairJson {
    keypair: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs::wba_prereq::{CompleteArgs, WbaPrereqProgram}; // Eksik importlar
    use solana_sdk::message::Message; // Eksik import
    use solana_sdk::system_program;
    use solana_sdk::{system_instruction::transfer, transaction::Transaction};
    use std::str::FromStr; // Eksik import

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        let pubkey = kp.pubkey().to_string();
        println!("You've generated a new Solana wallet: {}", pubkey);
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");

        let keypair_bytes = kp.to_bytes();
        let keypair_json = KeypairJson {
            keypair: keypair_bytes.to_vec(),
        };
        let json = serde_json::to_string(&keypair_json).unwrap();
        let mut file = File::create("dev-wallet.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();

        println!("{:?}", keypair_json);
    }

    #[test]
    fn airdrop() {
        // Dosyadan keypair'i okuyun
        let mut file = File::open("dev-wallet.json").expect("Couldn't find wallet file");
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        let keypair_json: KeypairJson = serde_json::from_str(&json_str).unwrap();
        let keypair = Keypair::from_bytes(&keypair_json.keypair).unwrap();

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
        let mut file = File::open("dev-wallet.json").expect("Couldn't find wallet file");
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        let keypair_json: KeypairJson = serde_json::from_str(&json_str).unwrap();
        let keypair = Keypair::from_bytes(&keypair_json.keypair).unwrap();

        let to_pubkey = Pubkey::from_str("HWkiywmVgVmVzg3JtHivLrTrQrrKjMaP7mR8QKk84F7b").unwrap();
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

    #[test]
    fn transfer_sol_balance() {
        let mut file = File::open("dev-wallet.json").expect("Couldn't find wallet file");
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        let keypair_json: KeypairJson = serde_json::from_str(&json_str).unwrap();
        let keypair = Keypair::from_bytes(&keypair_json.keypair).unwrap();

        let to_pubkey = Pubkey::from_str("CcVWtVfQ3944MuNXbEXrkacqwtuy5uccZ5XydUKgyiE5").unwrap();
        let client = RpcClient::new(RPC_URL);
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let balance = client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
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

    #[test]
    fn consume_idl() {
        let client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet");

        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"0xBitzz".to_vec(),
        };

        let blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
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
