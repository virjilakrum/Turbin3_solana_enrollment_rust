use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
use std::fs::File;
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        let keypair_bytes = kp.to_bytes();
        let json = serde_json::to_string(&keypair_bytes).unwrap();
        let mut file = File::create("dev-wallet.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
