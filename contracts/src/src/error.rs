use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Too many poll options")]
    TooManyOptions {},

    #[error("Option not found")]
    OptionNoExist {},

    #[error("Poll not found")]
    PollNotFound {},
}
