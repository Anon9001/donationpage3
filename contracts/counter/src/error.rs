use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("input error")]
    InputError {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("You need to be the account holder to run this.")]
    OnlyOwner {},

    #[error("Incorrect number of types of coins sent.")]
    DonateIncorrectNumberTypes {},

    #[error("Incorrect type of coin sent.")]
    DonateIncorrectType {},

    #[error("Incorrect amount of coin was sent.")]
    DonateIncorrectAmountSent {},

    #[error("This user doesn't exist.  User needs to exist before modifiy is called.")]
    UserNonExistModify {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
