const { expect } = require("chai");
import { Wallet, Provider, Contract } from 'zksync-web3';
import * as hre from 'hardhat';
import { Deployer } from '@matterlabs/hardhat-zksync-deploy';

const RICH_WALLET_PK =
  '0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110';
const PRINCIPAL = "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049"


async function deployIdentifier(deployer: Deployer): Promise<Contract> {
  const artifact = await deployer.loadArtifact('Identifier');
  return await deployer.deploy(artifact, []);
}

describe('Identifier', function () {
  it("Should emit transfer event when new identity is registered", async function () {
    const provider = Provider.getDefaultProvider();

    const wallet = new Wallet(RICH_WALLET_PK, provider);
    const deployer = new Deployer(hre, wallet);

    const identifier = await deployIdentifier(deployer);

    const ipfsAddress = "ipfsAddress"

    await expect(await identifier.registerIdentity(PRINCIPAL, ipfsAddress)).to.emit(identifier, 'Transfer');

    // const setGreetingTx = await greeter.setGreeting('Hola, mundo!');
    // // wait until the transaction is mined
    // await setGreetingTx.wait();

    // expect(await greeter.greet()).to.equal('Hola, mundo!');
  });
});
