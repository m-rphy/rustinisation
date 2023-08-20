
/*
 * Documentations comments start with -> `///`, rather than `//`
 * 
 *  You typically want to document structs, enums, impl and functions
 *  and you document them in mark down
 * */

/// A savings account
pub struct SavingsAccount {
    balance: f32,
}

impl SavingsAccount {
    /// Creates a "SvaingsAccount" with a balance of 0
    ///
    ///  # Examples
    ///
    ///  ```
    ///  use bank::SavingsAccount;
    ///  let account = SavingsAccount::new();
    ///  assert_eq!(account.get_balance(), 0.0);
    ///  ```
    
    // NOTE - the code inside the markdown code block
    //          will be executed as a test, so you 
    //          are confident that the examples are 
    //          valid

    pub fn new() -> Self {
        SavingsAccount {
            balance: 0.0,
        }
    }

    pub fn get_balance(&self) -> f32 {self.balance}

    pub fn deposit(&mut self, amount: f32) {
        if amount < 0.0 {
            panic!("Can not deposit negative amounts");
        }
        self.balance += amount
    }

    pub fn transfer(&self, acc_num: i32, amount: f32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {acc_num}"))
    }
}

// in rust, unit tests are written with the source code
// Have a test file in each module you write

#[cfg(test)]
mod test {
    // import all the items from parent module
    use super::*;

    #[test]
    // assert starting balance is zero
    fn should_have_initial_balance_of_0() {
        // set-up 
        let test_account = SavingsAccount::new();
        // assertion
        assert_eq!(test_account.get_balance(), 0.0);
    }

    #[test]
    // make sure a function panics
    #[should_panic]
    fn should_panic_if_deposit_is_neg() {
        let mut test_account = SavingsAccount::new();
        test_account.deposit(-78.78);
    }
    
    #[test]
    fn should_be_able_to_deposit() {
        let mut test_account = SavingsAccount::new();
        test_account.deposit(78.78);
        assert_eq!(test_account.get_balance(), 78.78);
        assert_ne!(test_account.get_balance(), 0.0);
        assert!(test_account.get_balance() == 78.78);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut test_account = SavingsAccount::new();
        test_account.deposit(78.78);
        // the question mark will propagate errs
        test_account.transfer(123456, 78.78)?;
        Ok(())
    }
    
    

}
