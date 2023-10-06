## Install Dependencies

install [rtx](https://github.com/jdx/rtx) in order to install and manage tooling:

```shell
brew install rtx
```

initialize rtx in current shell. This can be added to your bash/zshrc file:

```shell
rtx activate zsh
```

(Optional) Add this to your bash/zshrc file:
```shell
eval "$(rtx activate zsh)"
```

install tooling:

```shell
rtx install
```

(MacOS only) install nerdctl. You first have to start a colima vm with a containerd runtime in order to install nerdctl on a non linux system.

```shell
colima start --runtime containerd
```

```shell
colima nerdctl install
```

## Deploy node and blockchain dependencies to K8s

Run the following command to build the container images and deploy the node and all its dependencies to a local instance of k8s. This will also run helm tests post deployment:

```shell
task start-local BUILD=true
```
The first time you run this command it will take a long time, as the binary run in the node pod is using a statically compiled 


## helpful cmds
helpful commands:

add --api=/dns4/localhost/tcp/5001 flag to point to exposed pod in k8s

ipfs repo gc
    - initiate garbage collection
ipfs pin ls 
    - list pinned files

curl -X POST -d '{"meta_data": "info", "data": {"foo": "hi"}}' http://localhost:8000/register/0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92
curl -X DELETE http://localhost:8000/remove/0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92/0


ipfs api docs
http://docs.ipfs.tech.ipns.localhost:8080/reference/kubo/rpc/#getting-started
http://ipfscluster.io.ipns.localhost:8080/documentation/reference/api/
http://docs.ipfs.tech.ipns.localhost:8080/reference/kubo/rpc/#api-v0-bitswap-ledger



k delete pods -l test=true

https://github.com/zkSync-Community-Hub/zkync-developers/discussions/56

yarn hardhat compile



look into axum for rocket alternative

## Dependencies

colima
nerdctl (colima install nerdctl)