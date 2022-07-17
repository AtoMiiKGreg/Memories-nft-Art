elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait EventsModule {
    #[event("stakeEvent")]
    fn stake_event(
        &self,
        #[indexed] user_address: &ManagedAddress<Self::Api>,
        entry_after_action: &UserEntry<Self::Api>,
    );

    #[event("unstakeEvent")]
    fn unstake_event(
        &self,
        #[indexed] user_address: &ManagedAddress<Self::Api>,
        entry_after_action: &UserEntry<Self::Api>,
    );

    #[event("unbondEvent")]
    fn unbond_event(
        &self,
        #[indexed] user_address: &ManagedAddress<Self::Api>,
        opt_entry_after_action: Option<&UserEntry<Self::Api>>,
    );
}
