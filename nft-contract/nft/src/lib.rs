#![no_std]

use erc721::{ERC721Metadata, ERC721};
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, String,
};

#[contracttype]
pub struct Id();

#[contract]
pub struct GiveCreditNFTCollection;


#[contractimpl]
impl GiveCreditNFTCollection {
    pub fn initialize(env: Env, admin: Address) {
        let name = String::from_slice(&env, "Give Credit Token");
        let sym = String::from_slice(&env, "GCT");
        erc721::ERC721Contract::initialize(env, admin, name, sym);
    }

    pub fn upgrade(env: Env, wasm_hash: BytesN<32>) {
        erc721::ERC721Contract::upgrade(env, wasm_hash)
    }

    pub fn mint(env: Env, to: Address) {
        // Get and increment token id
        let token_id = env.storage().instance().get(&Id()).unwrap_or(0);
        env.storage().instance().set(&Id(), &(token_id + 1));

        // Mint
        erc721::ERC721Contract::mint(env.clone(), to.clone(), token_id)
    }

    pub fn balance_of(env: Env, owner: Address) -> u32 {
        erc721::ERC721Contract::balance_of(env, owner)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32) {
        erc721::ERC721Contract::transfer_from(env, spender, from, to, token_id)
    }

    pub fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    ) {
        erc721::ERC721Contract::approve(env, caller, operator, token_id, expiration_ledger)
    }

    pub fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    ) {
        erc721::ERC721Contract::set_approval_for_all(
            env,
            caller,
            owner,
            operator,
            approved,
            expiration_ledger,
        )
    }

    pub fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        erc721::ERC721Contract::get_approved(env, token_id)
    }

    pub fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        erc721::ERC721Contract::is_approval_for_all(env, owner, operator)
    }

    pub fn name(env: Env) -> String {
        erc721::ERC721Contract::name(env)
    }

    pub fn symbol(env: Env) -> String {
        erc721::ERC721Contract::symbol(env)
    }

    pub fn token_uri(env: Env, token_id: u32) -> String {
        if token_id < env.storage().instance().get(&Id()).unwrap_or(0) {
            const BASE: &str = "http://localhost:3000/test/";
            //const BASE: &str = "https://givecredit.com/test/";
            let d = to_hex(token_id);

            // concat
            let mut uri = Bytes::new(&env);
            uri.extend_from_slice(BASE.as_bytes());
            uri.extend_from_slice(d.as_slice());
            uri.extend_from_slice(".json".as_bytes());

            // Bytes to &str
            let mut slice = [0; BASE.len() + 10];
            uri.copy_into_slice(&mut slice);
            let struri = core::str::from_utf8(slice.as_slice()).unwrap();

            String::from_slice(&env, struri)
        } else {
            String::from_slice(&env, "No NFT with token_id")
        }
    }
}

fn to_hex(n: u32) -> [u8; 5] {
    let mut out = [0; 5];
    out[0] = b'0';
    out[1] = b'x';
    for i in (0..3).rev() {
        let x = ((n >> (i * 4)) & 0xf) as u8;
        let digit: u8 = match x {
            x @ 0..=9 => b'0' + x,
            x @ 10..=15 => b'a' + (x - 10),
            x => panic!("number not in the range 0..16: {}", x),
        };

        out[2 - i + 2] = digit;
    }

    out
}

mod test;