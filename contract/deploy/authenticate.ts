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
  "privateKey": "0xf12e28c0eb1ef4ff90478f6805b68d63737b7f33abfa091601140805da450d93",
  "address": "0x8002cd98cfb563492a6fb3e7c82"
}

if (!PRIVATE_KEY)
  throw "⛔️ Private key not detected! Add it to the .env file!";

// Address of the contract on zksync testnet
const CONTRACT_ADDRESS = "0x8b6E8186dE74fe0128C0a6a3B2733c1365f4c9e2";

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
