use ethers_contract_abigen::Abigen;

fn main() {
    println!("cargo:rerun-if-changed=contract/artifacts-zk/contracts/Identifier.sol/Identifier.json");

    Abigen::new("Identifier", "contract/artifacts-zk/contracts/Identifier.sol/Identifier.json")
        .unwrap().generate().unwrap().write_to_file("src/identifier.rs").unwrap();

}