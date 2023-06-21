FROM curlimages/curl:latest

USER root

RUN apk update && apk add jq postgresql

# wget https://gethstore.blob.core.windows.net/builds/geth-darwin-amd64-1.12.0-e501b3b0.tar.gz
# tar -xvf geth-darwin-amd64-1.12.0-e501b3b0.tar.gz
# git clone --depth=1 https://github.com/ethereum/go-ethereum.git
