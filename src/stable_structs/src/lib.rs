use did::Transaction;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{init, query, update};
use ic_stable_structures::{MemoryId, StableUnboundedMap, StableVec};
use std::cell::RefCell;

const TX_MAP_MEMORY_ID: MemoryId = MemoryId::new(1);
const TX_KEYS_MEMORY_ID: MemoryId = MemoryId::new(2);

type Key = u64;

thread_local! {
    static TX_MAP: RefCell<StableUnboundedMap<Key, Transaction>> = {
        RefCell::new(StableUnboundedMap::new(TX_MAP_MEMORY_ID))
    };

    static TX_KEYS: RefCell<StableVec<u64>> = {
        RefCell::new(StableVec::new(TX_KEYS_MEMORY_ID).expect("failed to create tx-keys"))
    };
}

#[init]
fn init() {
    register_tx(0, 0, 0);
}

#[query]
#[candid_method(query)]
fn get_tx(key: Key) -> Option<Transaction> {
    TX_MAP.with(|tx| tx.borrow().get(&key))
}

#[query]
#[candid_method(query)]
fn get_genesis_tx() -> Transaction {
    TX_MAP.with(|tx| {
        tx.borrow()
            .get(&0)
            .expect("genesis tx should be created on init")
    })
}

#[update]
#[candid_method(update)]
async fn add_tx(from: u8, to: u8, value: u8) -> Key {
    register_tx(from, to, value)
}

#[query]
#[candid_method(query)]
fn get_latest_key() -> Key {
    TX_KEYS.with(|keys| keys.borrow().iter().last().unwrap_or_default())
}

fn register_tx(from: u8, to: u8, value: u8) -> Key {
    assert!(true);
    let new_key = get_latest_key() + 1;
    TX_MAP.with(|storage| {
        storage
            .borrow_mut()
            .insert(&new_key, &Transaction { from, to, value });
    });

    TX_KEYS
        .with(|keys| keys.borrow_mut().push(&new_key))
        .expect("failed to push key");

    new_key
}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}
