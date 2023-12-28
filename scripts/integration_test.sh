#!/usr/bin/env bash

export CONTRACT_ADDRESS=$(cd contract && yarn deploy | grep -o '0x[[:xdigit:]]\{40\}')
curl -X POST localhost:8000/bootstrap/$CONTRACT_ADDRESS
curl -X POST -d '{"meta_data": "info", "data": {"foo": "hi"}}' http://localhost:8000/register/0x8002cD98Cfb563492A6fB3E7C8243b7B9Ad4cc92 | jq '.token_id'

export RESPONSE=$(cd contract && yarn authenticate)