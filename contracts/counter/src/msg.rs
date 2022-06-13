use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr,Uint128};
use crate::state::{VictimData};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub raffle_state: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Donate {donations: Vec<InputDonation>},

    /* ADMIN FUNCTIONS */
    //Enter all the victim's addresses and how much they are owed
    //If a victim already exists, their amount owed gets modified
    VictimEntry {victims: Vec<InputVictimInfoOwe>},

    //Modify victim's amount recived
    VictimAmtModify {victims: Vec<InputVictimInfoPaid>},

    //transfers ownership of contract, no admins exist, just owners
    TransferOwnership{address: String},

    SetRaffleState { new_raffle_value: u8 },
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


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllVictimsResponse {
    pub victims: Vec<VictimInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VictimInfo {
    pub address: Addr,
    pub victim: VictimData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InputVictimInfoOwe {
    pub address: String,
    pub owed: u32,
    pub onchain: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InputVictimInfoPaid {
    pub address: String,
    pub paid: u32,
}





#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InputDonation {
    pub address: String,
    pub amt: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DonationResponse {
    pub donations: Vec<SingleOutputDonation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SingleOutputDonation {
    pub address: Addr,
    pub donation_amount: Uint128,
    pub raffle_state: u8,
}


