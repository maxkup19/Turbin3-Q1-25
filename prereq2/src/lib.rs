mod programs;

#[cfg(test)]
mod tests {
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, pubkey::Pubkey, system_instruction::transfer, transaction::Transaction, message::Message};
    use solana_client::rpc_client::RpcClient;
    use std::str::FromStr;
    use solana_program::system_program;
    use crate::programs::turbin3_prereq::{WbaPrereqProgram, CompleteArgs};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!();
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        
        match client.request_airdrop(&keypair.pubkey(), 2000000000u64) {
            Ok(s) => {
            println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }
            Err(e) => {
                println!("Oops, something went wrong: {}", e);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("8e58JaFXqnxCNDxw3arKpdGpTyUSBnHeb42HvarLne9x").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Couldn't get balance");

        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Couldn't get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance,
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client.get_fee_for_message(&message).expect("Couldn't get fee");

        let transaction = Transaction::new_signed_with_payer(&[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance - fee,
        )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = WbaPrereqProgram::derive_program_address(
            &[b"prereq", signer.pubkey().to_bytes().as_ref()]
        );

        let args = CompleteArgs {
            github: b"maxkup19".to_vec(),
        };

        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Couldn't get recent blockhash");

        let transaction = WbaPrereqProgram::complete(
          &[&signer.pubkey(), &prereq, &system_program::id()],
          &args,
          Some(&signer.pubkey()),
          &[&signer],
          recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }
}
