use solana_sdk::{
    client::RpcClient,
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction,
};

fn reward_contributor(
    client: &RpcClient,
    contributor_pubkey: &Pubkey,
    amount: u64,
    payer: &Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate sending a reward to a contributor
    let reward_transaction = system_instruction::transfer(
        &payer.pubkey(),
        contributor_pubkey,
        amount,
    );

    let recent_blockhash = client.get_recent_blockhash()?.0;
    let transaction = Transaction::new_signed_with_payer(
        &[reward_transaction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    client.send_and_confirm_transaction(&transaction)?;
    Ok(())
}

// Example usage
fn main() {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let payer = Keypair::from_base58_string("<your_private_key>");
    let contributor_pubkey = Pubkey::from_str("<contributor_public_key>").unwrap();

    let reward_amount = 1000; // Example token amount for reward

    match reward_contributor(&client, &contributor_pubkey, reward_amount, &payer) {
        Ok(_) => println!("Successfully rewarded the contributor!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
