# Solana Deposit Program
 
Smart contract (program) that allows users to deposit SOL, track their balances, and withdraw funds.

## Instructions

1. Build the program
2. Build the client
3. Start validator
4. Deploy the program
5. Start the client

```bash
solana-test-validator --reset
solana config set --url localhost
solana program-v4 deploy target/deploy/solana_deposit_program.so --program-keypair program-keypair.json
```

## Building the Program

1. Build the program:
```bash
cd program
cargo build-sbf
```

2. Deploy the program to your chosen network (localnet, devnet, or mainnet):
```bash
solana program-v4 deploy target/deploy/solana_deposit_program.so --program-keypair program-keypair.json
```

Note the program ID after deployment and update it in the client code.

## Running the Client

1. Make sure you have a Solana keypair file at `wallet/id.json` (included in repository for testing)
2. Update the program ID in `client/src/main.rs` with your deployed program ID
3. Run the client:
```bash
cd client
cargo run
```

## Wallet Configuration

Current wallet public key: `EMmbbKuV6vghd2oBgEV5eWmR6CBrWgRZyB7aWeWtajeV`

To use this wallet:
1. Make sure you have Solana CLI installed
2. Use the provided test wallet or create your own:
```bash
# Use provided test wallet
cp wallet/id.json ~/.config/solana/id.json

# Or generate new wallet
solana-keygen new --outfile ~/.config/solana/id.json

# Or import existing wallet (replace with your private key)
# solana-keygen recover -o ~/.config/solana/id.json
```

## Checking Balance

There are two ways to check the balance:

1. Using Solana CLI:
```bash
solana balance <ACCOUNT_ADDRESS>
```

2. Using our custom utility:
```bash
cd client
cargo run --bin check_balance
```

Example for current deposit account:
```bash
# Using Solana CLI
solana balance Ctqpa8nH5cCbFmUh2f1HgqgPStSAVehwNMWzjKj4WFkz

# Using custom utility
cargo run --bin check_balance
```

## Important Addresses

- Program ID: `CmtwqjuwoTREErtzfD8Q3QY7caopyd6k5TmYW48xXFRA`
- Wallet Address: `EMmbbKuV6vghd2oBgEV5eWmR6CBrWgRZyB7aWeWtajeV`
- Current Deposit Account: `Ctqpa8nH5cCbFmUh2f1HgqgPStSAVehwNMWzjKj4WFkz`

![img.png](img.png)