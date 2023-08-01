//SPDX-License-Identifier: Unlicense
pragma solidity >=0.8.9;

// import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "../node_modules/@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract Identifier is ERC721 {

    uint256 public tokenId;

    event AuthenticationRequest(string indexed ipfsAddress);

    mapping(uint256 => string) public tokenIdToIpfs;
    mapping(uint256 => string) public tokenIdToPrincipal;

    constructor() ERC721("Identity Token", "IDTKN") {
        tokenId = 0;
    }

    modifier onlyTokenOwner(uint256 tokenId) {
        require(ownerOf(tokenId) == msg.sender, "You are not the owner of this token");
        _;
    }

    function registerIdentity(address principal, string memory ipfsAddress) 
    external {
        _safeMint(principal, tokenId);
        tokenIdToIpfs[tokenId] = ipfsAddress;

        tokenId++;
    }

    function getIpfsAddress(uint256 tokenId) 
    view public returns(string memory) {
        return tokenIdToIpfs[tokenId];
    }

    function getCurrentTokenID() view public returns(uint256) {
        return tokenId;
    }

    function authenticate(uint256 tokenId) 
    public onlyTokenOwner(tokenId) returns(string memory){
        return "hi";
    }
}