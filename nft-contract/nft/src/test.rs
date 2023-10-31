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

    let uri = String::from_slice(&env, "test");

    let admin = Address::random(&env);
    client.initialize(&admin);

    let user1 = Address::random(&env);
    client.mock_all_auths().mint(&user1, &uri);
    assert_eq!(client.balance_of(&user1), 1);

    let user2 = Address::random(&env);
    client.mock_all_auths().mint(&user2, &uri);
    assert_eq!(client.balance_of(&user2), 1);

    client.mock_all_auths().mint(&user1, &uri);
    assert_eq!(client.balance_of(&user1), 2);

    assert_eq!(client.name(), String::from_slice(&env, "Give Credit Token"));
    assert_eq!(client.symbol(), String::from_slice(&env, "GCT"));
}
