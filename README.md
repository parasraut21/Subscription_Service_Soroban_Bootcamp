# Subscription Service Smart Contract

## Introduction
- Welcome to the Subscription Service Smart Contract repository! This project utilizes Soroban and Rust on the Stellar blockchain to create a robust subscription management system.

## Features
- Secure Subscriptions: Utilize Stellar blockchain's security features for subscription management.
- Transparent Transactions: Ensure transparent and immutable transaction records.
- Flexible Configuration: Easily customize subscription plans and parameters.
- Efficient Payment Processing: Enable fast and reliable payment processing.

## Getting Started
- Follow these instructions to get the project up and running on your local machine for development and testing purposes.

## Prerequisites
Before you begin, make sure you have the following installed:
- Soroban
- Rust

## Installation

1. Clone the repo
   ```sh
   git clone https://github.com/parasraut21/Subscription_Service_Soroban_Bootcamp
   ```
2. Build the Contract : To build the Soroban contract, use the following command:
   ```sh
   soroban contract build
   ```
3. Deploy the Contract : 
- This should return "CBOZIA5JNPALLLVCDLGG2SQ234RH7RSYOEXQ6XP5LASIEKDSZFZT43N2".
- Replace the contract id (CBOZIA5JNPALLLVCDLGG2SQ234RH7RSYOEXQ6XP5LASIEKDSZFZT43N2) with your actual contract id.
   ```js
   soroban contract deploy `
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm `
  --source alice `
  --network testnet
   ```
4. Check your address :
- This should return "GCPJW3EEK2CLCIQOBDY35VQWJDBG2EJDRXXYFCJILCVSS7OMS6AV2JRN".
- add the address in the subscribe command
   ```js
    soroban keys address alice
   ```
5. Subscribe : - Invoke the contract to subscribe using the following command:
- This should return "Subscribed".
   ```js
      soroban contract invoke `
        --id CBOZIA5JNPALLLVCDLGG2SQ234RH7RSYOEXQ6XP5LASIEKDSZFZT43N2 `
        --source alice `
        --network testnet `
        -- `
        subscribe `
        --from GCPJW3EEK2CLCIQOBDY35VQWJDBG2EJDRXXYFCJILCVSS7OMS6AV2JRN `
        --amount 365
   ```
6. Unsubscribe
- This should return "UnSubscribed".
   ```js
      soroban contract invoke --id CBOZIA5JNPALLLVCDLGG2SQ234RH7RSYOEXQ6XP5LASIEKDSZFZT43N2 --source alice --network testnet -- unsubscribe --from GCPJW3EEK2CLCIQOBDY35VQWJDBG2EJDRXXYFCJILCVSS7OMS6AV2JRN
   ```


