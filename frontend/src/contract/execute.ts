import { LCDClient, MsgExecuteContract, Fee, Int, Coins, Coin } from "@terra-money/terra.js";
import { ConnectedWallet } from "@terra-money/wallet-provider";
import { contractAdress } from "./address";
//import { JSONSerializable } from "@terra-money/terra.js/dist/util/json";

// ==== utils ====

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const until = Date.now() + 1000 * 60 * 60;
const untilInterval = Date.now() + 1000 * 60;

const _exec = (msg: any, totalLunaAmt: Number) =>
  async (wallet: ConnectedWallet) => {
    const lcd = new LCDClient({
      URL: wallet.network.lcd,
      chainID: wallet.network.chainID,
    });
    console.log('msg', msg)
//    totalLunaAmt = 0;
    console.log("totalLunaAmt:")
    console.log(totalLunaAmt)
    
    
    var msgExeCon =         new MsgExecuteContract(
      wallet.walletAddress,
      contractAdress(wallet),
      msg,
    )

    if (totalLunaAmt > 0) {
      //    const c = new Coin("uluna", 1500); // .0015 LUNA
      const c = new Coin("uluna", totalLunaAmt.valueOf()); // .0015 LUNA
      msgExeCon =         new MsgExecuteContract(
        wallet.walletAddress,
        contractAdress(wallet),
        msg,
      new Coins([c])
      )
    }
    const { result } = await wallet.post({
      msgs: [
        msgExeCon,
      ],
    });

    while (true) {
      try {
        return await lcd.tx.txInfo(result.txhash);
      } catch (e) {
        if (Date.now() < untilInterval) {
          await sleep(500);
        } else if (Date.now() < until) {
          await sleep(1000 * 10);
        } else {
          throw new Error(
            `Transaction queued. To verify the status, please check the transaction hash: ${result.txhash}`
          );
        }
      }
    }
  };



// ==== execute contract ====

export const increment = _exec({ increment: {} },0);

export const set_raffle_state = async (wallet: ConnectedWallet, new_raffle_value: number) =>
  _exec({ set_raffle_state: { new_raffle_value } },0)(wallet);

export const transfer_ownership = async (wallet: ConnectedWallet, address: string) =>
  _exec({ transfer_ownership: { address } },0)(wallet);

  export const victim_entry = async (wallet: ConnectedWallet, victims: any[]) =>
  _exec({ victim_entry: { victims } },0)(wallet);

  export const victim_amt_modify = async (wallet: ConnectedWallet, victims: any[]) =>
  _exec({ victim_amt_modify: { victims } },0)(wallet);

  export const donate = async (wallet: ConnectedWallet, donations: any[], totalLunaAmt: number) =>
  _exec({ donate: { donations } },  totalLunaAmt)(wallet);

