# DAO-Treasury
A minimal Soroban smart contract that stores, deposits, and withdraws a shared treasury balance on the Stellar network.


# DAO Treasury (Soroban Smart Contract)

## 📌 Project Description

This project is a basic DAO (Decentralized Autonomous Organization) Treasury smart contract built on the Stellar Soroban platform. It provides a minimal structure for managing shared funds in a decentralized environment.

The contract allows a designated admin to control withdrawals while allowing anyone to deposit funds into the treasury.

---

## ⚙️ What It Does

* Initializes a DAO treasury with an admin account
* Allows users to deposit funds into the treasury
* Allows only the admin to withdraw funds
* Maintains and tracks the total treasury balance

---

##  Deploy smart contract 
contract address : CDFK36LG7MUFELYXPIC2WN6ID22ZROD76KA3Q5OVXBAR2DWQNM6CEPPJ
<img width="1819" height="778" alt="image" src="https://github.com/user-attachments/assets/0dee27a3-f389-4f28-ac53-dc64a8596f47" />


## 🚀 Features

* **Admin Control**: Only the admin can withdraw funds
* **Open Deposits**: Anyone can contribute to the treasury
* **Simple Balance Tracking**: Keeps record of total funds
* **Lightweight Design**: Minimal and easy to extend
* **Secure Access**: Uses Soroban's built-in authentication

---

## 🛠️ Tech Stack

* Rust
* Soroban SDK
* Stellar Blockchain

---

## 📦 Future Improvements

* Token integration (XLM / custom tokens)
* DAO voting system for withdrawals
* Multi-signature approval
* Proposal creation & governance

---

## ▶️ How to Run

1. Install Soroban CLI:

   ```
   cargo install soroban-cli
   ```

2. Build contract:

   ```
   cargo build --target wasm32-unknown-unknown --release
   ```

3. Deploy:

   ```
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/dao_treasury.wasm
   ```

---

## 📜 License

MIT License
