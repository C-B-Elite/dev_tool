use ic_cdk_macros::*;
use ic_cdk::export::{candid, Principal};

// #[import(canister = "ControlCanister")]
// struct ControlCanister;

static mut COUNTER : Option<candid::Nat> = None;

#[init]
fn init() {
    unsafe {
        COUNTER = Some(candid::Nat::from(0));
    }
}

#[query]
fn get() -> candid::Nat {
    unsafe {
        COUNTER.as_mut().unwrap().clone()
    }
}

#[update]
async fn getNum(p: Principal) -> candid::Nat {
    let result: Result<(candid::Nat,), _> = ic_cdk::api::call::call(p, "get", ()).await;
    result.unwrap().0
    // ControlCanister::get().await.0
}

#[update]
async fn getBalance(p: Principal) -> candid::Nat{
    let result: Result<(candid::Nat,), _> = ic_cdk::api::call::call(p, "getBalance", ()).await;
    result.unwrap().0

    // ControlCanister::getBalance().await.0
}

// #[export_name = "canister_heartbeat"]
// async fn canister_heartbeat() {
//     unsafe {
//         COUNTER.as_mut().unwrap().0 += 1u64;
//         ControlCanister::inc().await
//     }
// }