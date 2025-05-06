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
    std::str::FromStr,
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
    // Connect to localhost validator
    let rpc_client = RpcClient::new("http://localhost:8899");

    // Load keypair from file (you'll need to create this)
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Failed to read keypair file");
    println!("Payer pubkey: {}", payer.pubkey());

    // Program ID (replace with your deployed program ID)
    let program_id = Pubkey::from_str("4g5NvZ3fjfn46KjJ5ZLCfABp7JfozidespyFNwbBbThQ")?;
    println!("Program ID: {}", program_id);

    // Create a new deposit account
    let deposit_account = Keypair::new();
    println!("Deposit account pubkey: {}", deposit_account.pubkey());

    // Initialize the deposit account
    let initialize_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(deposit_account.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: DepositInstruction::Initialize.try_to_vec()?,
    };
    println!("System Program ID: {}", system_program::id());

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    println!("Recent blockhash: {}", recent_blockhash);

    let initialize_tx = Transaction::new_signed_with_payer(
        &[initialize_ix],
        Some(&payer.pubkey()),
        &[&payer, &deposit_account],
        recent_blockhash,
    );

    rpc_client.send_and_confirm_transaction(&initialize_tx)?;
    println!("Initialized deposit account");

    // Deposit 1 SOL
    let deposit_amount = 1_000_000_000; // 1 SOL in lamports
    let deposit_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(deposit_account.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: DepositInstruction::Deposit { amount: deposit_amount }.try_to_vec()?,
    };

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let deposit_tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&payer.pubkey()),
        &[&payer, &deposit_account],
        recent_blockhash,
    );

    rpc_client.send_and_confirm_transaction(&deposit_tx)?;
    println!("Deposited 1 SOL");

    // Get account data
    let account_data = rpc_client.get_account_data(&deposit_account.pubkey())?;
    let deposit_data = DepositAccount::try_from_slice(&account_data)?;
    println!("Current balance: {} lamports", deposit_data.balance);

    // Withdraw 0.5 SOL
    let withdraw_amount = 500_000_000; // 0.5 SOL in lamports
    let withdraw_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(deposit_account.pubkey(), false),
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
    println!("Withdrew 0.5 SOL");

    // Get final account data
    let account_data = rpc_client.get_account_data(&deposit_account.pubkey())?;
    let deposit_data = DepositAccount::try_from_slice(&account_data)?;
    println!("Final balance: {} lamports", deposit_data.balance);

    Ok(())
}
