## Install Dependencies

install [rtx](https://github.com/jdx/rtx) in order to install and manage tooling:

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

Save the contract address, you will need it later to bootstrap the api.

You now have to port-forward to the node instance in a seperate terminal session in order to interact with the api locally:

```shell
kubectl port-forward svc/node 8000:8000
```

Now bootstrap the api by providing the contract address:

```shell
curl -X POST localhost:8000/bootstrap/<your contract address>
```

Register Identity to smart contract:

```shell
curl -X POST -d '{"meta_data": "info", "data": {"foo": "hi"}}' http://localhost:8000/register/<your wallet address>
```

Now send authentication request to smart contract which will emit an event that our node is listening for. You need to update the `CONTRACT_ADDRESS` variable to be the value of the address that was returned from `yarn deploy`. Additionally, you must update the `PRINCIPAL_CREDS` variable and provide the address and private key of the wallet that you registered earlier. 

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
