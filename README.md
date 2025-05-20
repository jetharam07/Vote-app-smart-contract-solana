# Solana Vote App Smart Contract

This repository contains a simple voting smart contract built for the Solana blockchain using the Anchor framework. 
The contract was initially developed, deployed and tested on Solana Playground and later ported to a local Anchor environment in VS Code (`lib.rs`).

## 📁 Find lib.rs 

```
programs/
└── solana_project/
    ├── Cargo.toml           # Rust dependencies and metadata
    └── src/
        └── lib.rs           # Main smart contract code
```

## YouTube Tutorial
Watch it here: ▶️ https://youtu.be/FWyjmP4SbJA?si=ehcn8uNAdqm4S20M

## Features
- Register candidate
- Register voter
- Cast vote

- ## Setup Instructions
1. Clone this repository
2. Install Solana CLI and Anchor CLI
3. Run `anchor build` to compile the program
4. Deploy with `anchor deploy` to localnet or devnet
5. Use Anchor tests or your own frontend to interact with the program

## Tech Stack
- Rust (smart contract logic)
- Anchor framework (Solana smart contract development)
- Solana CLI and Anchor CLI for deployment and testing


## YouTube Tutorial
I have created a detailed video walkthrough explaining the development and deployment process of this Vote app smart contract in solana playground.  
Watch it here: ▶️ https://youtu.be/FWyjmP4SbJA?si=ehcn8uNAdqm4S20M

Solana devnet explorer: https://explorer.solana.com/address/Arp1b8XkZ2btv3NmvT5ANMxZ3dzo9dnGK2E28Fq2Mjuj?cluster=devnet
