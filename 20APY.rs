// These lines import the necessary tools for our program
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError, // Importar ProgramError
};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    state::Mint,
};

// Set the program entrypoint
// This line tells the computer where to start running our program
entrypoint!(process_instruction);

// Define constants
// Here we define some important information about our token, such as its name, total quantity, how many decimal places it has and the transfer fee
const TOKEN_NAME: &str = "US20";
const TOTAL_SUPPLY: u64 = 100_000_000_000 * 100_000_000; // 100 billion to 8 decimal places
const DECIMALS: u8 = 8;
const FEE_PERCENTAGE: f64 = 0.01; // 1.00%

// Define official wallets
// This part creates a structure to store the official wallets of our token
struct OfficialWallets {
    treasury: Pubkey,
    market: Pubkey,
    seed: Pubkey,
    loan: Pubkey,
}

// Enum for program instructions
// Here we define the types of actions our program can perform: initializing the token or transferring it
enum TokenInstruction {
    InitializeMint,
    Transfer { amount: u64 },
}

// This function helps to interpret the instructions that the program receives
impl TokenInstruction {
    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => TokenInstruction::InitializeMint,
            1 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                TokenInstruction::Transfer { amount }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

// Main function of the program
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Decode the instruction
    let instruction = TokenInstruction::unpack(instruction_data)?;

    match instruction {
        TokenInstruction::InitializeMint => {
            // Initialize US20 token
            let mint_account = next_account_info(accounts_iter)?;
            let mint_authority = next_account_info(accounts_iter)?;
            let rent = next_account_info(accounts_iter)?;

            initialize_mint(
                program_id,
                mint_account.key, // Acessar a chave pública
                mint_authority.key, // Acessar a chave pública
                None,
                DECIMALS,
            )?;

            // Mint the total supply of tokens
            mint_to(
                program_id,
                mint_account.key, // Acessar a chave pública
                mint_authority.key, // Acessar a chave pública
                mint_authority.key, // Acessar a chave pública
                &[],
                TOTAL_SUPPLY,
            )?;

            msg!("US20 token initialized successfully!");
        }
        TokenInstruction::Transfer { amount } => {
            let source_account = next_account_info(accounts_iter)?;
            let destination_account = next_account_info(accounts_iter)?;
            let authority = next_account_info(accounts_iter)?;

            let official_wallets = OfficialWallets {
                treasury: *next_account_info(accounts_iter)?.key,
                market: *next_account_info(accounts_iter)?.key,
                seed: *next_account_info(accounts_iter)?.key,
                loan: *next_account_info(accounts_iter)?.key,
            };

            transfer_tokens(source_account, destination_account, amount, &official_wallets, program_id, authority)?;

            msg!("Transfer completed successfully!");
        }
    }

    Ok(())
}

// Function to check if a wallet is official
fn is_official_wallet(wallet: &Pubkey, official_wallets: &OfficialWallets) -> bool {
    wallet == &official_wallets.treasury
        || wallet == &official_wallets.market
        || wallet == &official_wallets.seed
        || wallet == &official_wallets.loan
}

// Function to calculate transfer rate
fn calculate_fee(amount: u64) -> u64 {
    (amount as f64 * FEE_PERCENTAGE).round() as u64
}

// Function to transfer tokens including fee calculation and application
fn transfer_tokens(
    from: &AccountInfo,
    to: &AccountInfo,
    amount: u64,
    official_wallets: &OfficialWallets,
    program_id: &Pubkey, // Adicionar program_id como parâmetro
    authority: &AccountInfo, // Adicionar authority como parâmetro
) -> ProgramResult {
    let fee = if !is_official_wallet(from.key, official_wallets) && !is_official_wallet(to.key, official_wallets) {
        calculate_fee(amount)
    } else {
        0
    };

    let transfer_amount = amount - fee;

    // Logic for transferring tokens (using the spl_token library)
    spl_token::instruction::transfer(
        program_id,
        from.key,
        to.key,
        authority.key, // authority agora está definido
        &[],
        transfer_amount,
    )?;

    if fee > 0 {
        // Transfer the fee to the SEED wallet
        spl_token::instruction::transfer(
            program_id,
            from.key,
            &official_wallets.seed,
            authority.key,
            &[],
            fee,
        )?;
    }

    Ok(())
}
