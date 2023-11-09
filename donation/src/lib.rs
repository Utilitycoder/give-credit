#![no_std]
use soroban_sdk::{contract, contractimpl, contractmeta, contracttype, token, Address, Env};

pub mod give_credit_nft {
    soroban_sdk::contractimport!(file = "give_credit_nft.wasm",);
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    PubNodeAddr,
    NftAddr,
    CarbonPrice,
    TokenAddr,
    StellarCarbonAddr,
    Admin,
}

fn get_balance(e: &Env, contract_id: &Address) -> i128 {
    let client = token::Client::new(e, contract_id);
    client.balance(&e.current_contract_address())
}

fn get_token(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<_, Address>(&DataKey::TokenAddr)
        .expect("not initialized")
}

fn send_donation(e: &Env, token_id: Address) {
    let contract_balance = get_balance(e, &token_id);

    let pub_node_addr = e
        .storage()
        .instance()
        .get::<_, Address>(&DataKey::PubNodeAddr)
        .unwrap();
    let stellar_carbon_addr = e
        .storage()
        .instance()
        .get::<_, Address>(&DataKey::StellarCarbonAddr)
        .unwrap();

    // Split the amount to be sent
    if contract_balance >= 100 {
        let amount = contract_balance;
        let donation = amount * 80 / 100;
        let fee = amount - donation;

        transfer(e, &stellar_carbon_addr, &donation);
        transfer(e, &pub_node_addr, &fee);
    }
}

fn transfer(e: &Env, to: &Address, amount: &i128) {
    let token_contract_id = &get_token(e);
    let client = token::Client::new(e, token_contract_id);
    client.transfer(&e.current_contract_address(), to, amount);
}

#[contract]
pub struct GiveCredit;

#[contractimpl]
#[allow(clippy::needless_pass_by_value)]
impl GiveCredit {
    pub fn initialize(
        e: Env,
        admin: Address,
        nft_address: Address,
        pub_node_addr: Address,
        token_addr: Address,
        stellar_carbon_addr: Address,
    ) {
        e.storage().instance().set(&DataKey::NftAddr, &nft_address);
        e.storage().instance().set(&DataKey::TokenAddr, &token_addr);
        e.storage().instance().set(&DataKey::Admin, &admin);
        e.storage()
            .instance()
            .set(&DataKey::StellarCarbonAddr, &stellar_carbon_addr);
        e.storage()
            .instance()
            .set(&DataKey::PubNodeAddr, &pub_node_addr);

        let nft_contract_address: Address = e.storage().instance().get(&DataKey::NftAddr).unwrap();
        let nft_client = give_credit_nft::Client::new(&e, &nft_contract_address);
        nft_client.initialize(&e.current_contract_address());
    }

    pub fn update_carbon_price(e: Env, price: u32) {
        let admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        e.storage().instance().set(&DataKey::CarbonPrice, &price);
    }

    pub fn get_carbon_price(e: Env) -> u32 {
        let carbon_price: u32 = e
            .storage()
            .instance()
            .get(&DataKey::CarbonPrice)
            .unwrap_or(0);
        carbon_price
    }

    pub fn deposit(e: Env, user: Address, amount: i128) {
        user.require_auth();
        assert!(amount > 0, "amount must be positive");

        let token_id = get_token(&e);

        let client = token::Client::new(&e, &token_id);
        client.transfer(&user, &e.current_contract_address(), &amount);

        send_donation(&e, token_id);

        let nft_contract_address: Address = e.storage().instance().get(&DataKey::NftAddr).unwrap();
        let nft_client = give_credit_nft::Client::new(&e, &nft_contract_address);
        nft_client.mint(&user);
    }
}

mod test;
