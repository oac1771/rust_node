//SPDX-License-Identifier: Unlicense
pragma solidity >=0.8.9;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
// import "../node_modules/@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract Identifier is ERC721 {

    uint256 public tokenId;

    event AuthenticationRequest(string indexed ipfsAddress);

    mapping(uint256 => string) private nftToIpfs;

    constructor() ERC721("Identity Token", "IDTKN") {
        tokenId = 0;
    }

    function registerIdentity(address principal, string memory ipfsAddress) external {
        _safeMint(principal, tokenId);
        nftToIpfs[tokenId] = ipfsAddress;

        tokenId++;
    }
}
