use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    // 09 Execute 1
    // - #[error("Custom Error val: {val:?}")]
    // - CustomError { val: String },
    // + #[error("Too many poll options")]
    // + TooManyOptions {},
    #[error("Too many poll options")]
    TooManyOptions {},
}
