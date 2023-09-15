import { Provider } from 'zksync-web3';
import { HardhatRuntimeEnvironment } from "hardhat/types";
const { ethers } = require("ethers");

// load env file
import dotenv from "dotenv";
dotenv.config();

// load contract artifact. Make sure to compile first!
import * as ContractArtifact from "../artifacts-zk/contracts/Identifier.sol/Identifier.json";

const PRIVATE_KEY = process.env.WALLET_PRIVATE_KEY || "";
const PRINCIPAL_CREDS = {
  "privateKey": "0x3eb15da85647edd9a1159a4a13b9e7c56877c4eb33f614546d4db06a51868b1c",
  "address": "0xE90E12261CCb0F3F7976Ae611A29e84a6A85f424"
}
const DATA_HASH = "aGFzaGVkX3N0cmluZw=="
const IPFS_ADDRESS = "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu"

if (!PRIVATE_KEY)
  throw "⛔️ Private key not detected! Add it to the .env file!";

// Address of the contract on zksync testnet
const CONTRACT_ADDRESS = "0x4da8b63F2Ce2331065E9EE1ED79Fe157B2Bd3286";

if (!CONTRACT_ADDRESS) throw "⛔️ Contract address not provided";

// An example of a deploy script that will deploy and call a simple contract.
export default async function (hre: HardhatRuntimeEnvironment) {
  console.log(`Running script to interact with contract ${CONTRACT_ADDRESS}`);

  // Initialize the provider.
  // @ts-ignore

  const provider = new Provider(hre.userConfig.networks?.zkSyncTestnet?.url);
  const signer = new ethers.Wallet(PRIVATE_KEY, provider);


  // Initialize contract instance
  const contract = new ethers.Contract(
    CONTRACT_ADDRESS,
    ContractArtifact.abi,
    signer
  );

  // await contract.estimateGas.authenticate;

  const token_id = await contract.getCurrentTokenID();
  console.log(`The current tokenID is ${token_id}`);


  const wallet = new ethers.Wallet(PRINCIPAL_CREDS["privateKey"], provider);

  const owned_contract = new ethers.Contract(
    CONTRACT_ADDRESS,
    ContractArtifact.abi,
    wallet
  );

  const auth_tx = await owned_contract.authenticate(token_id-1)
  await auth_tx.wait();

  console.log(`auth tx: ${auth_tx}`);

}
