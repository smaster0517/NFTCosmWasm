use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CustomQuery};
use crate::route::CudosRoute;

/// TerraQueryWrapper is an override of QueryRequest::Custom to access Terra-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CudosQueryWrapper {
    pub route: CudosRoute,
    pub query_data: CudosQuery,
}

// implement custom query
impl CustomQuery for CudosQueryWrapper {}

/// TerraQuery is defines available query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CudosQuery {
    Denom { denom_id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomResponse {
    pub denom: Denom,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct Denom {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub creator: String,
}
