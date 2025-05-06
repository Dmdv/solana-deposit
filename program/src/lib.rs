use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program::invoke,
    sysvar::Sysvar,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum DepositInstruction {
    /// Initialize a new deposit account
    /// Accounts expected:
    /// 0. `[signer]` The account of the person initializing the deposit
    /// 1. `[writable]` The deposit account to be initialized
    /// 2. `[]` The system program
    Initialize,
    
    /// Deposit SOL into the account
    /// Accounts expected:
    /// 0. `[signer]` The account of the person making the deposit
    /// 1. `[writable]` The deposit account
    /// 2. `[]` The system program
    Deposit { amount: u64 },
    
    /// Withdraw SOL from the account
    /// Accounts expected:
    /// 0. `[signer]` The account of the person making the withdrawal
    /// 1. `[writable]` The deposit account
    /// 2. `[]` The system program
    Withdraw { amount: u64 },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DepositAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = DepositInstruction::try_from_slice(instruction_data)?;
    
    match instruction {
        DepositInstruction::Initialize => {
            msg!("Instruction: Initialize");
            process_initialize(program_id, accounts)
        }
        DepositInstruction::Deposit { amount } => {
            msg!("Instruction: Deposit");
            process_deposit(accounts, amount)
        }
        DepositInstruction::Withdraw { amount } => {
            msg!("Instruction: Withdraw");
            process_withdraw(accounts, amount)
        }
    }
}

fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let deposit_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let rent = solana_program::rent::Rent::get()?;
    let space = std::mem::size_of::<DepositAccount>();
    let lamports = rent.minimum_balance(space);

    invoke(
        &system_instruction::create_account(
            initializer.key,
            deposit_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[initializer.clone(), deposit_account.clone()],
    )?;

    let mut deposit_data = DepositAccount {
        owner: *initializer.key,
        balance: 0,
    };
    deposit_data.serialize(&mut *deposit_account.data.borrow_mut())?;

    Ok(())
}

fn process_deposit(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let depositor = next_account_info(account_info_iter)?;
    let deposit_account = next_account_info(account_info_iter)?;
    let _system_program = next_account_info(account_info_iter)?;

    if !depositor.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    invoke(
        &system_instruction::transfer(
            depositor.key,
            deposit_account.key,
            amount,
        ),
        &[depositor.clone(), deposit_account.clone()],
    )?;

    let mut deposit_data = DepositAccount::try_from_slice(&deposit_account.data.borrow())?;
    deposit_data.balance = deposit_account.lamports();
    deposit_data.serialize(&mut *deposit_account.data.borrow_mut())?;

    Ok(())
}

fn process_withdraw(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let withdrawer = next_account_info(account_info_iter)?;
    let deposit_account = next_account_info(account_info_iter)?;
    let _system_program = next_account_info(account_info_iter)?;

    if !withdrawer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let deposit_data = DepositAccount::try_from_slice(&deposit_account.data.borrow())?;
    if deposit_data.owner != *withdrawer.key {
        return Err(ProgramError::InvalidAccountData);
    }

    if deposit_account.lamports() < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    **deposit_account.try_borrow_mut_lamports()? -= amount;
    **withdrawer.try_borrow_mut_lamports()? += amount;

    let mut deposit_data = DepositAccount::try_from_slice(&deposit_account.data.borrow())?;
    deposit_data.balance = deposit_account.lamports();
    deposit_data.serialize(&mut *deposit_account.data.borrow_mut())?;

    Ok(())
}
