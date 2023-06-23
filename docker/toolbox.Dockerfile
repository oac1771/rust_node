FROM ethereum/client-go:latest

USER root

RUN apk update && apk add jq postgresql curl
