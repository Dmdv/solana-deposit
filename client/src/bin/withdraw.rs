use {
    anyhow::Result,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_client::rpc_client::RpcClient,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    solana_sdk::{
        signature::{Keypair, read_keypair_file},
        signer::Signer,
        transaction::Transaction,
    },
    std::{path::PathBuf, str::FromStr},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum DepositInstruction {
    Initialize,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DepositAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

fn main() -> Result<()> {
    let rpc_client = RpcClient::new("http://localhost:8899");

    // Load keypair from project's wallet directory
    let wallet_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wallet")
        .join("id.json");
    
    let payer = read_keypair_file(wallet_path)
        .expect("Failed to read keypair file");
    println!("Payer pubkey: {}", payer.pubkey());

    // Program ID (replace with your deployed program ID)
    let program_id = Pubkey::from_str("CmtwqjuwoTREErtzfD8Q3QY7caopyd6k5TmYW48xXFRA")?;
    println!("Program ID: {}", program_id);

    // Deposit account pubkey (replace with your deposit account)
    let deposit_account_pubkey = Pubkey::from_str("AWSmjmFAnyp25Jj5woJseP8HPybxVsvnp5Dgz8GTN1tq")?;
    println!("Deposit account: {}", deposit_account_pubkey);

    // Get current balance
    let account_data = rpc_client.get_account_data(&deposit_account_pubkey)?;
    let deposit_data = DepositAccount::try_from_slice(&account_data)?;
    println!("Current balance: {} lamports ({} SOL)", 
        deposit_data.balance,
        deposit_data.balance as f64 / 1_000_000_000.0
    );

    // Withdraw 0.5 SOL
    let withdraw_amount = 500_000_000; // 0.5 SOL in lamports
    println!("Withdrawing {} SOL", withdraw_amount as f64 / 1_000_000_000.0);

    let withdraw_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(deposit_account_pubkey, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: DepositInstruction::Withdraw { amount: withdraw_amount }.try_to_vec()?,
    };

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let withdraw_tx = Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    rpc_client.send_and_confirm_transaction(&withdraw_tx)?;
    println!("Successfully withdrew {} SOL", withdraw_amount as f64 / 1_000_000_000.0);

    // Get final balance
    let account_data = rpc_client.get_account_data(&deposit_account_pubkey)?;
    let deposit_data = DepositAccount::try_from_slice(&account_data)?;
    println!("Final balance: {} lamports ({} SOL)", 
        deposit_data.balance,
        deposit_data.balance as f64 / 1_000_000_000.0
    );

    Ok(())
} 