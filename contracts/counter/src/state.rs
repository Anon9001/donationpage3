use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub raffle_state: u8,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VictimData {
    pub amount_owed: u32,
    pub amount_recived: u32,
    pub on_chain: bool,
}

pub const STATE: Item<State> = Item::new("state");
//Victims of the terra scam.  Address of the victim to amount they lost
//Should be inputed by admin
pub const VICTIMS: Map<&Addr, VictimData> = Map::new("victims");

//Donator's address mapped to donations by raffle state
pub const DONATIONS: Map<(&Addr, u8), Uint128> = Map::new("donations");


