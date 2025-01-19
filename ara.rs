use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use spl_token::{
    instruction::{initialize_mint, mint_to},
    state::Mint,
};
use std::str::FromStr;

// Configuration Constants
const RPC_URL: &str = "https://api.mainnet-beta.solana.com"; // Solana Mainnet
const DECIMALS: u8 = 9;
const REWARD_AMOUNT: u64 = 100 * 10u64.pow(DECIMALS as u32);

// Struct for Compute Node
struct ComputeNode {
    pub id: Pubkey,
    pub stake: u64,
}

impl ComputeNode {
    fn new(id: Pubkey, stake: u64) -> Self {
        Self { id, stake }
    }
}

// Initialize Ara Token
fn initialize_ara_token(
    rpc_client: &RpcClient,
    admin_keypair: &Keypair,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let token_mint = Keypair::new();
    let token_mint_pubkey = token_mint.pubkey();

    // Create token mint
    let create_mint_ix = initialize_mint(
        &spl_token::id(),
        &token_mint_pubkey,
        &admin_keypair.pubkey(),
        None,
        DECIMALS,
    )?;
    let blockhash = rpc_client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[create_mint_ix],
        Some(&admin_keypair.pubkey()),
        &[admin_keypair, &token_mint],
        blockhash,
    );
    rpc_client.send_and_confirm_transaction(&tx)?;

    Ok(token_mint_pubkey)
}

// Reward Function
fn reward_node(
    rpc_client: &RpcClient,
    token_mint_pubkey: Pubkey,
    node_pubkey: Pubkey,
    admin_keypair: &Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    let node_account = spl_associated_token_account::get_associated_token_address(&node_pubkey, &token_mint_pubkey);

    // Mint tokens to the compute node
    let mint_to_ix = mint_to(
        &spl_token::id(),
        &token_mint_pubkey,
        &node_account,
        &admin_keypair.pubkey(),
        &[],
        REWARD_AMOUNT,
    )?;
    let blockhash = rpc_client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&admin_keypair.pubkey()),
        &[admin_keypair],
        blockhash,
    );
    rpc_client.send_and_confirm_transaction(&tx)?;

    println!("Rewarded {} tokens to {}", REWARD_AMOUNT, node_pubkey);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Solana client
    let rpc_client = RpcClient::new(RPC_URL.to_string());

    // Admin Keypair (replace with secure storage in production)
    let admin_keypair = Keypair::new();

    // Initialize Ara Token
    let ara_token_pubkey = initialize_ara_token(&rpc_client, &admin_keypair)?;
    println!("ARA Token Mint Address: {}", ara_token_pubkey);

    // Example Compute Node
    let node_keypair = Keypair::new();
    let compute_node = ComputeNode::new(node_keypair.pubkey(), 1000);

    // Reward the Compute Node
    reward_node(&rpc_client, ara_token_pubkey, compute_node.id, &admin_keypair)?;

    Ok(())
}
