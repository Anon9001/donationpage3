#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{StdError, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Addr, Uint128, BankMsg, Coin, Order};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{OwnderAddressResponse, RaffleStateResponse, ExecuteMsg, InstantiateMsg, QueryMsg,
    InputDonation, InputVictimInfoOwe, InputVictimInfoPaid, AllVictimsResponse, VictimInfo, SingleOutputDonation, 
    DonationResponse};
use crate::state::{State, STATE, DONATIONS, VICTIMS, VictimData};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        raffle_state: 0,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("raffle_state", msg.raffle_state.to_string()))
}

//function maps from msg to here.  Gateway to main contract from msg
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRaffleState { new_raffle_value } => try_set_raffle_state(deps, info, new_raffle_value),
        ExecuteMsg::TransferOwnership{address} => try_transfer_ownership(deps, info, address),
        ExecuteMsg::VictimEntry {victims} => try_victim_entry(deps, info, victims),
        ExecuteMsg::VictimAmtModify {victims} => try_victim_amt_modify(deps, info, victims),
        ExecuteMsg::Donate { donations } => try_donate(deps, env, info, donations),
    }
}

// Tries to donate to users, errors out if input isn't correct or tries to donate too much to a user
pub fn try_donate (
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    donations: Vec<InputDonation>, 
) -> Result<Response, ContractError> {
     
    if info.funds.len() != 1 {
        return Err(ContractError::DonateIncorrectNumberTypes{})
    }

    //NOTE change this to USDC, USDT, stable coins on prod
    //PUT this in a settings file?
    if info.funds[0].denom != "uluna" {
        return Err(ContractError::DonateIncorrectType{})
    }

    let mut amt_recived = info.funds[0].amount.u128();
    let donation_amount = info.funds[0].amount;
    let mut messages = vec![];
    for cur_donation in donations {
        let cur_amt = cur_donation.amt;
        let coin_amount = Coin {
            amount: cur_amt,
            denom: info.funds[0].denom.to_string(),
        };
        amt_recived -= coin_amount.amount.u128();
        messages.push(BankMsg::Send {
            to_address: cur_donation.address.clone(),
            amount: vec![coin_amount],
        });

        let cur_addr:Addr = deps.api.addr_validate(&cur_donation.address)?;

        let update_victim_data_closure = |current_victim_data: Option<VictimData>| -> StdResult<VictimData> {
            match current_victim_data{
            //Note: relying on compiler optimization here to save only what's changed... kind of a long shot, 
            //get it working then replace with a modify
            // Some(data) => Ok(VictimData{amount_owed: Uint128::from(123u128), amount_recived: data.amount_recived}),
            // None => Ok(VictimData{amount_owed: Uint128::from(456u128), amount_recived: Uint128::from(0u128)}),
           Some(data) => {
                let new_amt_recived = data.amount_recived + cur_donation.amt;
                if new_amt_recived > data.amount_owed {
                    return Err(StdError::generic_err(format!("{} Donate function: unable to donate more than the user is owed", cur_addr)))
                }
                Ok(VictimData{amount_owed: data.amount_owed, amount_recived: new_amt_recived, on_chain: data.on_chain})
           },
           None => Err(StdError::generic_err(format!("{} Donate function: user not found, for user modify recived", cur_addr))),
        }
        };
        VICTIMS.update(deps.storage, &cur_addr, update_victim_data_closure)?;
          
    }

    //Could just refund / send back extra coins and let the system error out on too few coins but would rather err on the side of causion
    if amt_recived != 0 {
        return Err(ContractError::DonateIncorrectAmountSent{})
    }
    let raffle_state = STATE.load(deps.storage)?.raffle_state;
    let donor_address = info.sender;

    let update_donate_data_closure = |current_donor_data: Option<Uint128>| -> StdResult<Uint128> {
        match current_donor_data{
            Some(data) => Ok(data + donation_amount), //
            None => Ok(donation_amount),
        }
    };

    DONATIONS.update(deps.storage, (&donor_address, raffle_state), update_donate_data_closure)?;


    Ok(Response::new().add_messages(messages))
}

pub fn try_victim_amt_modify(deps: DepsMut, info: MessageInfo, victims: Vec<InputVictimInfoPaid>)  -> Result<Response, ContractError> {
    //Uint128
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::OnlyOwner{})
    }

    for cur_victim in victims {
        let cur_recived = cur_victim.paid;
        let cur_addr:Addr = deps.api.addr_validate(&cur_victim.address)?;
        
        let update_victim_data_closure = |current_victim_data: Option<VictimData>| -> StdResult<VictimData> {
                match current_victim_data{
                //Note: relying on compiler optimization here to save only what's changed... kind of a long shot, 
                //get it working then replace with a modify
                // Some(data) => Ok(VictimData{amount_owed: Uint128::from(123u128), amount_recived: data.amount_recived}),
                // None => Ok(VictimData{amount_owed: Uint128::from(456u128), amount_recived: Uint128::from(0u128)}),
               Some(data) => Ok(VictimData{amount_owed: data.amount_owed, amount_recived: cur_recived, on_chain: data.on_chain}),
               None => Err(StdError::generic_err(format!("{} user not found, for user modify recived", cur_addr))),
            }
        };
        VICTIMS.update(deps.storage, &cur_addr, update_victim_data_closure)?;
    }
    return Ok(Response::new().add_attribute("method", "try_victim_amt_modify"))
}


pub fn try_victim_entry(deps: DepsMut, info: MessageInfo, victims: Vec<InputVictimInfoOwe>)  -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::OnlyOwner{})
    }

    
    for cur_victim in victims {
        let cur_owed = cur_victim.owed; 
        let cur_addr:Addr = deps.api.addr_validate(&cur_victim.address)?;
        let on_chain = cur_victim.onchain;
        
        let update_victim_data_closure = |current_victim_data: Option<VictimData>| -> StdResult<VictimData> {
                match current_victim_data{
                //Note: relying on compiler optimization here to save only what's changed... kind of a long shot, 
                //get it working then replace with a modify
                // Some(data) => Ok(VictimData{amount_owed: Uint128::from(123u128), amount_recived: data.amount_recived}), //
                // None => Ok(VictimData{amount_owed: Uint128::from(456u128), amount_recived: Uint128::from(0u128)}),
               Some(data) => Ok(VictimData{amount_owed: cur_owed, amount_recived: data.amount_recived, on_chain: on_chain}), //
               None => Ok(VictimData{amount_owed: cur_owed, amount_recived: Uint128::from(0u128), on_chain: on_chain}),
            }
        };
        VICTIMS.update(deps.storage, &cur_addr, update_victim_data_closure)?;
    }
    return Ok(Response::new().add_attribute("method", "try_victim_entry"))
}

pub fn try_transfer_ownership(deps: DepsMut, info: MessageInfo, address:String)  -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::OnlyOwner{})
    }

    let enter_address:Addr = deps.api.addr_validate(&address)?;
    state.owner = enter_address;
    STATE.save(deps.storage, &state)?;
    return Ok(Response::new().add_attribute("method", "try_transfer_ownership"))

}

pub fn try_set_raffle_state(deps: DepsMut, info: MessageInfo, new_raffle_value: u8) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.raffle_state = new_raffle_value;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "try_set_raffle_state"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRaffleState {} => to_binary(&query_raffle_state(deps)?),
        QueryMsg::GetVictimData {} => to_binary(&query_victim_data(deps)?),
        QueryMsg::GetDonorData {} => to_binary(&query_donor_data(deps)?),
        QueryMsg::GetOwnerAddr {} => to_binary(&query_owner_addr(deps)?),
    }
}
fn query_owner_addr(deps: Deps) -> StdResult<OwnderAddressResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnderAddressResponse { owner_address: state.owner })
}

fn query_victim_data(deps: Deps) -> StdResult<AllVictimsResponse> {
    let victims = VICTIMS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            item.map(|(addr, victim)| VictimInfo {
                address: addr.into(),
                victim,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;
    Ok(AllVictimsResponse { victims })
}

fn query_donor_data(deps: Deps) -> StdResult<DonationResponse> {
    let donations = DONATIONS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            item.map(|((addr, raffle_state), donation_amount)| SingleOutputDonation {
                address: addr.into(),
                donation_amount,
                raffle_state,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;
    Ok(DonationResponse { donations })
}

fn query_raffle_state(deps: Deps) -> StdResult<RaffleStateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(RaffleStateResponse { raffle_state: state.raffle_state })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { raffle_state: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetRaffleState {}).unwrap();
        let value: RaffleStateResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.raffle_state);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { raffle_state: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // // beneficiary can release it
        // let info = mock_info("anyone", &coins(2, "token"));
        // let msg = ExecuteMsg::Increment {};
        // let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetRaffleState {}).unwrap();
        let value: RaffleStateResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.raffle_state);
    }

    #[test]
    fn reset() {
        // let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        // let msg = InstantiateMsg { count: 17 };
        // let info = mock_info("creator", &coins(2, "token"));
        // let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // // beneficiary can release it
        // let unauth_info = mock_info("anyone", &coins(2, "token"));
        // let msg = ExecuteMsg::Reset { count: 5 };
        // let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        // match res {
        //     Err(ContractError::Unauthorized {}) => {}
        //     _ => panic!("Must return unauthorized error"),
        // }

        // // only the original creator can reset the counter
        // let auth_info = mock_info("creator", &coins(2, "token"));
        // let msg = ExecuteMsg::Reset { count: 5 };
        // let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // // should now be 5
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: RaffleStateResponse = from_binary(&res).unwrap();
        // assert_eq!(5, value.count);
    }

////////////// multi send tests here
    // #[test]
    // fn multisend() {

    //     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    //     let msg = InstantiateMsg { raffle_state: 17 };
    //     let info = mock_info("creator", &coins(2000, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // // beneficiary can release it
    //     // let info = mock_info("anyone", &coins(2, "token"));
    //     // let msg = ExecuteMsg::Increment {};
    //     // let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetRaffleState {}).unwrap();
    //     let value: RaffleStateResponse = from_binary(&res).unwrap();
    //     assert_eq!(0, value.raffle_state);


    //     let balance = coins(200, "token");
    //     let env = mock_env("anyone", &balance);
    //     let coin = Coin {
    //         amount: Uint128(100),
    //         denom: "token".to_string(),
    //     };
    //     let payment1 = Payment {
    //         recipient: "recipient1",
    //         pay: vec![coin.clone()],
    //     };
    //     let payment2 = Payment {
    //         recipient: "recipient2",
    //         pay: vec![coin.clone()],
    //     };

    //     let msg = ExecuteMsg::MultiSend {
    //         payments: vec![payment1, payment2],
    //     };

    //     let res = handle(&mut deps, env, msg).unwrap();
    //     println!("Messages {:#?}", res.messages);
    //     println!("Log {:#?}", res.log);
    // }

    // #[test]
    // fn empty_multisend() {
    //     let mut deps = mock_dependencies(20, &[]);
    //     let fee = Coin {
    //         amount: Uint128(100),
    //         denom: "token".to_string(),
    //     };

    //     let msg = InitMsg { fee: fee.clone() };
    //     let env = mock_env("creator", &coins(1000, "token"));

    //     let _res = init(&mut deps, env, msg).unwrap();

    //     let env = mock_env("anyone", &[]);
    //     let coin = Coin {
    //         amount: Uint128(100),
    //         denom: "token".to_string(),
    //     };
    //     let payment1 = Payment {
    //         recipient: "recipient1",
    //         pay: vec![coin.clone()],
    //     };

    //     let msg = HandleMsg::MultiSend {
    //         payments: vec![payment1],
    //     };
    //     let res = handle(&mut deps, env, msg);
    //     match res {
    //         Ok(_) => panic!("expected error"),
    //         Err(StdError::GenericErr { msg, .. }) => {
    //             assert_eq!(msg, "You must pass some coins to send make a multisend")
    //         }
    //         Err(e) => panic!("unexpected error: {:?}", e),
    //     }
    // }


}
