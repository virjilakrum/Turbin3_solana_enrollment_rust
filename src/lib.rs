mod programs;

/*
Declares a module named programs, which is presumably defined elsewhere in the project.
This module likely contains the implementation of the WbaPrereqProgram.
*/

#[cfg(test)]

/*
Defines a module for unit tests.
The #[cfg(test)] attribute ensures that this module is only compiled when running tests.
*/

mod tests {
    use crate::programs::wba_prereq::{CompleteArgs, WbaPrereqProgram};
    use solana_client::rpc_client::RpcClient;

    /*
    The RPC client for interacting with the Solana blockchain.
    */

    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    //import pubkey
    use solana_sdk::{
        message::Message,
        native_token::LAMPORTS_PER_SOL,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
        transaction::Transaction,
        /*
        Message: Represents a message in a Solana transaction.
        LAMPORTS_PER_SOL: A constant defining the number of lamports in one SOL token.
        read_keypair_file, Keypair, Signer: Utilities for handling keypairs.
        system_program: Represents the system program's public key.
        Transaction: Represents a transaction on the Solana blockchain.
        */
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";
    // URL for the Solana devnet RPC endpoint.
    const MY_WBA_WALLET_ADDRESS: &str = "HWkiywmVgVmVzg3JtHivLrTrQrrKjMaP7mR8QKk84F7b";
    // A predefined public key address as a string.
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("Generated Sol Wallet with Pub key: {}", kp.pubkey());
        println!("Copy for security:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("./wallets/dev-wallet.json")
            .expect("Couldn't find dev-wallet.json file");
        let client = RpcClient::new(DEVNET_RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2 * LAMPORTS_PER_SOL) {
            Ok(signature) => {
                println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
                    signature
                );
            }
            Err(e) => println!("404: {}", e),
        };
    }

    #[test]
    fn transfer_sol() {
        let keypair =
            read_keypair_file("./wallets/dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str(MY_WBA_WALLET_ADDRESS).unwrap();
        let client = RpcClient::new(DEVNET_RPC_URL);

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let ix = transfer(&keypair.pubkey(), &to_pubkey, LAMPORTS_PER_SOL / 10);

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn transfer_all_sol() {
        let keypair =
            read_keypair_file("./wallets/dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str(MY_WBA_WALLET_ADDRESS).unwrap();
        let client = RpcClient::new(DEVNET_RPC_URL);

        let from_balance = client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let mock_ix = transfer(&keypair.pubkey(), &to_pubkey, from_balance);
        let message =
            Message::new_with_blockhash(&[mock_ix], Some(&keypair.pubkey()), &recent_blockhash);
        let fee = client
            .get_fee_for_message(&message)
            .expect("Failed to get fee");

        let ix = transfer(&keypair.pubkey(), &to_pubkey, from_balance - fee);

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn complete() {
        let signer =
            read_keypair_file("./wallets/my-wba-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(DEVNET_RPC_URL);

        let pda_pubkey = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        let args = CompleteArgs {
            github: b"virjilakrum".to_vec(),
        };

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let tx = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &pda_pubkey, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&tx)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_byte_array() {
        println!("Enter your secret key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();

        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("Your secret key as byte array:");
        println!("{:?}", wallet);
    }

    #[test]
    fn byte_array_to_base58() {
        println!("Enter your secret key as byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let base58 = bs58::encode(wallet).into_string();
        println!("Your secret key as base58:");
        println!("{}", base58);
    }
}
