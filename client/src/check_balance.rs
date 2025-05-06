use {
    anyhow::Result,
    borsh::BorshDeserialize,
    solana_client::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    std::str::FromStr,
};

#[derive(BorshDeserialize, Debug)]
pub struct DepositAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

pub fn get_balance(
    rpc_client: &RpcClient,
    deposit_account_pubkey: &str,
) -> Result<u64> {
    let pubkey = Pubkey::from_str(deposit_account_pubkey)?;
    let account_data = rpc_client.get_account_data(&pubkey)?;
    let deposit_data = DepositAccount::try_from_slice(&account_data)?;
    Ok(deposit_data.balance)
}

pub fn print_balance(balance: u64) {
    println!("Current balance: {} lamports ({} SOL)", 
        balance, 
        balance as f64 / 1_000_000_000.0
    );
} 