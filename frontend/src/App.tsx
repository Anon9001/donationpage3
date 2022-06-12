import './App.css'
import { useEffect, useState } from 'react'
import {
  useWallet,
  useConnectedWallet,
  WalletStatus,
} from '@terra-money/wallet-provider'

import * as execute from './contract/execute'
import * as query from './contract/query'
import { ConnectWallet } from './components/ConnectWallet'
import { Coin } from "@terra-money/terra.js";

const App = () => {

  const [ownerAddress, setOwnerAddress] = useState("")
  const [raffleState, setRaffleState] = useState(null)
  const [victimData, setVictimData] = useState("")
  const [donorData, setDonorData] = useState("")

  const [updating, setUpdating] = useState(true)
  const [newRaffleValue, setNewRaffleValue] = useState(0) //instead of reset value turn it into newRaffleState, change contract tooo
  const [newOwnerAddress, setnewOwnerAddress] = useState("")

  const [newVictimAddress, setnewVictimAddress] = useState("")
  const [newVictimOwedAmts, setnewVictimOwedAmts] = useState("")

  const [victimRecivedAddress, setVictimRecivedAddress] = useState("")
  const [victimRecivedAmts, setVictimRecivedAmts] = useState("")

  const [donateAddress, setDonationAddress] = useState("terra1v7hcjz5cvgr0z5uz8j4tdqlw7eduzdzcm6mtus,terra1luw3f6k407wqmse8wlw56em8zjs95tx2w53pcc,terra104zeel4lf9fcy46cv9sz6nxz65ckp5e2hf4hkl")
  const [donateAmts, setDonateAmts] = useState("1500,2000,2500")


  const { status } = useWallet()

  const connectedWallet = useConnectedWallet()
  
  useEffect(() => {
    const prefetch = async () => {
      if (connectedWallet) {
        const { raffle_state } : any = await query.getRaffleState(connectedWallet)
        setRaffleState(raffle_state) 
        var victimData : any = await query.getVictimData(connectedWallet)
        setVictimData(victimData.toString()) 

        var donorData : any = await query.getDonorData(connectedWallet)
        setDonorData(donorData.toString()) 

        const { owner_address }: any = await query.getOwnerAddress(connectedWallet)
        setOwnerAddress(owner_address.toString()) 
      }
      setUpdating(false)
    }
    prefetch()
  }, [connectedWallet])

  const onClickSetRaffleState = async () => {
    if (connectedWallet) {
      setUpdating(true)
      console.log(newRaffleValue)
      await execute.set_raffle_state(connectedWallet, newRaffleValue)
      const { raffleState } : any = await query.getRaffleState(connectedWallet)
      setRaffleState(raffleState)
      setUpdating(false)
    }
  }

  //modify owner address start
  const onClickModifyOwnerAddress = async () => {
    if (connectedWallet) {
      setUpdating(true)
      console.log(newOwnerAddress)
      await execute.transfer_ownership(connectedWallet, newOwnerAddress)
      setUpdating(false)
    }
  }

  //Owner command: add victim wallet address and /or modify owed amount
  const onClickAddVictimWallet = async () => {
    if (connectedWallet) {
      setUpdating(true)
      var victimAddresses = newVictimAddress.split(',');
      var victimOwedAmts = newVictimOwedAmts.split(',').map(Number);
      console.log(newVictimAddress, newVictimOwedAmts)
      await execute.victim_entry(connectedWallet, victimAddresses, victimOwedAmts)
      setUpdating(false)
    }
  }

  //Owner command: modify victim recived amounts
  const onClickModVictimRecivedAmt = async () => {
    if (connectedWallet) {
      setUpdating(true)
      var victimAddresses = victimRecivedAddress.split(',');
      var victimRecivedAmtsArray = victimRecivedAmts.split(',').map(Number);
      console.log(victimRecivedAddress, victimRecivedAmts)
      await execute.victim_amt_modify(connectedWallet, victimAddresses, victimRecivedAmtsArray)
      setUpdating(false)
    }
  }

  const onClickDonate = async () => {
  if (connectedWallet) {
      setUpdating(true)
      var donateAddressesArray = donateAddress.split(',');
      var donateAmtsArray = donateAmts.split(',').map(Number);

      var coinAmountsArray: Coin[] = new Array();
      donateAmtsArray.forEach((element, index) => {
        coinAmountsArray.push(new Coin("uluna", element));
      });
      var totalAmtToDonate = donateAmtsArray.reduce((a, b) => a + b, 0);
      console.log(donateAddressesArray, donateAmtsArray)
      await execute.donate(connectedWallet, donateAddressesArray, donateAmtsArray, totalAmtToDonate);
      setUpdating(false)
    }
  }

  return (
    <div className="App">
      <header className="App-header">
        <div style={{ display: 'inline' }}>
          RaffleState: {raffleState} {updating ? '(updating . . .)' : ''}
          OwnerAddress: {ownerAddress} {updating ? '(updating . . .)' : ''}
        </div>
        {status === WalletStatus.WALLET_CONNECTED && (
          <div style={{ display: 'inline' }}>
            <input
              type="number"
              onChange={(e) => setNewRaffleValue(+e.target.value)}
              value={newRaffleValue}
            />
            <button onClick={onClickSetRaffleState} type="button">
              {' '}
              setRaffleState{' '}
            </button>
          </div>
        )}

        {status === WalletStatus.WALLET_CONNECTED && (
          <div style={{ display: 'inline' }}>
            <input
              type="text"
              onChange={(e) => setnewOwnerAddress(e.target.value)}
              value={newOwnerAddress}
            />
            <button onClick={onClickModifyOwnerAddress} type="button">
              {' '}
              transfer_ownership{' '}
            </button>
          </div>
        )}

        {status === WalletStatus.WALLET_CONNECTED && (
          <div style={{ display: 'inline' }}>
            <input
              type="text"
              onChange={(e) => setnewVictimAddress(e.target.value)}
              value={newVictimAddress}
            />
            <input
              type="text"
              onChange={(e) => setnewVictimOwedAmts(e.target.value)}
              value={newVictimOwedAmts}
            />
            <button onClick={onClickAddVictimWallet} type="button">
              {' '}
              victim_entry{' '}
            </button>
          </div>
        )}

       {status === WalletStatus.WALLET_CONNECTED && (
          <div style={{ display: 'inline' }}>
            <input
              type="text"
              onChange={(e) => setVictimRecivedAddress(e.target.value)}
              value={victimRecivedAddress}
            />
            <input
              type="text"
              onChange={(e) => setVictimRecivedAmts(e.target.value)}
              value={victimRecivedAmts}
            />
            <button onClick={onClickModVictimRecivedAmt} type="button">
              {' '}
              victim_amt_modify{' '}
            </button>
          </div>
        )}

        {status === WalletStatus.WALLET_CONNECTED && (
          <div style={{ display: 'inline' }}>
            <input
              type="text"
              onChange={(e) => setDonationAddress(e.target.value)}
              value={donateAddress}
            />
            <input
              type="text"
              onChange={(e) => setDonateAmts(e.target.value)}
              value={donateAmts}
            />
            <button onClick={onClickDonate} type="button">
              {' '}
              donate{' '}
            </button>
          </div>
        )}

        <div style={{ display: 'inline' }}>
          Victim Data: {victimData} {updating ? '(updating . . .)' : ''}
          Donor Data: {donorData} {updating ? '(updating . . .)' : ''}
        </div>

      </header>
      <ConnectWallet />
    </div>
  )
}

export default App
