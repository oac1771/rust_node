use ethers::{
    types::{U256, Address, H160}, 
    abi::{Detokenize, Token, InvalidOutputType}
};

#[derive(Debug)]
pub struct Registration {
    pub principal: Address,
    pub token_id: U256
}

// make getter functions to return these values, and have those be from a trait 
    // so when you call the query() method it has to implement detokenize and the getter trait

impl Registration {

    pub fn new() -> Registration {
        return Registration { principal: H160::zero(), token_id: U256::zero() }
    }
    
    pub fn get_name() -> String {
        return "Registration".to_string()
    }

    pub fn get_signature() -> String {
        return "Registration(address,uint256)".to_string()
    }

    pub fn set_principal(&mut self, principal: Address) {
        self.principal = principal;
    }

    pub fn set_token_id(&mut self, token_id: U256) {
        self.token_id = token_id;
    }

}

impl Detokenize for Registration {

    fn from_tokens(tokens: Vec<Token>) -> Result<Registration, InvalidOutputType> 
    {
        let mut transfer_request = Registration::new();

        for token in tokens {
            match token {
                Token::Address(address) => {
                    let principal = address.clone();
                    transfer_request.set_principal(principal);
                },
                Token::Uint(token_id) => {
                    transfer_request.set_token_id(token_id);
                }
                _ => {return Err(InvalidOutputType("No matching Tokens found".to_string()))}
            }
        }

        return Ok(transfer_request);
    }
}








// struct IpfsDeletionRequest {
//     principal: Address,
//     ipfs_addr: String
// }

// impl IpfsDeletionRequest {
//     const NAME_WITH_SIGNATURES: &'static str = "IpfsDeletionRequest(string,address)";
//     const NAME: &'static str = "IpfsDeletionRequest";

//     pub fn new() -> IpfsDeletionRequest {
//         return IpfsDeletionRequest { principal: H160::zero(), ipfs_addr: "".to_string() }
//     }

//     pub fn set_principal(&mut self, principal: Address) {
//         self.principal = principal;
//     }

//     pub fn set_ipfs_addr(&mut self, ipfs_addr: String) {
//         self.ipfs_addr = ipfs_addr;
//     }
// }

// impl Detokenize for IpfsDeletionRequest {

//     fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> 
//     {
//         let mut ipfs_deletion_request = IpfsDeletionRequest::new();

//         for token in tokens {
//             match token {
//                 Token::Address(address) => {
//                     let principal = address.clone();
//                     ipfs_deletion_request.set_principal(principal);
//                 },
//                 Token::String(ipfs_addr) => {
//                     ipfs_deletion_request.set_ipfs_addr(ipfs_addr);
//                 }
//                 _ => {return Err(InvalidOutputType("No matching Tokens found".to_string()))}
//             }
//         }

//         return Ok(ipfs_deletion_request);
//     }
// }