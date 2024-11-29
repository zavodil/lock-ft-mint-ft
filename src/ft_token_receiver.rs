use crate::*;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    #[allow(unused_variables)]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(
            env::predecessor_account_id(),
            self.token_id_to_lock,
            "Invalid token ID"
        );

        self.ft_transfer(
            sender_id,
            amount,
            Some(format!("Transfer on #{}", env::block_height())),
        );

        env::log_str("Thank you ser!");

        PromiseOrValue::Value(0.into())
    }
}
