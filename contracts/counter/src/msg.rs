//This file is for defining messages going in and going out

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr,Uint128};
use crate::state::{VictimData};

//what message gets returned when you init
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub raffle_state: u8,
}

//What messages we take in that modify the contract and use gas
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

//What messages we use to query or get data from the contract that doesn't cost gas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetRaffleState {},
    GetVictimData{},
    GetDonorData{},
    GetOwnerAddr {},
}

// Response type of when we get raffle state back
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RaffleStateResponse {
    pub raffle_state: u8,
}

// Response type of when we get the owner address
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnderAddressResponse {
    pub owner_address: Addr,
}

// Output: Response of when we get all the victim data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllVictimsResponse {
    pub victims: Vec<VictimInfo>,
}

// Output: Each victim data of a victim, gets put into AllVictimResponse
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VictimInfo {
    pub address: Addr,
    pub victim: VictimData,
}
// Input for when we add in a victim or we change how much they are owed or change if they are on chain or not
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InputVictimInfoOwe {
    pub address: String,
    pub owed: Uint128,
    pub onchain: bool,
}

// Input for when admin changes how much a user has been paid.  This is for when lawsuits are paid out or external donations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InputVictimInfoPaid {
    pub address: String,
    pub paid: Uint128,
}




// Input for when a donation comes in
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InputDonation {
    pub address: String,
    pub amt: Uint128,
}

// Output for when user queies for all donation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DonationResponse {
    pub donations: Vec<SingleOutputDonation>,
}

// Single output for a single donation raffle for a single user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SingleOutputDonation {
    pub address: Addr,
    pub donation_amount: Uint128,
    pub raffle_state: u8,
}


