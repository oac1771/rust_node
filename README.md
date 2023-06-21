helpful commands:

ipfs repo gc
    - initiate garbage collection
ipfs pin ls 
    - list pinned files
curl -X POST -d '{"meta_data": "info", "data": {"foo": "hi"}}' http://localhost:8000/add

pyenv shell 3.8.10
poetry env use 3.8.10
poetry shell

ipfs api docs
http://docs.ipfs.tech.ipns.localhost:8080/reference/kubo/rpc/#getting-started
http://ipfscluster.io.ipns.localhost:8080/documentation/reference/api/
http://docs.ipfs.tech.ipns.localhost:8080/reference/kubo/rpc/#api-v0-bitswap-ledger

colima start --cpu 5 --memory 8