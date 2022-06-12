import { LCDClient } from '@terra-money/terra.js'
import { ConnectedWallet } from '@terra-money/wallet-provider'
import { contractAdress } from './address'

export const getOwnerAddress = async (wallet: ConnectedWallet) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  })
  var ret = lcd.wasm.contractQuery(contractAdress(wallet), { get_owner_addr: {} })
  console.log("GOT HERE 1:")
  console.log(ret)
  return ret
}

export const getRaffleState = async (wallet: ConnectedWallet) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  })
  var ret = lcd.wasm.contractQuery(contractAdress(wallet), { get_raffle_state: {} })
  console.log("GOT HERE 1:")
  console.log(ret)
  return ret
}

export const getVictimData = async (wallet: ConnectedWallet) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  })

  var ret = lcd.wasm.contractQuery(contractAdress(wallet), { get_victim_data: {} })
  console.log("GOT HERE 2:")
  console.log(ret)
  return ret
}

export const getDonorData = async (wallet: ConnectedWallet) => {
  const lcd = new LCDClient({
    URL: wallet.network.lcd,
    chainID: wallet.network.chainID,
  })

  var ret = lcd.wasm.contractQuery(contractAdress(wallet), { get_donor_data: {} })
  console.log("GOT HERE 3:")
  console.log(ret)
  return ret
}

