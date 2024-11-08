use cosmwasm_schema::cw_serde;
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_storage_plus::Map;

use crate::ContractError;

pub const VALUES: Map<String, String> = Map::new("values");

#[cw_serde]
pub struct InitMsg {
    pub initial_key: String,
    pub initial_value: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    VALUES.save(deps.storage, msg.initial_key, &msg.initial_value)?;
    Ok(Response::default())
}
