#![cfg(test)]
extern crate std;

use crate::{contract::Token, TokenClient};
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

struct Clients<'a> {
    token: TokenClient<'a>,
    nft: nft_contract::Client<'a>,
}

fn create_token<'a>(e: &Env, admin: &Address, pub_node: &Address) -> Clients<'a> {
    let token = TokenClient::new(e, &e.register_contract(None, Token {}));
    let nft_contract_id = e.register_contract_wasm(None, nft_contract::WASM);
    let nft = nft_contract::Client::new(e, &nft_contract_id);
    token.initialize(admin, pub_node, &nft_contract_id, &7);
    Clients { token, nft }
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = Address::random(&e);
    let admin2 = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let pub_node = Address::random(&e);
    let client = create_token(&e, &admin1, &pub_node);



    client.token.mint(&user1, &1000);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, 1000_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(client.token.balance(&user1), 1000);

    client.token.approve(&user2, &user3, &500, &200);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(client.token.allowance(&user2, &user3), 500);

    client.token.transfer(&user1, &user2, &600);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(client.token.balance(&user1), 400);
    assert_eq!(client.nft.balance_of(&user1), 1);
    assert_eq!(client.token.balance(&user2), 480);
    std::println!("{}", client.token.balance(&user2));
    assert_eq!(client.token.balance(&pub_node), 120);

    client.token.transfer_from(&user3, &user2, &user1, &400);
    assert_eq!(
        e.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.token.address.clone(),
                    Symbol::new(&e, "transfer_from"),
                    (&user3, &user2, &user1, 400_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(client.token.balance(&user1), 800);
    assert_eq!(client.token.balance(&user2), 80);

    client.token.transfer(&user1, &user3, &300);
    assert_eq!(client.token.balance(&user1), 500);
    assert_eq!(client.nft.balance_of(&user1), 2);
    assert_eq!(client.token.balance(&user3), 240);
    assert_eq!(client.token.balance(&pub_node), 180);

    client.token.set_admin(&admin2);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.token.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // Increase to 500
    client.token.approve(&user2, &user3, &500, &200);
    assert_eq!(client.token.allowance(&user2, &user3), 500);
    client.token.approve(&user2, &user3, &0, &200);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(client.token.allowance(&user2, &user3), 0);
}

#[test]
fn test_burn() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let client = create_token(&e, &admin, &user3);

    client.token.mint(&user1, &1000);
    assert_eq!(client.token.balance(&user1), 1000);

    client.token.approve(&user1, &user2, &500, &200);
    assert_eq!(client.token.allowance(&user1, &user2), 500);

    client.token.burn_from(&user2, &user1, &500);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.token.address.clone(),
                    symbol_short!("burn_from"),
                    (&user2, &user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(client.token.allowance(&user1, &user2), 0);
    assert_eq!(client.token.balance(&user1), 500);
    assert_eq!(client.token.balance(&user2), 0);

    client.token.burn(&user1, &500);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    client.token.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(client.token.balance(&user1), 0);
    assert_eq!(client.token.balance(&user2), 0);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn transfer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let client = create_token(&e, &admin, &user3);

    client.token.mint(&user1, &1000);
    assert_eq!(client.token.balance(&user1), 1000);

    client.token.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let client = create_token(&e, &admin, &user3);

    client.token.mint(&user1, &1000);
    assert_eq!(client.token.balance(&user1), 1000);

    client.token.approve(&user1, &user3, &100, &200);
    assert_eq!(client.token.allowance(&user1, &user3), 100);

    client.token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic(expected = "already initialized")]
fn initialize_already_initialized() {
    let e = Env::default();
    let admin = Address::random(&e);
    let user3 = Address::random(&e);
    let nft_address = Address::random(&e);
    let client = create_token(&e, &admin, &user3);

    client.token.initialize(&admin, &user3, &nft_address, &10);
}

#[test]
#[should_panic(expected = "Decimal must fit in a u8")]
fn decimal_is_over_max() {
    let e = Env::default();
    let admin = Address::random(&e);
    let user3 = Address::random(&e);
    let token = TokenClient::new(&e, &e.register_contract(None, Token {}));
    token.initialize(
        &admin,
        &user3,
        &Address::random(&e),
        &(u32::from(u8::MAX) + 1),
    );
}