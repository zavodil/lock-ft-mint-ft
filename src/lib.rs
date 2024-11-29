mod ft_token_receiver;

use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::fungible_token::{
    FungibleToken, FungibleTokenCore, FungibleTokenResolver,
};
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::near;
use near_sdk::serde::Serialize;
use near_sdk::{
    env, log, near_bindgen, AccountId, BorshStorageKey, NearToken, PanicOnDefault, PromiseOrValue,
};

pub type Balance = u128;
const TRANSFER_ERROR: &str = "Tokens are non-transferable";

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
enum StorageKey {
    Ft,
    FtMeta,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    #[serde(skip)]
    pub ft: FungibleToken,

    pub token_id_to_lock: AccountId,

    #[serde(skip)]
    pub metadata: LazyOption<FungibleTokenMetadata>,
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[near_bindgen]
impl FungibleTokenCore for Contract {
    #[payable]
    #[allow(unused_variables)]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        // allow only token_id_to_lock to initiate a transfer
        if env::predecessor_account_id() != self.token_id_to_lock {
            env::panic_str(TRANSFER_ERROR);
        }
        // register new account if needed
        if !self.ft.accounts.contains_key(&receiver_id) {
            self.ft.internal_register_account(&receiver_id);
        }

        self.ft
            .internal_transfer(&env::current_account_id(), &receiver_id, amount.0, memo);
    }

    #[payable]
    #[allow(unused_variables)]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        env::panic_str(TRANSFER_ERROR);
    }

    fn ft_total_supply(&self) -> U128 {
        U128::from(self.ft.total_supply)
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.ft.ft_balance_of(account_id)
    }
}

#[near]
impl FungibleTokenResolver for Contract {
    #[private]
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128 {
        let (used_amount, burned_amount) =
            self.ft
                .internal_ft_resolve_transfer(&sender_id, receiver_id, amount);
        if burned_amount > 0 {
            log!("Account @{} burned {}", sender_id, burned_amount);
        }
        used_amount.into()
    }
}

#[near]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.ft.storage_deposit(account_id, registration_only)
    }

    #[payable]
    #[allow(unused_variables)]
    fn storage_withdraw(&mut self, amount: Option<NearToken>) -> StorageBalance {
        env::panic_str("Not available")
    }

    #[payable]
    #[allow(unused_variables)]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        env::panic_str("Not available")
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        self.ft.storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.ft.storage_balance_of(account_id)
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(token_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        metadata.assert_valid();
        let mut ft = FungibleToken::new(StorageKey::Ft);
        ft.internal_register_account(&env::current_account_id());
        ft.internal_deposit(&env::current_account_id(), total_supply.into());

        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &env::current_account_id(),
            amount: total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();

        Self {
            ft,
            token_id_to_lock: token_id,
            metadata: LazyOption::new(StorageKey::FtMeta, Some(&metadata)),
        }
    }
}

