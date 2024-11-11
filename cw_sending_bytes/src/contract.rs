use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response, Timestamp};
use cw_storage_plus::Map;

use crate::{
    msg::{ExecuteMsg, InitMsg},
    ContractError,
};

pub const VALUES: Map<String, String> = Map::new("values");

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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ExecuteMsg { channel_id, packet } = msg;

    let encoded_data = packet.encode();

    let ibc_packet = IbcMsg::SendPacket {
        channel_id,
        data: Binary::new(encoded_data),
        timeout: IbcTimeout::with_timestamp(Timestamp::from_nanos(
            env.block.time.nanos() + (60 * 1_000_000_000 * 5),
        )),
    };

    Ok(Response::default().add_message(ibc_packet))
}
