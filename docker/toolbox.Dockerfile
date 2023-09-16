FROM ethereum/client-go:stable

USER root

RUN apk update && apk add jq postgresql curl bash vim
