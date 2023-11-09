use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json,from_str};
use near_sdk::{env,ext_contract, Balance,Gas, near_bindgen, AccountId, PromiseOrValue, PromiseResult, PanicOnDefault, log, BorshStorageKey, require};
use near_sdk::json_types::{U128, U64};
use near_sdk::Promise;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};

pub const IMAGE: &str = "QmdBddzRiQfWDs5uAovq4jxoBtsAKeJAipoqHAefmhoLBs";

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn nft_mint_quest(&mut self, quest_number: u64) -> Promise {
        let receiver_id = env::signer_account_id();
        let initial_storage_usage = env::storage_usage();
        let deposit = env::attached_deposit();

        let mut has_por_nft = false;
        let mut has_quest_nft = false;

        if let Some(quest) = QuestType::from_number(quest_number) {
            let (id, name, description, score, image) = quest.get_quest_info();

            let tokens_for_owner_set = self.tokens_per_owner.get(&receiver_id);
            let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
                tokens_for_owner_set
            } else {
                near_sdk::collections::UnorderedSet::new(b"tokens".to_vec()) 
            };

            if !tokens.is_empty() {
                let start = u128::from(U128(0));
                has_quest_nft = tokens.iter()
                .skip(start as usize)
                .filter_map(|token_id| {
                    let token = self.nft_token(token_id.clone()).unwrap();
                    let title = token.metadata.title.unwrap_or_default();
                    if title == name.to_string() {
                        Some(true)
                    } else {
                        Some(false)
                    }
                })
                .any(|value| value);

                has_por_nft = tokens.iter()
                .skip(start as usize)
                .filter_map(|token_id| {
                    let token = self.nft_token(token_id.clone()).unwrap();
                    let title = token.metadata.title.unwrap_or_default();
                    if title == "Proof Of Reputation NFT".to_string() {
                        Some(true)
                    } else {
                        Some(false)
                    }
                })
                .any(|value| value);
            }

            if has_quest_nft {
            env::panic_str("Quest token has already been minted");
            }

            let accountid = env::signer_account_id().clone();
            // Im Human
            if quest_number == 0 {
                let call0 = ext_nft::is_human(
                    accountid.clone().to_string(),
                    "registry.i-am-human.near".parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    Gas(100_000_000_000_000)
                );
                let callback0 = ext_self::resolve_mint_im_human(
                    name.to_string(), description.to_string(), score, image.to_string(), has_por_nft,
                    "proof-of-reputation.near".parse::<AccountId>().unwrap(),
                    deposit,
                    Gas(100_000_000_000_000)
                );
                return call0.then(callback0);
            }
            // stNEAR
            if quest_number == 1 {
                let call1 = ext_nft::ft_balance_of(
                    accountid.clone().to_string(),
                    "meta-pool.near".parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    Gas(100_000_000_000_000)
                );
                let callback1 = ext_self::resolve_mint_st_near(
                    name.to_string(), description.to_string(), score, image.to_string(), has_por_nft,
                    "proof-of-reputation.near".parse::<AccountId>().unwrap(),
                    deposit,
                    Gas(100_000_000_000_000)
                );
                return  call1.then(callback1);
            }
            // Meta Token
            if quest_number == 2 {
                let call2 = ext_nft::ft_balance_of(
                    accountid.clone().to_string(),
                    "meta-token.near".parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    Gas(100_000_000_000_000)
                );
                let callback2 = ext_self::resolve_mint_meta_token(
                    name.to_string(), description.to_string(), score, image.to_string(), has_por_nft,
                    "proof-of-reputation.near".parse::<AccountId>().unwrap(),
                    deposit,
                    Gas(100_000_000_000_000)
                );
                return call2.then(callback2);
            }
            // Voting Power
            if quest_number == 3 {
                let call3 = ext_nft::get_locked_balance(
                    accountid.clone().to_string(),
                    "meta-vote.near".parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    Gas(100_000_000_000_000)
                );
                let callback3 = ext_self::resolve_mint_voting_power(
                    name.to_string(), description.to_string(), score, image.to_string(), has_por_nft,
                    "proof-of-reputation.near".parse::<AccountId>().unwrap(),
                    deposit,
                    Gas(100_000_000_000_000)
                );
                return call3.then(callback3);
            }
            env::panic_str("Token has already been minted");
        } else {
            env::panic_str("Quest not found");
        }

    }

    #[payable]
    pub fn resolve_mint_im_human(&mut self, name: String, description: String, score: u64, image: String, has_por_nft: bool) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                return "Error al obtener el valor".to_string();
            },
            PromiseResult::Successful(result) => {
                let value = std::str::from_utf8(&result).unwrap();
                log!("Registrado en Im Human: {}",value);

                let receiver_id = env::signer_account_id();
                let initial_storage_usage = env::storage_usage();
                if value != "[]" {
                    // Minar
                    log!("Registrado en Im Human: {}",value);
                    let mut new_token = TokenMetadata {
                        title:  Some(name.to_string()), 
                        description:  Some(description.to_string()),
                        media:  Some(image.to_string()),
                        expires_at: None,
                        starts_at: None,
                        copies: None,
                        extra: None,
                        issued_at: None,
                        media_hash: None,
                        reference: None,
                        reference_hash: None,
                        updated_at: None
                    };
                
                        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                
                        let mut empty_quest = Quest {
                            score : 0,
                            date: 0
                        };
                
                        empty_quest.score = score;
                        empty_quest.date = env::block_timestamp();
                
                        let mut extra_data_string = serde_json::to_string(&empty_quest).unwrap();
                        extra_data_string = str::replace(&extra_data_string, "\"", "'");
                        new_token.extra = Some(extra_data_string);
                
                        let mut royalty = HashMap::new();
                
                        let token = Token {
                            owner_id: receiver_id.clone(),
                            approved_account_ids: Default::default(),
                            next_approval_id: 0,
                            royalty
                        };
                
                        assert!(
                            self.tokens_by_id.insert(&token_id, &token).is_none(),
                            "Token already exists"
                        );
                
                        self.token_metadata_by_id.insert(&token_id, &new_token);
                        self.internal_add_token_to_owner(&token.owner_id, &token_id);
                
                        // Validar si ya tiene el POR NFT
                        if has_por_nft {
                            log!("Ya tiene el POR NFT");

                            let tokens_for_owner_set = self.tokens_per_owner.get(&receiver_id);
                            let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
                                tokens_for_owner_set
                            } else {
                                near_sdk::collections::UnorderedSet::new(b"tokens".to_vec()) 
                            };
                
                            // Actualizar los valores del NFT
                            let start = u128::from(U128(0));
                            let mut por_nft = tokens.iter()
                            .skip(start as usize) 
                            .filter_map(|token_id| {
                                let token = self.nft_token(token_id.clone()).unwrap();
                                let title = token.metadata.title.unwrap_or_default();
                                if title == "Proof Of Reputation NFT".to_string() {
                                    Some(self.nft_token(token_id.clone()).unwrap())
                                } else {
                                    None
                                }
                            })
                            .next().unwrap();
                
                            let mut metadata = self.token_metadata_by_id.get(&por_nft.token_id).unwrap();
                            let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
                            let mut extradatajson: PoR = serde_json::from_str(&newextradata).unwrap();
                
                            extradatajson.score = extradatajson.score+score;
                            extradatajson.quest_completed = extradatajson.quest_completed+1;
                            extradatajson.date_last_quest = env::block_timestamp();
                
                            let mut extra_string = serde_json::to_string(&extradatajson).unwrap();
                            extra_string = str::replace(&extra_string, "\"", "'");
                            metadata.extra = Some(extra_string.clone());
                
                            self.token_metadata_by_id.insert(&por_nft.token_id, &metadata);
                
                            log!("POR NFT Actualizado");
                        } else {
                            log!("No tiene el POR NFT");
                
                            // Minar el token nuevo
                            let mut new_token = TokenMetadata {
                                title:  Some("Proof Of Reputation NFT".to_string()), 
                                description:  Some("This nft contains the information with the progress within the network".to_string()),
                                media:  Some(IMAGE.to_string()),
                                expires_at: None,
                                starts_at: None,
                                copies: None,
                                extra: None,
                                issued_at: None,
                                media_hash: None,
                                reference: None,
                                reference_hash: None,
                                updated_at: None
                            };
                    
                            let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                    
                            let mut empty_poh = PoR {
                                score : 0,
                                quest_completed : 0,
                                date_last_quest : 0
                            };
                    
                            empty_poh.score = score;
                            empty_poh.quest_completed = 1;
                            empty_poh.date_last_quest = env::block_timestamp();
                    
                            let mut extra_data_string = serde_json::to_string(&empty_poh).unwrap();
                            extra_data_string = str::replace(&extra_data_string, "\"", "'");
                            new_token.extra = Some(extra_data_string);
                    
                            let mut royalty = HashMap::new();
                    
                            let token = Token {
                                owner_id: receiver_id.clone(),
                                approved_account_ids: Default::default(),
                                next_approval_id: 0,
                                royalty
                            };
                    
                            assert!(
                                self.tokens_by_id.insert(&token_id, &token).is_none(),
                                "Token already exists"
                            );
                    
                            self.token_metadata_by_id.insert(&token_id, &new_token);
                            self.internal_add_token_to_owner(&token.owner_id, &token_id);
                        }
                
                    let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
                    refund_deposit(required_storage_in_bytes);
                } else {
                    // No minar
                    log!("No estas registrado en Im Human");
                }
                
                return "Éxito al obtener Im Human".to_string();
            }
        }
    }

    #[payable]
    pub fn resolve_mint_st_near(&mut self, name: String, description: String, score: u64, image: String, has_por_nft: bool) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                return "Error al obtener el valor".to_string();
            },
            PromiseResult::Successful(result) => {
                let value = std::str::from_utf8(&result).unwrap();
                log!("stNEAR: {}",value);

                let receiver_id = env::signer_account_id();
                let initial_storage_usage = env::storage_usage();
                if value != "0" {
                    // Minar
                    log!("stNEAR: {}",value);
                    let mut new_token = TokenMetadata {
                        title:  Some(name.to_string()), 
                        description:  Some(description.to_string()),
                        media:  Some(image.to_string()),
                        expires_at: None,
                        starts_at: None,
                        copies: None,
                        extra: None,
                        issued_at: None,
                        media_hash: None,
                        reference: None,
                        reference_hash: None,
                        updated_at: None
                    };
                
                        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                
                        let mut empty_quest = Quest {
                            score : 0,
                            date: 0
                        };
                
                        empty_quest.score = score;
                        empty_quest.date = env::block_timestamp();
                
                        let mut extra_data_string = serde_json::to_string(&empty_quest).unwrap();
                        extra_data_string = str::replace(&extra_data_string, "\"", "'");
                        new_token.extra = Some(extra_data_string);
                
                        let mut royalty = HashMap::new();
                
                        let token = Token {
                            owner_id: receiver_id.clone(),
                            approved_account_ids: Default::default(),
                            next_approval_id: 0,
                            royalty
                        };
                
                        assert!(
                            self.tokens_by_id.insert(&token_id, &token).is_none(),
                            "Token already exists"
                        );
                
                        self.token_metadata_by_id.insert(&token_id, &new_token);
                        self.internal_add_token_to_owner(&token.owner_id, &token_id);
                
                        // Validar si ya tiene el POR NFT
                        if has_por_nft {
                            log!("Ya tiene el POR NFT");

                            let tokens_for_owner_set = self.tokens_per_owner.get(&receiver_id);
                            let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
                                tokens_for_owner_set
                            } else {
                                near_sdk::collections::UnorderedSet::new(b"tokens".to_vec()) 
                            };
                
                            // Actualizar los valores del NFT
                            let start = u128::from(U128(0));
                            let mut por_nft = tokens.iter()
                            .skip(start as usize) 
                            .filter_map(|token_id| {
                                let token = self.nft_token(token_id.clone()).unwrap();
                                let title = token.metadata.title.unwrap_or_default();
                                if title == "Proof Of Reputation NFT".to_string() {
                                    Some(self.nft_token(token_id.clone()).unwrap())
                                } else {
                                    None
                                }
                            })
                            .next().unwrap();
                
                            let mut metadata = self.token_metadata_by_id.get(&por_nft.token_id).unwrap();
                            let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
                            let mut extradatajson: PoR = serde_json::from_str(&newextradata).unwrap();
                
                            extradatajson.score = extradatajson.score+score;
                            extradatajson.quest_completed = extradatajson.quest_completed+1;
                            extradatajson.date_last_quest = env::block_timestamp();
                
                            let mut extra_string = serde_json::to_string(&extradatajson).unwrap();
                            extra_string = str::replace(&extra_string, "\"", "'");
                            metadata.extra = Some(extra_string.clone());
                
                            self.token_metadata_by_id.insert(&por_nft.token_id, &metadata);
                
                            log!("POR NFT Actualizado");
                        } else {
                            log!("No tiene el POR NFT");
                
                            // Minar el token nuevo
                            let mut new_token = TokenMetadata {
                                title:  Some("Proof Of Reputation NFT".to_string()), 
                                description:  Some("This nft contains the information with the progress within the network".to_string()),
                                media:  Some(IMAGE.to_string()),
                                expires_at: None,
                                starts_at: None,
                                copies: None,
                                extra: None,
                                issued_at: None,
                                media_hash: None,
                                reference: None,
                                reference_hash: None,
                                updated_at: None
                            };
                    
                            let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                    
                            let mut empty_poh = PoR {
                                score : 0,
                                quest_completed : 0,
                                date_last_quest : 0
                            };
                    
                            empty_poh.score = score;
                            empty_poh.quest_completed = 1;
                            empty_poh.date_last_quest = env::block_timestamp();
                    
                            let mut extra_data_string = serde_json::to_string(&empty_poh).unwrap();
                            extra_data_string = str::replace(&extra_data_string, "\"", "'");
                            new_token.extra = Some(extra_data_string);
                    
                            let mut royalty = HashMap::new();
                    
                            let token = Token {
                                owner_id: receiver_id.clone(),
                                approved_account_ids: Default::default(),
                                next_approval_id: 0,
                                royalty
                            };
                    
                            assert!(
                                self.tokens_by_id.insert(&token_id, &token).is_none(),
                                "Token already exists"
                            );
                    
                            self.token_metadata_by_id.insert(&token_id, &new_token);
                            self.internal_add_token_to_owner(&token.owner_id, &token_id);
                        }
                
                    let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
                    refund_deposit(required_storage_in_bytes);
                } else {
                    // No minar
                    log!("No tiene stNEAR");
                }
                
                return "Éxito al obtener stNEAR".to_string();
            }
        }
    }

    #[payable]
    pub fn resolve_mint_meta_token(&mut self, name: String, description: String, score: u64, image: String, has_por_nft: bool) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                return "Error al obtener el valor".to_string();
            },
            PromiseResult::Successful(result) => {
                let value = std::str::from_utf8(&result).unwrap();
                log!("Meta Token: {}",value);

                let receiver_id = env::signer_account_id();
                let initial_storage_usage = env::storage_usage();
                if value != "0" {
                    // Minar
                    log!("Meta Token: {}",value);
                    let mut new_token = TokenMetadata {
                        title:  Some(name.to_string()), 
                        description:  Some(description.to_string()),
                        media:  Some(image.to_string()),
                        expires_at: None,
                        starts_at: None,
                        copies: None,
                        extra: None,
                        issued_at: None,
                        media_hash: None,
                        reference: None,
                        reference_hash: None,
                        updated_at: None
                    };
                
                        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                
                        let mut empty_quest = Quest {
                            score : 0,
                            date: 0
                        };
                
                        empty_quest.score = score;
                        empty_quest.date = env::block_timestamp();
                
                        let mut extra_data_string = serde_json::to_string(&empty_quest).unwrap();
                        extra_data_string = str::replace(&extra_data_string, "\"", "'");
                        new_token.extra = Some(extra_data_string);
                
                        let mut royalty = HashMap::new();
                
                        let token = Token {
                            owner_id: receiver_id.clone(),
                            approved_account_ids: Default::default(),
                            next_approval_id: 0,
                            royalty
                        };
                
                        assert!(
                            self.tokens_by_id.insert(&token_id, &token).is_none(),
                            "Token already exists"
                        );
                
                        self.token_metadata_by_id.insert(&token_id, &new_token);
                        self.internal_add_token_to_owner(&token.owner_id, &token_id);
                
                        // Validar si ya tiene el POR NFT
                        if has_por_nft {
                            log!("Ya tiene el POR NFT");

                            let tokens_for_owner_set = self.tokens_per_owner.get(&receiver_id);
                            let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
                                tokens_for_owner_set
                            } else {
                                near_sdk::collections::UnorderedSet::new(b"tokens".to_vec()) 
                            };
                
                            // Actualizar los valores del NFT
                            let start = u128::from(U128(0));
                            let mut por_nft = tokens.iter()
                            .skip(start as usize) 
                            .filter_map(|token_id| {
                                let token = self.nft_token(token_id.clone()).unwrap();
                                let title = token.metadata.title.unwrap_or_default();
                                if title == "Proof Of Reputation NFT".to_string() {
                                    Some(self.nft_token(token_id.clone()).unwrap())
                                } else {
                                    None
                                }
                            })
                            .next().unwrap();

                            env::log(
                                json!(por_nft)
                                .to_string()
                                .as_bytes(),
                            );
                
                            let mut metadata = self.token_metadata_by_id.get(&por_nft.token_id).unwrap();
                            let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
                            let mut extradatajson: PoR = serde_json::from_str(&newextradata).unwrap();
                
                            extradatajson.score = extradatajson.score+score;
                            extradatajson.quest_completed = extradatajson.quest_completed+1;
                            extradatajson.date_last_quest = env::block_timestamp();
                
                            let mut extra_string = serde_json::to_string(&extradatajson).unwrap();
                            extra_string = str::replace(&extra_string, "\"", "'");
                            metadata.extra = Some(extra_string.clone());
                
                            self.token_metadata_by_id.insert(&por_nft.token_id, &metadata);
                
                            log!("POR NFT Actualizado");
                        } else {
                            log!("No tiene el POR NFT");
                
                            // Minar el token nuevo
                            let mut new_token = TokenMetadata {
                                title:  Some("Proof Of Reputation NFT".to_string()), 
                                description:  Some("This nft contains the information with the progress within the network".to_string()),
                                media:  Some(IMAGE.to_string()),
                                expires_at: None,
                                starts_at: None,
                                copies: None,
                                extra: None,
                                issued_at: None,
                                media_hash: None,
                                reference: None,
                                reference_hash: None,
                                updated_at: None
                            };
                    
                            let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                    
                            let mut empty_poh = PoR {
                                score : 0,
                                quest_completed : 0,
                                date_last_quest : 0
                            };
                    
                            empty_poh.score = score;
                            empty_poh.quest_completed = 1;
                            empty_poh.date_last_quest = env::block_timestamp();
                    
                            let mut extra_data_string = serde_json::to_string(&empty_poh).unwrap();
                            extra_data_string = str::replace(&extra_data_string, "\"", "'");
                            new_token.extra = Some(extra_data_string);
                    
                            let mut royalty = HashMap::new();
                    
                            let token = Token {
                                owner_id: receiver_id.clone(),
                                approved_account_ids: Default::default(),
                                next_approval_id: 0,
                                royalty
                            };
                    
                            assert!(
                                self.tokens_by_id.insert(&token_id, &token).is_none(),
                                "Token already exists"
                            );
                    
                            self.token_metadata_by_id.insert(&token_id, &new_token);
                            self.internal_add_token_to_owner(&token.owner_id, &token_id);
                        }
                
                    let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
                    refund_deposit(required_storage_in_bytes);
                } else {
                    // No minar
                    log!("No tienes Meta Token");
                }
                
                return "Éxito al obtener Meta Token".to_string();
            }
        }
    }

    #[payable]
    pub fn resolve_mint_voting_power(&mut self, name: String, description: String, score: u64, image: String, has_por_nft: bool) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                return "Error al obtener el valor".to_string();
            },
            PromiseResult::Successful(result) => {
                let value = std::str::from_utf8(&result).unwrap();
                log!("Voting Power: {}",value);

                let receiver_id = env::signer_account_id();
                let initial_storage_usage = env::storage_usage();
                if value != "0" {
                    // Minar
                    log!("Voting Power: {}",value);
                    let mut new_token = TokenMetadata {
                        title:  Some(name.to_string()), 
                        description:  Some(description.to_string()),
                        media:  Some(image.to_string()),
                        expires_at: None,
                        starts_at: None,
                        copies: None,
                        extra: None,
                        issued_at: None,
                        media_hash: None,
                        reference: None,
                        reference_hash: None,
                        updated_at: None
                    };
                
                        let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                
                        let mut empty_quest = Quest {
                            score : 0,
                            date: 0
                        };
                
                        empty_quest.score = score;
                        empty_quest.date = env::block_timestamp();
                
                        let mut extra_data_string = serde_json::to_string(&empty_quest).unwrap();
                        extra_data_string = str::replace(&extra_data_string, "\"", "'");
                        new_token.extra = Some(extra_data_string);
                
                        let mut royalty = HashMap::new();
                
                        let token = Token {
                            owner_id: receiver_id.clone(),
                            approved_account_ids: Default::default(),
                            next_approval_id: 0,
                            royalty
                        };
                
                        assert!(
                            self.tokens_by_id.insert(&token_id, &token).is_none(),
                            "Token already exists"
                        );
                
                        self.token_metadata_by_id.insert(&token_id, &new_token);
                        self.internal_add_token_to_owner(&token.owner_id, &token_id);
                
                        // Validar si ya tiene el POR NFT
                        if has_por_nft {
                            log!("Ya tiene el POR NFT");

                            let tokens_for_owner_set = self.tokens_per_owner.get(&receiver_id);
                            let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
                                tokens_for_owner_set
                            } else {
                                near_sdk::collections::UnorderedSet::new(b"tokens".to_vec()) 
                            };
                
                            // Actualizar los valores del NFT
                            let start = u128::from(U128(0));
                            let mut por_nft = tokens.iter()
                            .skip(start as usize) 
                            .filter_map(|token_id| {
                                let token = self.nft_token(token_id.clone()).unwrap();
                                let title = token.metadata.title.unwrap_or_default();
                                if title == "Proof Of Reputation NFT".to_string() {
                                    Some(self.nft_token(token_id.clone()).unwrap())
                                } else {
                                    None
                                }
                            })
                            .next().unwrap();
                
                            let mut metadata = self.token_metadata_by_id.get(&por_nft.token_id).unwrap();
                            let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
                            let mut extradatajson: PoR = serde_json::from_str(&newextradata).unwrap();
                
                            extradatajson.score = extradatajson.score+score;
                            extradatajson.quest_completed = extradatajson.quest_completed+1;
                            extradatajson.date_last_quest = env::block_timestamp();
                
                            let mut extra_string = serde_json::to_string(&extradatajson).unwrap();
                            extra_string = str::replace(&extra_string, "\"", "'");
                            metadata.extra = Some(extra_string.clone());
                
                            self.token_metadata_by_id.insert(&por_nft.token_id, &metadata);
                
                            log!("POR NFT Actualizado");
                        } else {
                            log!("No tiene el POR NFT");
                
                            // Minar el token nuevo
                            let mut new_token = TokenMetadata {
                                title:  Some("Proof Of Reputation NFT".to_string()), 
                                description:  Some("This nft contains the information with the progress within the network".to_string()),
                                media:  Some(IMAGE.to_string()),
                                expires_at: None,
                                starts_at: None,
                                copies: None,
                                extra: None,
                                issued_at: None,
                                media_hash: None,
                                reference: None,
                                reference_hash: None,
                                updated_at: None
                            };
                    
                            let token_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                    
                            let mut empty_poh = PoR {
                                score : 0,
                                quest_completed : 0,
                                date_last_quest : 0
                            };
                    
                            empty_poh.score = score;
                            empty_poh.quest_completed = 1;
                            empty_poh.date_last_quest = env::block_timestamp();
                    
                            let mut extra_data_string = serde_json::to_string(&empty_poh).unwrap();
                            extra_data_string = str::replace(&extra_data_string, "\"", "'");
                            new_token.extra = Some(extra_data_string);
                    
                            let mut royalty = HashMap::new();
                    
                            let token = Token {
                                owner_id: receiver_id.clone(),
                                approved_account_ids: Default::default(),
                                next_approval_id: 0,
                                royalty
                            };
                    
                            assert!(
                                self.tokens_by_id.insert(&token_id, &token).is_none(),
                                "Token already exists"
                            );
                    
                            self.token_metadata_by_id.insert(&token_id, &new_token);
                            self.internal_add_token_to_owner(&token.owner_id, &token_id);
                        }
                
                    let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
                    refund_deposit(required_storage_in_bytes);
                } else {
                    // No minar
                    log!("No tienes Voting Power: {}",value);
                }
                
                return "Éxito al obtener Voting Power".to_string();
            }
        }
    }
}