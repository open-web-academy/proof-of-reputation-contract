CONTRATO INTELIGENTE PARA MINAR NFT’s en NEAR

Versión de node: 12.22.5

Versión de NEAR CLI: 3.4.2

El siguiente contrato inteligente es un ejemplo del minado de NFT´s en NEAR Protocol, siga los siguientes pasos para compilar y desplegar el contrato.

Compilar y desplegar contrato Contrato:

    ./build.sh

Una vez compilado y desplegado el contrato deberá cambiar el CONTRACT y USER_ACCOUNT por las correspondientes.

export NEAR_ENV=mainnet
export NEAR_ENV=testnet

CONTRACT=proof-of-reputation.near
USER_ACCOUNT=yairnava.near

Contract Init:

    near call $CONTRACT new_default_meta '{"owner_id": "'$CONTRACT'"}' --accountId $CONTRACT

Show Metadata

    near view $CONTRACT nft_metadata

Mint Quest

    near call $CONTRACT nft_mint_quest '{"quest_number":0, "receiver_id":"yairnava.near"}' --accountId $USER_ACCOUNT --amount 0.1  --gas=300000000000000

    near call $CONTRACT nft_mint_quest '{"quest_number":1, "receiver_id":"yairnava.near"}' --accountId $USER_ACCOUNT --amount 0.1  --gas=300000000000000

    near call $CONTRACT nft_mint_quest '{"quest_number":2, "receiver_id":"yairnava.near"}' --accountId $USER_ACCOUNT --amount 0.1  --gas=300000000000000

    near call $CONTRACT nft_mint_quest '{"quest_number":3, "receiver_id":"yairnava.near"}' --accountId $USER_ACCOUNT --amount 0.1  --gas=300000000000000

Get number of minted tokens

    near view $CONTRACT nft_total_supply

Get NFT by Id

    near view $CONTRACT nft_token '{"token_id": "0"}'

Get NFTs of account

    near view $CONTRACT nft_tokens_for_owner '{"account_id": "syi216.testnet", "from_index": "0", "limit": 50}' 

Review Quests

I Am Human:
    near view registry.i-am-human.near is_human '{"account": "owa-is-bos.near"}'

Get stNEAR:
    near view meta-pool.near ft_balance_of '{"account_id": "yairnava.near"}'

Get META:
    near view meta-token.near ft_balance_of '{"account_id": "yairnava.near"}'

Get Voting Power:
    near view meta-vote.near get_locked_balance '{"voter_id": "yairnava.near"}'
