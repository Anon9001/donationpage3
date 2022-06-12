use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr,Uint128, Coin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub raffle_state: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetRaffleState { new_raffle_value: u8 },


    Donate {addresses: Vec<String>, transfer_amts: Vec<u32>},
//    MultiSend { payments: Vec<Payment> },

    /* ADMIN FUNCTIONS */
    //Enter all the victim's addresses and how much they are owed
    //If a victim already exists, their amount owed gets modified
    VictimEntry {addresses: Vec<String>, owed_amts: Vec<u32>},

    //Modify victim's amount recived
    VictimAmtModify {addresses: Vec<String>, amounts_recived: Vec<u32>},

    //transfers ownership of contract, no admins exist, just owners
    TransferOwnership{address: String},


}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetRaffleState {},
    GetVictimData{},
    GetDonorData{},
    GetOwnerAddr {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RaffleStateResponse {
    pub raffle_state: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnderAddressResponse {
    pub owner_address: Addr,
}

