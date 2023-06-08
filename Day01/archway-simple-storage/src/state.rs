use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]

pub struct Humans {
    pub name: String,
    pub age: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]

pub struct Age {
    pub age: u64,
}

pub const HUMANS: Item<Humans> = Item::new("humans");
pub const NAMETOAGE: Map<String, Age> = Map::new("nametoage");
