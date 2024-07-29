# Turbin3 Solana Enrollment Rust

```
__     ___      _ _ _       _
\ \   / (_)_ __(_|_) | __ _| | ___ __ _   _ _ __ ___
 \ \ / /| | '__| | | |/ _` | |/ / '__| | | | '_ ` _ \
  \ V / | | |  | | | | (_| |   <| |  | |_| | | | | | |
   \_/  |_|_| _/ |_|_|\__,_|_|\_\_|   \__,_|_| |_| |_|
              |__/
```
 
### Purpose:
This project is a Rust program developed to perform various operations on the Solana blockchain. The project allows users to create Solana wallets, receive SOL tokens to their wallets (airdrop), transfer these tokens to other wallets, and interact with certain programs.

### Main Functions:

#### Wallet Creation (keygen):
Creates a new Solana wallet and saves the private key of the wallet in the dev-wallet.json file in JSON format. This function allows the user to securely store the private and public key information of their wallet.

Base58 Conversions (byte_array_to_base58, base58_to_byte_array):
byte_array_to_base58: Converts a private key from a byte array format to base58 format. This conversion is used to securely store and share private keys.
base58_to_byte_array: Converts a private key in base58 format to a byte array format. This process makes the private key reusable.

#### Airdrop:
Sends SOL tokens to the user's wallet in a testnet or devnet environment. This provides the initial balance required for the user to be able to perform transactions on the Solana blockchain.

#### SOL Transfers (transfer_sol, transfer_all_sol):
transfer_sol: Sends a fixed amount of SOL tokens to a specific address. This function allows users to transfer their balances to other accounts.
transfer_all_sol: Sends the entire balance in the wallet to a specific address. This function allows users to transfer their entire balances to another wallet.

#### Interact with WbaPrereqProgram (complete):
Interacts with a special program called WbaPrereqProgram and completes a specific transaction. This function is designed for users who want to perform more advanced program interactions on Solana.

## Test Commands 
`cargo test ...`

## .gitignore
<img width="293" alt="gitignore" src="https://github.com/user-attachments/assets/0aec2c75-a542-4a44-a192-cab39a848b86">

## File Directory 
<img width="292" alt="file directory" src="https://github.com/user-attachments/assets/52a8ed8f-50fc-4201-b7b1-efc1fe0fe387">


## Transactions and Transaction Links

| Test Name | Transaction Description | Transaction Link |
|------------------------|-------------------------------------|-----------------------------------------------------------------------------------------------------------|
| `keygen` | Created a new Solana wallet and saved it to `dev-wallet.json` file. | N/A |
| `byte_array_to_base58` | Converted the secret key from byte array format to base58 format. | N/A |
| `base58_to_byte_array` | Converted the secret key from base58 format to byte array format. | N/A |
| `airdrop` | 2 SOL airdrops were made to the wallet. | [2SAKHeJt6gWwaaB8GwPnRTyyqMYZTpHpy9sCLHXCvdP9m3HxxLDtWgqoKc6TLU5EkpKVJt2ipowW9H98DHRdda7b](https://explorer.solana.com/tx/2SAKHeJt6gWwaaB8GwPnRTyyqMYZTpHpy9sCLHXCvdP9m3HxxLDtWgqoKc6TLU5EkpKVJt2ipowW9H98DHRdda7b?cluster=devnet) |
| `transfer_sol` | 1 SOL was transferred to another wallet. | [2HzJMe8XRWyziRgnCnP1JtVZKRQTWUjPRtajGaFg9UTp427AyZDJq36t4wocqeETs17ysyj4rJU4QvGG5pHoqrTa](https://explorer.solana.com/tx/2HzJMe8XRWyziRgnCnP1JtVZKRQTWUjPRtajGaFg9UTp427AyZDJq36t4wocqeETs17ysyj4rJU4QvGG5pHoqrTa?cluster=devnet) |
| `transfer_all_sol` | The entire wallet balance was transferred to another address. | [52MEmumRaq88PUmx3oKg4QnoTXZJsfNn83o7f9dnirQ72VMVYRzccGD3LygzR84zoRsC59CJKBDaZkyLhPkTu5iE](https://explorer.solana.com/tx/52MEmumRaq88PUmx3oKg4QnoTXZJsfNn83o7f9dnirQ72VMVYRzccGD3LygzR84zoRsC59CJKBDaZkyLhPkTu5iE?cluster=devnet) |
| `complete` | Completion was done with `WbaPrereqProgram`. | [5svWhHiqCUtSQmJ1LHdAiGUCvTg6mU8FFm3NPpjkag5QErvqNNgdHYv8JjwJEpDC1rQ97d1d7RFvsbUu89GRPk1n](https://explorer.solana.com/tx/5svWhHiqCUtSQmJ1LHdAiGUCvTg6mU8FFm3NPpjkag5QErvqNNgdHYv8JjwJEpDC1rQ97d1d7RFvsbUu89GRPk1n?cluster=devnet) |
