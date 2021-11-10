use cosmwasm_std::CosmosMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::route::CudosRoute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MsgIssueDenom {
        id: String,
        name: String,
        schema: String,
        sender: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Denom { denom_id: String },
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// CudosMsgWrapper is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub struct CudosMsgWrapper {
    pub route: CudosRoute,
    pub msg_data: CudosMsg,
}

// this is a helper to be able to return these as CosmosMsg easier
impl From<CudosMsgWrapper> for CosmosMsg<CudosMsgWrapper> {
    fn from(original: CudosMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CudosMsg {
    IssueDenom {
        id: String,
        name: String,
        schema: String,
        sender: String,
    },
}

// create_swap_msg returns wrapped swap msg
pub fn create_issue_denom_msg(id: String, name: String, schema: String, sender: String) -> CosmosMsg<CudosMsgWrapper> {
    CudosMsgWrapper {
        route: CudosRoute::Nft,
        msg_data: CudosMsg::IssueDenom {
            id,
            name,
            schema,
            sender,
        },
    }
        .into()
}
