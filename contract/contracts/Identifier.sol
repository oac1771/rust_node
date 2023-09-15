//SPDX-License-Identifier: Unlicense
pragma solidity >=0.8.9;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
// import "../node_modules/@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract Identifier is ERC721 {

    uint256 public currentTokenID;
    
    struct Data {
        string ipfs_addr;
        string data_hash;
    }

    mapping (uint256 => Data) public tokenIdToData;
    address[] identities;

    event AuthenticationRequest(address indexed principal, string ipfsAddress, string dataHash);
    event IpfsDeletionRequest(address indexed principal, uint256 indexed token_id, string ipfsAddress);

    constructor() ERC721("Identity Token", "IDTKN") {
        currentTokenID = 0;
    }

    modifier onlyTokenOwner(uint256 tokenId) {
        require(ownerOf(tokenId) == msg.sender, "You are not the owner of this token");
        _;
    }

    function registerIdentity(address principal, 
        string memory ipfsAddress, 
        string memory dataHash) 
    external {
        _safeMint(principal, currentTokenID);
        tokenIdToData[currentTokenID] = Data(ipfsAddress, dataHash);
        identities.push(principal);

        currentTokenID++;  
    }

    function removeIdentity(uint256 tokenId, address principal) external {
        require(principal == ERC721.ownerOf(tokenId));
        _burn(tokenId);

        for (uint i = 0; i < identities.length; i++) {
            if (identities[i] == principal) {
                delete identities[i];
            }
        }

        emit IpfsDeletionRequest(principal, tokenId, tokenIdToData[tokenId].ipfs_addr);
        delete tokenIdToData[tokenId];

    }

    function checkIdentity(address principal) view public returns(bool) {
        for (uint i = 0; i < identities.length; i++) {
            if (identities[i] == principal) {
                return true;
            }
        }
        return false;
    }

    function getIpfsAddress(uint256 tokenId) 
    view public returns(string memory) {
        return tokenIdToData[tokenId].ipfs_addr;
    }

    function getCurrentTokenID() view public returns(uint256) {
        return currentTokenID;
    }

    function authenticate(uint256 tokenId) 
    public onlyTokenOwner(tokenId) {
        emit AuthenticationRequest(msg.sender, tokenIdToData[tokenId].ipfs_addr, tokenIdToData[tokenId].data_hash);
    }
}