//State is where the state of the contract lives

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

//main global state definition
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub raffle_state: u8,
    pub owner: Addr,
}

//A victim data that exists on the chain.  each victim data corrasponds to a victim
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VictimData {
    pub amount_owed: u32,
    pub amount_recived: u32,
    pub on_chain: bool,
}

//main global state
pub const STATE: Item<State> = Item::new("state");

//Victims of the terra scam.  Address of the victim to victim data that shows how much they are owed and recived
//Owed and on chain can only be entered by admin.  Admin can change amt recived but also donate can change it too if they donate
pub const VICTIMS: Map<&Addr, VictimData> = Map::new("victims");

//Donator's address combined with raffle state mapped to how much they donated.
//for every raffle state plus their address as a key to map to how much they donated in that raffle state
pub const DONATIONS: Map<(&Addr, u8), Uint128> = Map::new("donations");


