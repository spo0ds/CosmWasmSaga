use crate::state::{Age, Humans};
use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    FillData { name: String, age: u64 },
    MapData { name: String, age: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, QueryResponses)]

pub enum QueryMsg {
    #[returns(GetHumanDataResponse)]
    GetHumanData {},
    #[returns(GetMappedDataResponse)]
    GetMappedData { name: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetHumanDataResponse {
    pub human_data: Option<Humans>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetMappedDataResponse {
    pub mapped_data: Option<Age>,
}
