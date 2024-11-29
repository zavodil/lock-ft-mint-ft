The contract implements a unique token wrapping mechanism and expects tokens to be sent from the token_id_to_lock contract account directly to the contract's own address. When tokens are received, the contract automatically mints an equivalent amount of its own fungible tokens to the sender's account.

Key Characteristics
----

1. One-to-one token mapping
2. Automatic token minting upon receipt
3. Restricted to a specific source token account
4. Ensures precise token value preservation

Workflow Example
----

1. User wants to lock tokens 
2. User sends X tokens from token_id_to_lock contract to this contract
3. Contract automatically mints X non-transferable tokens of its own token
4. Minted tokens are credited to the original sender's account

This design enables a simple, controlled token wrapping or bridging process with minimal complexity and maximum security.

HOW TO INIT CONTRACT:
===
```commandline
export METADATA='{"spec": "ft-1.0.0","name": "Token", "symbol": "TKN", "icon": "data:image/png;base64,...", "decimals": 18}'

near call $CONTRACT_ID new '{"token_id": '"$FT_ID"', "total_supply": "1000000000000000000000000000","metadata":'"$METADATA"'}' --accountId $CONTRACT_ID

```

Register this contracts in token_id_to_lock contract

```
near call $FT_ID storage_deposit '{"account_id": "'"$CONTRACT_ID"'"}' --accountId $ACCOUNT_ID --deposit 0.0125
```

Start locking tokens
```
near call $FT_ID ft_transfer_call '{"receiver_id": "'"$CONTRACT_ID"'", "amount": "123", "msg": ""}' --accountId $ACCOUNT_ID --depositYocto 1 --gas 50000000000000
```

BUILD DOCKER ON M1:
===
Prepare docker
```
 clone https://github.com/near/near-sdk-rs/pull/720/files
 ./build_docker_m1.sh
```

Run docker buildx `contract-builder`
``` 
 ./build_docker_m1.sh
```

