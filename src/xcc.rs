use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_nft)]
pub trait ExternsContract {
    fn get_voting_power(&self, account_id: String) -> String;
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    fn resolve_get_voting_power(&self) -> String;
}