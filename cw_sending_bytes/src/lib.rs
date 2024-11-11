pub mod contract;
pub mod ibc;
pub mod msg;
pub mod testes;

use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Invalid IBC version, got {version}")]
    InvalidIbcVersion { version: String },
    #[error("Only supports unordered channel")]
    OnlyOrderedChannel {},
    #[error("The packet has not been serialized using ETH ABI")] //alterar isso!
    KeyNotFound,
    #[error("The packet has not been serialized using ETH ABI")]
    EthAbiDecoding,
}
