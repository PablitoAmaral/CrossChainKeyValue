use cosmwasm_schema::cw_serde;
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, IbcMsg, IbcTimeout, Binary,Timestamp};
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


#[cw_serde]
pub enum ExecuteMsg {
    Key {
        channel_id: String,
        key: String,
    },
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Key { channel_id, key } => {
            let ibc_packet = IbcMsg::SendPacket {
                channel_id,
                data: Binary::new(key.into_bytes()),
                timeout: IbcTimeout::with_timestamp(Timestamp::from_nanos(
                    env.block.time.nanos() + (60 * 1_000_000_000 *5)
                ))
            };
            Ok(Response::default().add_message(ibc_packet))
        }
    }
}