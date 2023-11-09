#![cfg(test)]
extern crate std;

use crate::{GiveCredit, GiveCreditClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,
};

mod nft_contract {
    soroban_sdk::contractimport!(
        file = "/Users/tm/give-credit/nft-contract/target/wasm32-unknown-unknown/release/give_credit_nft.wasm",  
    );
}

mod token {
    soroban_sdk::contractimport!(
        file = "/Users/tm/give-credit/token/target/wasm32-unknown-unknown/release/give_credit_token.wasm",  
    );
}

struct Clients<'a> {
    token: token::Client<'a>,
    nft: nft_contract::Client<'a>,
    token_contract_id: Address,
    nft_contract_id: Address,
}

fn create_token<'a>(e: &Env) -> Clients<'a> {
    let token_contract_id = e.register_contract_wasm(None, token::WASM);
    let token = token::Client::new(e, &token_contract_id);
    let nft_contract_id = e.register_contract_wasm(None, nft_contract::WASM);
    let nft = nft_contract::Client::new(e, &nft_contract_id);
    Clients {
        token,
        nft,
        token_contract_id,
        nft_contract_id,
    }
}

fn create_carbon_contract<'a>(
    e: &Env,
    admin: &Address,
    nft_address: Address,
    pub_node_addr: &Address,
    token_addr: Address,
    stellar_carbon_addr: &Address,
) -> GiveCreditClient<'a> {
    let carbon_contract = GiveCreditClient::new(e, &e.register_contract(None, GiveCredit {}));
    carbon_contract.initialize(
        admin,
        &nft_address,
        pub_node_addr,
        &token_addr,
        stellar_carbon_addr,
    );
    carbon_contract
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let stellar_carbon = Address::random(&e);
    let pub_node = Address::random(&e);
    let client = create_token(&e);

    let carbon_client = create_carbon_contract(
        &e,
        &admin1,
        client.nft_contract_id,
        &pub_node,
        client.token_contract_id,
        &stellar_carbon,
    );

    // Mint some tokens
    client.token.initialize(&admin1, &7);
    client.token.mint(&user1, &1000);
    assert_eq!(client.token.balance(&user1), 1000);

    // Verify that Givecredit contract holds the tokens in a silo when the balance is less than 100
    carbon_client.deposit(&user1, &50);
    assert_eq!(client.token.balance(&user1), 950);
    assert_eq!(client.nft.balance_of(&user1), 1);
    assert_eq!(client.token.balance(&carbon_client.address), 50);

    // Verify that Givecredit contract sends the tokens to the pub node and stellar carbon when the balance is greater than 100
    client.token.transfer(&user1, &user2, &500);
    assert_eq!(client.token.balance(&user2), 500);

    e.budget().reset_unlimited();
    carbon_client.deposit(&user2, &100);
    assert_eq!(client.token.balance(&user2), 400);
    assert_eq!(client.nft.balance_of(&user2), 1);

    assert_eq!(client.token.balance(&stellar_carbon), 120);
    assert_eq!(client.token.balance(&pub_node), 30);
    assert_eq!(client.token.balance(&carbon_client.address), 0);
}

#[test]
fn test_carbon_price() {
    let e = Env::default();
    e.mock_all_auths();

    let pub_node = Address::random(&e);
    let stellar_carbon = Address::random(&e);
    let admin = Address::random(&e);

    let client = create_token(&e);
    let carbon_client = create_carbon_contract(
        &e,
        &admin,
        client.nft_contract_id,
        &pub_node,
        client.token_contract_id,
        &stellar_carbon,
    );

    carbon_client.update_carbon_price(&100);
    assert_eq!(carbon_client.get_carbon_price(), 100);
}
