use crate::msg::{GetHumanDataResponse, GetMappedDataResponse};
use crate::state::{Age, Humans, HUMANS, NAMETOAGE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // version info for migration info
    const CONTRACT_NAME: &str = "crates.io:archway-simple-storage";
    const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
    Ok(Response::new()
        .add_attribute("action", "contract instantiated")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::FillData { name, age } => execute_fill_data(deps, env, info, name, age),
        ExecuteMsg::MapData { name, age } => execute_map_data(deps, env, info, name, age),
    }
}

fn execute_fill_data(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    name: String,
    age: u64,
) -> Result<Response, ContractError> {
    let human_data = Humans { name, age };
    HUMANS.save(deps.storage, &human_data)?;
    Ok(Response::new().add_attribute("action", "fill_data"))
}

fn execute_map_data(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    name: String,
    age: u64,
) -> Result<Response, ContractError> {
    if NAMETOAGE.has(deps.storage, name.clone()) {
        return Err(ContractError::CustomError {
            val: "Name already exists".to_string(),
        });
    }

    let name_to_age = Age { age };
    NAMETOAGE.save(deps.storage, name, &name_to_age)?;
    Ok(Response::new().add_attribute("action", "mapped_data"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHumanData {} => query_get_human_data(deps, env),
        QueryMsg::GetMappedData { name } => query_get_mapped_data(deps, env, name),
    }
}

fn query_get_human_data(deps: Deps, _env: Env) -> StdResult<Binary> {
    let human_data = HUMANS.may_load(deps.storage)?;
    to_binary(&GetHumanDataResponse { human_data })
}

fn query_get_mapped_data(deps: Deps, _env: Env, name: String) -> StdResult<Binary> {
    let mapped_data = NAMETOAGE.may_load(deps.storage, name)?;
    to_binary(&GetMappedDataResponse { mapped_data })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::attr;
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {};

        let expected_contract_name = "crates.io:archway-simple-storage";
        let expected_contract_version = env!("CARGO_PKG_VERSION");

        let result = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone());

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.attributes.len(), 3);
        assert_eq!(response.attributes[0].key, "action");
        assert_eq!(response.attributes[0].value, "contract instantiated");
        assert_eq!(response.attributes[1].key, "contract_name");
        assert_eq!(response.attributes[1].value, expected_contract_name);
        assert_eq!(response.attributes[2].key, "contract_version");
        assert_eq!(response.attributes[2].value, expected_contract_version);
    }

    #[test]
    fn test_fill_data() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = InstantiateMsg {};

        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone());

        let msg = ExecuteMsg::FillData {
            name: "John Newton".to_string(),
            age: 25,
        };
        let resp = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "fill_data")]);
    }

    #[test]
    fn test_map_data() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = InstantiateMsg {};

        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone());

        let msg = ExecuteMsg::MapData {
            name: "John".to_string(),
            age: 25,
        };
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "mapped_data")]);

        let msg = ExecuteMsg::MapData {
            name: "John".to_string(),
            age: 25,
        };
        let _resp = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }

    #[test]
    fn query_get_human_data() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = InstantiateMsg {};

        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone());

        let msg = ExecuteMsg::FillData {
            name: "John Newton".to_string(),
            age: 25,
        };
        let _resp = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let query_msg = QueryMsg::GetHumanData {};
        let query_resp = query(deps.as_ref(), env, query_msg).unwrap();

        let human_data: GetHumanDataResponse = from_binary(&query_resp).unwrap();
        assert_eq!(
            human_data,
            GetHumanDataResponse {
                human_data: Some(Humans {
                    name: "John Newton".to_string(),
                    age: 25,
                })
            }
        );
    }

    #[test]
    fn query_get_mapped_data() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = InstantiateMsg {};

        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone());

        let msg = ExecuteMsg::MapData {
            name: "John".to_string(),
            age: 25,
        };
        let _resp = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let query_msg = QueryMsg::GetMappedData {
            name: "John".to_string(),
        };
        let query_resp = query(deps.as_ref(), env, query_msg).unwrap();

        let mapped_data: GetMappedDataResponse = from_binary(&query_resp).unwrap();
        assert_eq!(
            mapped_data,
            GetMappedDataResponse {
                mapped_data: Some(Age { age: 25 })
            }
        );
    }
}
