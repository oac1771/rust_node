FROM ethereum/client-go:v1.13.0

USER root

RUN apk update && apk add jq postgresql curl bash vim
