import { Provider } from "zksync-web3";
import * as ethers from "ethers";
import { HardhatRuntimeEnvironment } from "hardhat/types";

// load env file
import dotenv from "dotenv";
dotenv.config();

// load contract artifact. Make sure to compile first!
import * as ContractArtifact from "../artifacts-zk/contracts/Identifier.sol/Identifier.json";

const PRIVATE_KEY = process.env.WALLET_PRIVATE_KEY || "";
const PRINCIPAL_ADDRESS = "0x0D43eB5B8a47bA8900d84AA36656c92024e9772e";
const DATA_HASH = "aGFzaGVkX3N0cmluZw=="
const IPFS_ADDRESS = "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu"

if (!PRIVATE_KEY)
  throw "⛔️ Private key not detected! Add it to the .env file!";

// Address of the contract on zksync testnet
const CONTRACT_ADDRESS = "0xdf5445d8518Ab352f721dAf6D945E20795e6A8A8";

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

  console.log(`The current tokenID is ${await contract.getCurrentTokenID()}`);

  // send transaction to update the message
  const tx = await contract.registerIdentity(PRINCIPAL_ADDRESS, IPFS_ADDRESS, DATA_HASH);

  console.log(`Transaction to change the message is ${tx.hash}`);
  await tx.wait();

  // // Read message after transaction
  console.log(`The current tokenID is ${await contract.getCurrentTokenID()}`);

}
