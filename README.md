## Install Dependencies

install [rtx](https://github.com/jdx/rtx) in order to install and manage tooling:
 (MacOs)
```shell
brew install rtx
```

initialize rtx in current shell.

if using zshell:
```shell
eval "$(rtx activate zsh)"
```

if using bashrc:
```shell
eval "$(rtx activate bash)"
```

(Optional) Add previous eval command to your bash/zshrc file so you do not need to manually initialize rtx in a new terminal session.

install tooling:

```shell
rtx install
```

(MacOs)
```shell
brew install qemu
```

(MacOS only) install nerdctl. You first have to start a colima vm with a containerd runtime in order to install nerdctl on a non linux system.

```shell
colima start --runtime containerd
```

```shell
colima nerdctl install
```

```shell
colima delete
```

## Deploy node and blockchain dependencies to K8s

Run the following command to build the container images and deploy the node and all its dependencies to a local instance of k8s. This will also run helm tests post deployment:

```shell
task start-local BUILD=true
```
The first time you run this command it will take a long time, as the binary running in the node pod is statically compiled using musl. Subsequent runs should be much faster

Once your images are built, you can start the k8s cluster and deploy with:

```shell
task start-local
```


## Interacting with cluster

Note: You will need private keys when interacting with anything on Zksync. I have used keys listed in this [file](https://github.com/matter-labs/local-setup/blob/main/rich-wallets.json). Save one of these private keys in a `.env` file in the `contract` directory. This will be used to deploy the smart contract and use non token owned functions.

Deploy your smart contract to zksync by running the following commands. First, in a seperate terminal window you must port-forward to the zksync service:

```shell 
kubectl port-forward svc/zksync 3050:3050
```

In a seperate terminal session, cd into the contract directory and deploy the contract
```shell 
cd contract
```

```shell
yarn deploy
```

Save the contract address as an env var: `CONTRACT_ADDRESS`

You now have to port-forward to the node instance in a seperate terminal session in order to interact with the api locally:

```shell
kubectl port-forward svc/node 8000:8000
```

Now bootstrap the api by providing the contract address:

```shell
curl -X POST localhost:8000/bootstrap/$CONTRACT_ADDRESS
```

Register Identity to smart contract:

```shell
curl -v -X POST -H 'Content-Type: application/json' -d '{"principal_address": "0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92", "data": {"meta_data": "info", "data": { "foo": "hi"}}}' http://localhost:3000/register
```

Now send authentication request to smart contract which will emit an event that our node is listening for. You must update the `PRINCIPAL_CREDS` variable and provide the address and private key of the wallet that you registered earlier. 

```shell
cd contract
```
```shell
yarn authenticate
```

You should see `Authentication Successful!` in the logs of the node pod if all worked correctly:

```shell
kubectl logs -l app=node
```

## Integration Test
Alternatively you can run an integration test that will do all of the steps above. You must have a port-forward open for ipfs, zksync http port and zksync ws port:

```shell
kubectl port-forward svc/zksync 3050:3050
```
```shell
kubectl port-forward svc/zksync 3051:3051
```
```shell
kubectl port-forward svc/ipfs 5001:5001
```

```shell
task cargo-run
```

```
task integration-test
```

## Clean up

To stop your k8s cluster run the following command:

```shell
colima stop
```

To destroy all local resources, run the following command:
```shell
task destroy-local
```

## Helpful Commands

### General

port forward to k8s service

```shell
kubectl port-forward svc/<service name> <local port>:<remote port>
```

delete helm test pods:

```shell
kubectl delete pods -l test=true
```
### ipfs
port forward to ipfs svc:

```shell
kubectl port-forward svc/ipfs 5001:5001
```

initiate garbage collection:

```shell
ipfs repo gc --api=/dns4/localhost/tcp/5001 
```

checked pinned data:

```shell
ipfs pin ls --api=/dns4/localhost/tcp/5001 
```


yarn hardhat compile
