const { expect } = require("chai");
import { Wallet, Provider, Contract } from 'zksync-web3';
import * as hre from 'hardhat';
import { Deployer } from '@matterlabs/hardhat-zksync-deploy';

const DEPLOYER_CREDS = {
  "privateKey": '0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110',
  "address": "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049"
}

const PRINCIPAL_CREDS = {
  "privateKey": "0xd293c684d884d56f8d6abd64fc76757d3664904e309a0645baf8522ab6366d9e",
  "address": "0x0D43eB5B8a47bA8900d84AA36656c92024e9772e"
}

// const RICH_WALLET_PK =
//   '0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110';
// const PRINCIPAL = "0x0D43eB5B8a47bA8900d84AA36656c92024e9772e"
const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000"


async function deployIdentifier(deployer: Deployer): Promise<Contract> {
  const artifact = await deployer.loadArtifact('Identifier');
  return await deployer.deploy(artifact, []);
}

describe('Identifier', function () {
  it("Should emit transfer event when new identity is registered", async function () {
    const provider = Provider.getDefaultProvider();

    const wallet = new Wallet(DEPLOYER_CREDS["privateKey"], provider);
    const deployer = new Deployer(hre, wallet);

    const ipfsAddress = "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu"

    const identifier = await deployIdentifier(deployer);

    await expect(await identifier.registerIdentity(PRINCIPAL_CREDS["address"], ipfsAddress))
      .to.emit(identifier, 'Transfer')
      .withArgs(ZERO_ADDRESS, PRINCIPAL_CREDS["address"], 0);
    
    const ipfsAddressFromContract = await identifier.getIpfsAddress(0)
    
    expect(ipfsAddressFromContract).to.equal(ipfsAddress)
    expect(await identifier.getCurrentTokenID()).to.equal(1)
  });

  it("Should not allow non token owner to call for authentication", async function () {
    const provider = Provider.getDefaultProvider();

    const wallet = new Wallet(DEPLOYER_CREDS["privateKey"], provider);
    const deployer = new Deployer(hre, wallet);

    const ipfsAddress = "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu"
    const tokenId = 0;

    const identifier = await deployIdentifier(deployer);

    await expect(await identifier.registerIdentity(PRINCIPAL_CREDS["address"], ipfsAddress))
      .to.emit(identifier, 'Transfer')
      .withArgs(ZERO_ADDRESS, PRINCIPAL_CREDS["address"], tokenId);

    try {
      await identifier.authenticate(tokenId);
    } catch (error) {
      expect(error.message).to.have.string("execution reverted: You are not the owner of this token");
      return; 
    }
    throw new Error("Transaction did not revert as expected");

  });
});
