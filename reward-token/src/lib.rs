#![no_std]

mod storage;

const NUM_DECIMAL: usize = 6;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait TokenRewardMem :
    storage::StorageModule

{
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        token_display_name: ManagedBuffer<Self::Api>,
        token_ticker: ManagedBuffer<Self::Api>,
    ) {
        require!(self.token_id().is_empty(),"Token already issued");

        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        let initial_supply = BigUint::zero();

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: NUM_DECIMAL,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress<Self::Api>,
        #[call_result] result: ManagedAsyncCallResult<Self::Api, TokenIdentifier<Self::Api>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.token_id().set(&token_identifier);
            },
            ManagedAsyncCallResult::Err(_) => {
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() {
                    self.send()
                        .direct(&caller, &returned.token_identifier, 0,&returned.amount);
                }
            }
        }
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        require!(
            !self.token_id().is_empty(),
            "Must issue token first"
        );

        let roles = [EsdtLocalRole::Mint, EsdtLocalRole::Burn];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.token_id().get(),
                roles[..].iter().cloned(),
            )
            .async_call()
            .call_and_exit()
    }


    #[only_owner]
    #[endpoint(mint)]
    fn mint(
        &self,
        amount: BigUint<Self::Api>,
    ){
        require!(amount>0,"The number of token mint can't be less than 1");
        require!(!self.token_id().is_empty(),"Token must be issued");

        let token = self.token_id().get();
        let roles = self.blockchain().get_esdt_local_roles(&token);

        require!(roles.has_role(&EsdtLocalRole::Mint),"Esdt local role mint not set");

        self.send().esdt_local_mint(&token,0,&amount);

    }
    #[only_owner]
    #[endpoint(burn)]
    fn burn(
        &self,
        amount: BigUint<Self::Api>
    ) {
        require!(amount>0,"The number of token burn can't be less than 1");
        require!(!self.token_id().is_empty(),"Token must be issued");

        let token = self.token_id().get();
        let roles = self.blockchain().get_esdt_local_roles(&token);

        require!(roles.has_role(&EsdtLocalRole::Burn),"Esdt local role mint not set");

        self.send().esdt_local_burn(&token, 0, &amount);
    }


    #[view(getMemBalance)]
    fn get_mem_balance(&self) -> BigUint<Self::Api> {
        let mem_id = self.token_id().get();
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(mem_id), 0)

        }
}

