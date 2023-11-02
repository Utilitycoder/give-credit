#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, String};

#[test]
fn simpl_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GiveCreditNFTCollection);
    let client = GiveCreditNFTCollectionClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    client.initialize(&admin);

    let user1 = Address::random(&env);
    client.mock_all_auths().mint(&user1);
    assert_eq!(client.balance_of(&user1), 1);

    let user2 = Address::random(&env);
    client.mock_all_auths().mint(&user2);
    assert_eq!(client.balance_of(&user2), 1);

    client.mock_all_auths().mint(&user1);
    assert_eq!(client.balance_of(&user1), 2);

    assert_eq!(client.name(), String::from_slice(&env, "Give Credit Token"));
    assert_eq!(client.symbol(), String::from_slice(&env, "GCT"));

    let turi = client.token_uri(&0);
    let mut uri = [0u8; 67];
    let (sl, _) = uri.split_at_mut(turi.len() as usize);
    turi.copy_into_slice(sl);
    //println!("{:?}", std::str::from_utf8(uri.as_slice()));
    assert_eq!(sl, "http://localhost:3000/test/0x000.json".as_bytes());
    client.token_uri(&1).copy_into_slice(sl);
    assert_eq!(sl, "http://localhost:3000/test/0x001.json".as_bytes());
}
