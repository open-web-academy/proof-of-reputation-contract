use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue, Gas
};
type U128String = U128;

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::royalty::*;
pub use crate::events::*;

mod internal;
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;


pub const NFT_METADATA_SPEC: &str = "1.0.0";
pub const NFT_STANDARD_NAME: &str = "nep171";
pub const ICON: &str = "data:image/jpeg;base64,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";

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PoR {
    score : u64,
    quest_completed : u64,
    date_last_quest : u64
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Quest {
    score : u64,
    date : u64
}

enum QuestType {
    Quest0,
    Quest1,
    Quest2,
    Quest3,
}

impl QuestType {
    fn from_number(number: u64) -> Option<QuestType> {
        match number {
            0 => Some(QuestType::Quest0),
            1 => Some(QuestType::Quest1),
            2 => Some(QuestType::Quest2),
            3 => Some(QuestType::Quest3),
            _ => None,
        }
    }

    fn get_quest_info(&self) -> (u64, &str, &str, u64, &str) {
        match self {
            QuestType::Quest0 => (0, "I'm Human", "Identify yourself as a human in the I am human app.", 40, "QmSt8ngyrTE6JG5gwPRNsHK3VkVMv5MZf4z2d8usGoCgNg"),
            QuestType::Quest1 => (1, "stNEAR", "Stake NEAR in metapool to get stNEAR.", 15, "QmPKTBiKzuFg4G62hDtJuhuqwrrT2MbgWSLWX9LMxeTWQn"),
            QuestType::Quest2 => (2, "Meta Token", "Get META token in your account.", 15, "Qmd8kZkFhFRimwSEuHa6Qyi91E1RxCU65jdunDmEYFNYcY"),
            QuestType::Quest3 => (3, "Voting Power", "You are registered in I am human", 30, "QmZ17TnjJZEGDn1ZpjTr7cMivdxBAPWFuJhpTsm1sZr3LE",),
        }
    }
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    TokensToMintCounter,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub tokens_by_id: LookupMap<TokenId, Token>,
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    pub metadata: LazyOption<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Proof of Reputation".to_string(),
                symbol: "POR".to_string(),
                icon: Some(ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            )
        };

        //return the Contract object
        this
    }
}

#[cfg(test)]
mod tests;