#!/usr/bin/env bash

export CONTRACT_ADDRESS=$(cd contract && yarn deploy | grep -o '0x[[:xdigit:]]\{40\}')
curl -v -X POST http://localhost:3000/bootstrap/$CONTRACT_ADDRESS

curl -v -X POST -H 'Content-Type: application/json' -d '{"principal_address": "0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92", "data": {"meta_data": "info", "data": { "foo": "hi"}}}' http://localhost:3000/register
export RESPONSE=$(cd contract && yarn authenticate)