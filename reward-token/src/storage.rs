

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule
{
    #[view(getMemTokenId)]
    #[storage_mapper("memTokenId")]
    fn token_id(&self) -> SingleValueMapper<Self::Api,TokenIdentifier<Self::Api>>;


}