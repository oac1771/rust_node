const { expect } = require("chai");
const { ethers } = require("ethers");

import { Wallet, Provider, Contract } from 'zksync-web3';
import * as hre from 'hardhat';
import { Deployer } from '@matterlabs/hardhat-zksync-deploy';

import * as ContractArtifact from "../artifacts-zk/contracts/Identifier.sol/Identifier.json";

const DEPLOYER_CREDS = {
  "privateKey": '0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110',
  "address": "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049"
}

const PRINCIPAL_CREDS = {
  "privateKey": "0xd293c684d884d56f8d6abd64fc76757d3664904e309a0645baf8522ab6366d9e",
  "address": "0x0D43eB5B8a47bA8900d84AA36656c92024e9772e"
}

const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000"
const DATA_HASH = "aGFzaGVkX3N0cmluZw=="
const IPFS_ADDRESS = "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu"

async function getIdentifier(privateKey: string, contractAddress: string): Promise<Contract> {
  const provider = Provider.getDefaultProvider();
  const wallet = new Wallet(privateKey, provider);

  const contract = new ethers.Contract(
    contractAddress,
    ContractArtifact.abi,
    wallet
  );

  return contract;
}


describe('Identifier', function () {

  let deployedIdentifier;

  beforeEach(async () => {

    const provider = Provider.getDefaultProvider();
    const wallet = new Wallet(DEPLOYER_CREDS["privateKey"], provider);
    const deployer = new Deployer(hre, wallet);
    const artifact = await deployer.loadArtifact('Identifier');

    deployedIdentifier = await deployer.deploy(artifact, []);

  })

  it("Should emit transfer event when new identity is registered", async function () {

    await expect(await deployedIdentifier.registerIdentity(PRINCIPAL_CREDS["address"], IPFS_ADDRESS, DATA_HASH))
      .to.emit(deployedIdentifier, 'Transfer')
      .withArgs(ZERO_ADDRESS, PRINCIPAL_CREDS["address"], 0);
    
    const ipfsAddressFromContract = await deployedIdentifier.getIpfsAddress(0)
    
    expect(ipfsAddressFromContract).to.equal(IPFS_ADDRESS)
    expect(await deployedIdentifier.getCurrentTokenID()).to.equal(1)
    expect(await deployedIdentifier.checkIdentity(PRINCIPAL_CREDS["address"])).to.equal(true)
  });

  it("Should not allow non token owner to call for authentication", async function () {

    const tokenId = 0;

    await expect(await deployedIdentifier.registerIdentity(PRINCIPAL_CREDS["address"], IPFS_ADDRESS, DATA_HASH))
      .to.emit(deployedIdentifier, 'Transfer')
      .withArgs(ZERO_ADDRESS, PRINCIPAL_CREDS["address"], tokenId);

    await expect(deployedIdentifier.authenticate(tokenId))
      .to.be.revertedWith("You are not the owner of this token");

  });

  it("Should emit Authentication request when tokenowner attempts to authenticate", async function () {

    const contract = await getIdentifier(PRINCIPAL_CREDS["privateKey"], deployedIdentifier.address)

    const tokenId = 0;

    await expect(await deployedIdentifier.registerIdentity(PRINCIPAL_CREDS["address"], IPFS_ADDRESS, DATA_HASH))
      .to.emit(deployedIdentifier, 'Transfer')
      .withArgs(ZERO_ADDRESS, PRINCIPAL_CREDS["address"], tokenId);

    await expect(await contract.authenticate(tokenId))
      .to.emit(deployedIdentifier, 'AuthenticationRequest')
      .withArgs(IPFS_ADDRESS, DATA_HASH);

  });

  it("Should return false if identity does not exist", async function () {

    await expect(await deployedIdentifier.checkIdentity(PRINCIPAL_CREDS["address"])).to.equal(false);

  });

});
