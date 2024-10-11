// Adding functions to the Seed wallet
impl OfficialWallets {
    // Function to send REWARD
    fn send_reward(&self, investor: &Pubkey) -> ProgramResult {
        // Logic for sending 25 US20 to investors
        Ok(())
    }

    // Function to send AWARD
    fn send_award(&self, client: &Pubkey) -> ProgramResult {
        // Logic for sending 50,000 US20 to customers
        Ok(())
    }

    // Function to carry out draw
    fn lottery(&self) -> ProgramResult {
        // Logic for carrying out a draw based on rules
        Ok(())
    }
}