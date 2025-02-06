# Jito Vault Auto Delegate

## Overview and Purpose

The Jito Vault Auto Delegate bot is a permissionless utility that must be run at the end of every epoch to call a series of instructions to delegate some of the stake to different node operators.
## Getting Started

### Options

- RPC URL: The RPC endpoint URL
- Keypair: The path to your keypair file used to pay for transactions
- Vault Program Id: The program ID of Jito Vault Program
- Restaking Program Id: The program ID of Jito Restaking Program
- Crank Interval: Time in seconds between cranking attempts (default: 300)
- Metrics Interval: Time in seconds between metrics emission (default: 300)
- Priority Fees: Priority fees in microlamports per compute unit (default: 10000)

### Run locally

To run the cranker with a specific vault and restaking program:

```bash
cargo run -p jito-vault-auto-dump -- \
  --keypair-path <KEYPAIR_PATH> \
  --rpc-url <RPC_URL> \
  --vault-program-id "Vau1t6sLNxnzB7ZDsef8TLbPLfyZMYXH8WTNqUdm9g8" \
  --kysol-vault-ata "CQpvXgoaaawDCLh8FwMZEwQqnPakRUZ5BnzhjnEBPJv" \
  --kyjto-vault-ata "ABsoYTwRPBJEf55G7N8hVw7tQnDKBA6GkZCKBVrjTTcf" \
  --crank-interval 300 \
  --priority-fees 10000

```