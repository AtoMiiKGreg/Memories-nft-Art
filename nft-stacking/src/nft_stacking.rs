#![no_std]

mod events;
mod storage;
mod locked_nfts;

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait MemoriesNftStacking {
    #[init]
    fn init(&self) {}
}
