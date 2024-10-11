Adding functions to the Treasury wallet
impl OfficialWallets {
    // Function to receive 100% of created US20 tokens
    fn receive_tokens(&self, amount: u64) -> ProgramResult {
        // Logic for receiving tokens
let account_info_iter = &mut accounts.iter(); // Cria um iterador sobre as contas
    let official_wallets = OfficialWallets {
        // Cria uma instância de OfficialWallets
        treasury: *next_account_info(account_info_iter)?.key, // Pega a chave da conta Tresury
        
        // Chama a função para inicializar o mint
            mint_to(
                program_id,                 // ID do programa
                mint_account,               // Conta do mint
                &official_wallets.treasury, // Envia tokens para a conta do Treasury
                mint_authority.key,         // Chave da autoridade do mint
                &[],                        // Sem contas adicionais
                TOTAL_SUPPLY,               // Total de tokens a serem mintados
            )?; // Chama a função para mintar os tokens
            msg!("Mint initialized and tokens minted to treasury."); // Mensagem de log após a mintagem
        }
        TokenInstruction::Transfer { amount } => {
            // Se a instrução for para transferir tokens
            let source_account = next_account_info(account_info_iter)?; // Pega a conta de origem
            let destination_account = next_account_info(account_info_iter)?; // Pega a conta de destino
            let authority = next_account_info(account_info_iter)?; // Pega a conta da autoridade

            // Verifica se a transferência é entre carteiras não oficiais
            is_non_official_transfer = *destination_account.key != official_wallets.market
                && *destination_account.key != official_wallets.seed;

            if *source_account.key == official_wallets.treasury {
                // Se a conta de origem for Treasury
                // Verifica se a transferência é para carteiras SEED ou MARKET
                if *destination_account.key != official_wallets.market
                    && *destination_account.key != official_wallets.seed
                {
                    return Err(ProgramError::IllegalOwner); // Retorna erro se a conta de destino não for oficial
                }

                // Se a transferência for para a MARKET, calcula a taxa de 20%
                if *destination_account.key == official_wallets.market {
                    let fee = (amount as f64 * 0.20) as u64; // Calcula 20% da quantidade
                    let amount_after_fee = amount - fee; // Calcula o valor após a taxa

                    // Transfere a taxa para a carteira SEED
                    transfer(
                        program_id,             // ID do programa
                        source_account,         // Conta de origem
                        &official_wallets.seed, // Conta SEED como destino
                        authority,              // Autoridade da transferência
                        &[],                    // Sem contas adicionais
                        fee,                    // Valor da taxa
                    )?;
                    msg!("20% da quantidade transferida para a carteira SEED."); // Mensagem de log sobre a taxa

                    // Transfere o valor restante para a MARKET
                    transfer(
                        program_id,          // ID do programa
                        source_account,      // Conta de origem
                        destination_account, // Conta de destino (MARKET)
                        authority,           // Autoridade da transferência
                        &[],                 // Sem contas adicionais
                        amount_after_fee,    // Valor restante após a taxa
                    )?;
                } else {
                    // Transfere normalmente se for para a carteira SEED
                    transfer(
                        program_id,          // ID do programa
                        source_account,      // Conta de origem
                        destination_account, // Conta de destino (SEED)
                        authority,           // Autoridade da transferência
                        &[],                 // Sem contas adicionais
                        amount,              // Valor total a ser transferido
                    )?;
                }
            } else {
                // Se a conta de origem não for Treasury
                // Verifica se a transferência é entre carteiras não oficiais
                let is_non_official_transfer = *destination_account.key != official_wallets.market
                    && *destination_account.key != official_wallets.seed;

                if is_non_official_transfer {
                    // Se a transferência for não oficial
                    // Calcula a taxa de 1.00%
                    let fee = (amount as f64 * FEE_PERCENTAGE) as u64; // Calcula 1% da quantidade
                    let amount_after_fee = amount - fee; // Calcula o valor após a taxa

                    // Transfere a taxa para a carteira SEED
                    transfer(
                        program_id,             // ID do programa
                        source_account,         // Conta de origem
                        &official_wallets.seed, // Conta SEED como destino
                        authority,              // Autoridade da transferência
                        &[],                    // Sem contas adicionais
                        fee,                    // Valor da taxa
                    )?;
                    msg!("1.00% de taxa transferida para a carteira SEED."); // Mensagem de log sobre a taxa

                    // Transfere o valor restante
                    transfer(
                        program_id,          // ID do programa
                        source_account,      // Conta de origem
                        destination_account, // Conta de destino
                        authority,           // Autoridade da transferência
                        &[],                 // Sem contas adicionais
                        amount_after_fee,    // Valor restante após a taxa
                    )?;
                } else {
                    // Transfere normalmente se for uma carteira oficial
                    transfer(
                        program_id,          // ID do programa
                        source_account,      // Conta de origem
                        destination_account, // Conta de destino
                        authority,           // Autoridade da transferência
                        &[],                 // Sem contas adicionais
                        amount,              // Valor total a ser transferido
                    )?;
                }
            }
        Ok(())
    }