use cosmwasm_schema::cw_serde;
use ethabi::{encode, ParamType, Token};

use crate::ContractError;

#[cw_serde]
pub enum Packet {
    Read(String),
    Write(String, String),
}

#[cw_serde]
pub struct ExecuteMsg {
    pub channel_id: String,
    pub packet: Packet,
}

impl Packet {
    pub fn encode(self) -> Vec<u8> {
        match self {
            Packet::Read(key) => {
                let tokens = vec![
                    Token::Uint(0.into()), // 0 for read
                    Token::String(key),
                ];
                encode(&tokens)
            }
            Packet::Write(key, value) => {
                let tokens = vec![
                    Token::Uint(1.into()), // 1 for write
                    Token::String(key),
                    Token::String(value),
                ];
                encode(&tokens)
            }
        }
    }

    pub fn decode(data: impl AsRef<[u8]>) -> Result<Self, ContractError> {
        // identify the operation
        let operation = ethabi::decode(&[ParamType::Uint(256)], data.as_ref())
            .map_err(|_| ContractError::EthAbiDecoding)?;

        let operation = operation[0]
            .clone()
            .into_uint()
            .ok_or(ContractError::EthAbiDecoding)?;

        match operation.as_u64() {
            0 => {
                //Read

                // key offset
                let offset_tokens = ethabi::decode(&[ParamType::Uint(256)], &data.as_ref()[32..])
                    .map_err(|_| ContractError::EthAbiDecoding)?;

                let offset = offset_tokens[0]
                    .clone()
                    .into_uint()
                    .ok_or(ContractError::EthAbiDecoding)?
                    .as_usize();

                // key len()
                let length_tokens =
                    ethabi::decode(&[ParamType::Uint(256)], &data.as_ref()[offset..])
                        .map_err(|_| ContractError::EthAbiDecoding)?;

                let length = length_tokens[0]
                    .clone()
                    .into_uint()
                    .ok_or(ContractError::EthAbiDecoding)?
                    .as_usize();

                // key
                let string_position = offset + 32;
                let string_data = &data.as_ref()[string_position..string_position + length];

                let key = String::from_utf8(string_data.to_vec())
                    .map_err(|_| ContractError::EthAbiDecoding)?;
                Ok(Packet::Read(key))
            }
            1 => {
                // Write

                //key offset
                let offset_key_tokens =
                    ethabi::decode(&[ParamType::Uint(256)], &data.as_ref()[32..])
                        .map_err(|_| ContractError::EthAbiDecoding)?;
                let offset_key = offset_key_tokens[0]
                    .clone()
                    .into_uint()
                    .ok_or(ContractError::EthAbiDecoding)?
                    .as_usize();

                // key len()
                let key_length_tokens =
                    ethabi::decode(&[ParamType::Uint(256)], &data.as_ref()[offset_key..])
                        .map_err(|_| ContractError::EthAbiDecoding)?;
                let key_length = key_length_tokens[0]
                    .clone()
                    .into_uint()
                    .ok_or(ContractError::EthAbiDecoding)?
                    .as_usize();

                // key
                let key_position = offset_key + 32;
                let key_data = &data.as_ref()[key_position..key_position + key_length];
                let key = String::from_utf8(key_data.to_vec())
                    .map_err(|_| ContractError::EthAbiDecoding)?;

                // value offset
                let offset_value_tokens =
                    ethabi::decode(&[ParamType::Uint(256)], &data.as_ref()[64..])
                        .map_err(|_| ContractError::EthAbiDecoding)?;
                let offset_value = offset_value_tokens[0]
                    .clone()
                    .into_uint()
                    .ok_or(ContractError::EthAbiDecoding)?
                    .as_usize();
                // value len()
                let value_length_tokens =
                    ethabi::decode(&[ParamType::Uint(256)], &data.as_ref()[offset_value..])
                        .map_err(|_| ContractError::EthAbiDecoding)?;
                let value_length = value_length_tokens[0]
                    .clone()
                    .into_uint()
                    .ok_or(ContractError::EthAbiDecoding)?
                    .as_usize();

                // value
                let value_position = offset_value + 32;
                let value_data = &data.as_ref()[value_position..value_position + value_length];
                let value = String::from_utf8(value_data.to_vec())
                    .map_err(|_| ContractError::EthAbiDecoding)?;

                Ok(Packet::Write(key, value))
            }
            _ => Err(ContractError::EthAbiDecoding),
        }
    }
}

#[cw_serde]
pub struct InitMsg {
    pub initial_key: String,
    pub initial_value: String,
}
