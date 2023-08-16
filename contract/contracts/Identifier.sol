//SPDX-License-Identifier: Unlicense
pragma solidity >=0.8.9;

// import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "../node_modules/@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract Identifier is ERC721 {

    uint256 public currentTokenID;

    event AuthenticationRequest(string indexed ipfsAddress, string indexed dataHash);

    mapping(uint256 => string) public tokenIdToIpfs;
    mapping(uint256 => string) public tokenIdToDataHash;

    constructor() ERC721("Identity Token", "IDTKN") {
        currentTokenID = 0;
    }

    modifier onlyTokenOwner(uint256 tokenId) {
        require(ownerOf(tokenId) == msg.sender, "You are not the owner of this token");
        _;
    }

    // add check if principal has already been registered
    function registerIdentity(address principal, 
        string memory ipfsAddress, 
        string memory dataHash) 
    external {
        _safeMint(principal, currentTokenID);
        tokenIdToIpfs[currentTokenID] = ipfsAddress;
        tokenIdToDataHash[currentTokenID] = dataHash;

        currentTokenID++;  
    }

    function getIpfsAddress(uint256 tokenId) 
    view public returns(string memory) {
        return tokenIdToIpfs[tokenId];
    }

    function getCurrentTokenID() view public returns(uint256) {
        return currentTokenID;
    }

    function authenticate(uint256 tokenId) 
    public onlyTokenOwner(tokenId) {
        emit AuthenticationRequest(tokenIdToIpfs[tokenId], tokenIdToDataHash[tokenId]);
    }
}