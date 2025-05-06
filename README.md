# Solana Deposit Program

Создать простое приложение на Rust, взаимодействующее с Solana. Нужно разработать смарт-контракт (программу), который позволяет пользователям вносить депозиты в SOL, отслеживать их баланс и выводить средства.
контракты можно тестировать в Localnet или Devnet, чтобы не требовались деньги на тесты

## Building the Program

1. Build the program:
```bash
cd program
cargo build-sbf
```

2. Deploy the program to your chosen network (localnet, devnet, or mainnet):
```bash
solana program deploy target/sbf-solana-solana/release/solana_deposit_program.so
```

Note the program ID after deployment and update it in the client code.

## Running the Client

1. Make sure you have a Solana keypair file at `~/.config/solana/id.json`
2. Update the program ID in `client/src/main.rs` with your deployed program ID
3. Run the client:
```bash
cd client
cargo run
```

## Program Features

- Initialize a new deposit account
- Deposit SOL into the account
- Withdraw SOL from the account
- Check account balance

## Testing

The program can be tested on localnet or devnet:

1. Start a local validator:
```bash
solana-test-validator
```

2. Build and deploy the program
3. Run the client

## Security Considerations

- The program only allows the account owner to withdraw funds
- All transactions require proper signatures
- The program checks for sufficient funds before withdrawals 