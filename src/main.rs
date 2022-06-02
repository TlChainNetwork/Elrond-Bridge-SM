#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::contract]
pub trait Bridge {
    // smart contract constructor
    #[init]
    fn init(
        &self,
        fee_amount: BigUint
    ) -> SCResult<()> {

        self.fee_amount().set(&fee_amount);

        let token_id = TokenIdentifier::egld();
        self.accepted_payment_token_id().set(&token_id);

        self.total_fee_amount().set(BigUint::zero());

        Ok(())
    }

    // endpoints

    /// User sends a fixed amount tokens to be locked in the contract .
    /// Optional `_data` argument is ignored.
    #[payable("*")]
    #[endpoint]
    fn deposit(
        &self,
        #[payment_token] payment_token: TokenIdentifier,
        #[payment_amount] payment_amount: BigUint,
    ) -> SCResult<()> {

        //You can only deposit egld
        require!(
            payment_token == self.accepted_payment_token_id().get(),
            "Invalid payment token"
        );

        //You can only deposit a fixed amount
        require!(
            payment_amount > self.fee_amount().get(),
            "The payment must be bigger than fee amount"
        );

        //You can only deposit once
        let caller = self.blockchain().get_caller();

        //Set a mapping between the address and the timestamp
        let current_block_timestamp = self.blockchain().get_block_timestamp();

        self.user_last_deposit(&caller).set(&current_block_timestamp);

        self.total_fee_amount().update(|v| *v += &self.fee_amount().get());

        //Everything went fine
        Ok(())
    }

    /// User can take back funds from the contract but only the admin can send a withdraw request.
    #[endpoint]
    fn withdraw(&self,
            address: &ManagedAddress,
            withdraw_amount: BigUint,
    ) -> SCResult<()> {

        // Check that who calls this contract function is the owner
        self.blockchain().check_caller_is_owner();

        let token_id = self.accepted_payment_token_id().get();

        require!(
            self.blockchain().get_sc_balance(&self.accepted_payment_token_id().get(), 0) >= withdraw_amount,
            "not enough egld in smart contract to withdraw"
        );

        self.send()
            .direct(&address, &token_id, 0, &withdraw_amount, b"withdraw successful");

        //Everything went fine
        Ok(())
    }

    #[endpoint]
    fn withdrawFee(&self,
            opt_destination_address: OptionalValue<ManagedAddress>,
    ) -> SCResult<()> {

        // Check that who calls this contract function is the owner
        self.blockchain().check_caller_is_owner();

        let total_fee_amount = self.total_fee_amount().get();

        // Check that the address already deposited
        require!(
            total_fee_amount > BigUint::zero(), 
            "No fee to withdraw"
        );

        let destination_address = match opt_destination_address {
            OptionalValue::Some(value) => {
                value
            },
            OptionalValue::None => self.blockchain().get_caller()
        };

        let token_id = self.accepted_payment_token_id().get();

        self.total_fee_amount().set(&BigUint::zero());

        require!(
            self.blockchain().get_sc_balance(&token_id, 0) >= total_fee_amount,
            "not enough egld in smart contract to withdraw fee"
        );

        self.send()
            .direct(&destination_address, &token_id, 0, &total_fee_amount, b"withdraw successful");

        //Everything went fine
        Ok(())
    }

    // views

    #[view(getAcceptedPaymentToken)]
    #[storage_mapper("acceptedPaymentTokenId")]
    fn accepted_payment_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getFeeAmount)]
    #[storage_mapper("feeAmount")]
    fn fee_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getUserDeposit)]
    #[storage_mapper("userDeposit")]
    fn user_last_deposit(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(totalFeeAmount)]
    #[storage_mapper("totalFeeAmount")]
    fn total_fee_amount(&self) -> SingleValueMapper<BigUint>;
}
