// Adding functions to the Loan wallet
impl OfficialWallets {
    // Borrowing function
    fn take_loan(&self, amount: u64) -> ProgramResult {
        // Logic for taking out a loan with zero interest rate in the first 30 days
        Ok(())
    }

    // Function to pay loan
    fn pay_loan(&self, amount: u64) -> ProgramResult {
        // Logic for paying loan with interest after 30 days
        Ok(())
    }
}