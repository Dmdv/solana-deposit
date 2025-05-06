use {
    anyhow::Result,
    solana_client::rpc_client::RpcClient,
    solana_deposit_client::check_balance::{get_balance, print_balance},
};

fn main() -> Result<()> {
    let rpc_client = RpcClient::new("http://localhost:8899");

    // Deposit account pubkey from the previous run
    let deposit_account_pubkey = "Ctqpa8nH5cCbFmUh2f1HgqgPStSAVehwNMWzjKj4WFkz";

    // Get and print the balance
    let balance = get_balance(&rpc_client, deposit_account_pubkey)?;
    print_balance(balance);

    Ok(())
} 