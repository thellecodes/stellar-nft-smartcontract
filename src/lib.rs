#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Name,
    Symbol,
    Admin,
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    TokenOwner(u32),
    Seat(Address),
}

#[contract]
pub struct Eras;

#[contractimpl]
impl Eras {
    pub fn initialize(env: Env, name: String, symbol: Symbol, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("This contract already has an admin");
        }

        // Store name, symbol, and admin
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn mint(env: Env, to: Address, seat_num: u32) {
        if env.storage().instance().has(&UserDataKey::TokenOwner(seat_num)) {
            panic!("This seat is already owned");
        }

        env.storage().instance().set(&UserDataKey::TokenOwner(seat_num), &to);
    }

    pub fn owner_of(env: Env, seat_num: u32) -> Address {
        match env.storage().instance().get(&UserDataKey::TokenOwner(seat_num)) {
            Some(owner) => owner,
            None => panic!("Seat number not found"),
        }
    }

    pub fn transfer(env: Env, from: Address, to: Address, seat_num: u32) {
        from.require_auth();

        if env.storage().instance().has(&UserDataKey::TokenOwner(seat_num)) {
            let current_owner: Address = env.storage().instance().get(&UserDataKey::TokenOwner(seat_num)).unwrap();
            if current_owner != from {
                panic!("Only the current owner can transfer the token");
            }

            if env.storage().instance().has(&UserDataKey::Seat(to.clone())) {
                panic!("This receiver already has a token");
            }

            env.storage().instance().set(&UserDataKey::TokenOwner(seat_num), &to);
            env.storage().instance().set(&UserDataKey::Seat(to.clone()), &seat_num);
        } else {
            panic!("Token does not exist");
        }
    }
}
