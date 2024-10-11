// Adding functions to the Market wallet
impl OfficialWallets {
    // Buyback function
    fn buyback(&self, amount: u64, asset: &str) -> ProgramResult {
        // Logic for repurchase based on the chosen asset
        Ok(())
    }

    // Function to calculate yield
    fn calculate_yield(&self, amount: u64) -> u64 {
        (amount as f64 * 2.48832).round() as u64 // 148.83200000% of income
    }
}