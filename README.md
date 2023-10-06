## Install Dependencies

install [rtx](https://github.com/jdx/rtx) in order to install and manage tooling:

```shell
brew install rtx
```

initialize rtx in current shell.

if using zshell:
```shell
rtx activate zsh
```

if using bashrc:
```shell
rtx activate bash
```

(Optional) Add this to your bash/zshrc file:
```shell
eval "$(rtx activate zsh)"
```

install tooling:

```shell
rtx install
```

install qemu as your emulator and virtualizer:
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

Once your images are built, you can just start the k8s cluster and deploy with:

```shell
task start-local
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
