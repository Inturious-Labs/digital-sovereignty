# README

## dfx commands

Switch to the correct identity:

```
dfx identity list
dfx identity use guizishanren
dfx identity whoami
```

Get the principal `lxmxz-3fjoo-5fcay-obm3a-3jk4a-r4ztf-sgvy5-w2pkk-ovwor-hf42s-7ae` for identity `guizishanren`:

```
dfx identity get-principal
```

Check the ICP balance and the account ID for the identity canister `guizishanren`:

```
dfx ledger --network ic balance
```

Check the account ID `0e5dc971adc229513ae59e5a8c83864dbf4d296d32306360ffd6bde154dab793` (for ICP transfer) for the identity canister `guizishanren`:

```
dfx ledger --network ic account-id
```

Check the cycles balance for the identity canister `guizishanren`:

```
dfx cycles --network ic balance
```

Transfer some $ICP into account `0e5dc...b793` and verify the balance has been topped up with `dfx ledger`. 

Then, convert $ICP into cycles from the ledger account to cycles account for this identity `guizishanren`:

```
dfx cycles convert --network ic --amount 1
```

Verify that the ICP balance has been deducted with `dfx ledger` and that cycles balance has been topped up with `dfx cycles`. 

