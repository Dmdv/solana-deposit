use {
    anyhow::Result,
    solana_client::rpc_client::RpcClient,
    solana_deposit_client::check_balance::{get_balance, print_balance},
};

fn main() -> Result<()> {
    // Connect to localhost validator
    let rpc_client = RpcClient::new("http://localhost:8899");

    // Deposit account pubkey from the previous run
    let deposit_account_pubkey = "CfybBwktjRTQ3ZicJVk9Y7gF9rynY3egtucEc1Zy66ws";

    // Get and print the balance
    let balance = get_balance(&rpc_client, deposit_account_pubkey)?;
    print_balance(balance);

    Ok(())
} 