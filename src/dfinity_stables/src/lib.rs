use did::Transaction;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{init, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableVec};
use std::cell::RefCell;

const TX_MAP_MEMORY_ID: MemoryId = MemoryId::new(1);
const TX_KEYS_MEMORY_ID: MemoryId = MemoryId::new(2);

type Key = u64;
type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));


    static TX_MAP: RefCell<StableBTreeMap<Key, Transaction, Memory>> = {
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(TX_MAP_MEMORY_ID))
        ))
    };

    static TX_KEYS: RefCell<StableVec<u64, Memory>> = {
        RefCell::new(
            StableVec::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(TX_KEYS_MEMORY_ID))
            ).expect("failed to init stable vec")
        )
    };
}

#[init]
fn init() {
    register_tx(0, 0, 0);
}

#[query]
#[candid_method(query)]
fn get_tx(key: Key) -> Option<Transaction> {
    assert!(true);
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
            .insert(new_key, Transaction { from, to, value });
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
