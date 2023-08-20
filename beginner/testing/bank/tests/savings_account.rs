
/*
 * Integration tests live in the /test folder
 * that is located in the root directory.
 * This is where cargo is programmed to look
 *
 * */

// Import the saving account struct
use bank::SavingsAccount;

mod utils;

#[test]
fn should_have_a_starting_balance_of_zero(){
    utils::common_setup();
    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0.0);
}
