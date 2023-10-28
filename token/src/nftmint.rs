use erc721::{DatakeyMetadata, ERC721Metadata, ERC721};
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, BytesN, Env, String, Symbol,
};

#[contracttype]
pub struct Id();

pub fn mint(env: &Env, to: Address, uri: String) {
    // Check ownly the admin can mint
    erc721::get_admin(env).require_auth();

    // Get and increment token id
    let token_id = env.storage().instance().get(&Id()).unwrap_or(0);
    env.storage().instance().set(&Id(), &(token_id + 1));

    // set the uri for the token id
    env.storage()
        .persistent()
        .set(&DatakeyMetadata::Uri(token_id), &uri);

    // Mint
    erc721::ERC721Contract::mint(env.clone(), to.clone(), token_id)
}

pub fn balance_of(env: Env, owner: Address) -> u32 {
    erc721::ERC721Contract::balance_of(env, owner)
}