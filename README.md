# Turbin3 - Solana Development Projects

This repository contains various Solana blockchain projects developed during the **Turbin3 Q1 2025 Builders Cohort**. Each project demonstrates different aspects of Solana development using the Anchor framework.

## Projects Overview

### Core Projects

- [Anchor Vault](./anchor-vault) - A simple vault program for depositing and withdrawing SOL
- [Anchor AMM](./anchor-amm) - An automated market maker for token swaps and liquidity provision
- [Dice Game](./dice) - A gambling program with bet placement and resolution
- [Anchor Marketplace](./anchor-marketplace) - NFT marketplace implementation
- [Anchor Escrow](./anchor-escrow) - Escrow service for secure token exchanges
- [Airdrop](./airdrop) - Token airdrop distribution system
- [Capstone Project](./capstone) - Final project with prediction markets

### Learning Projects

- [Solana Starter](./solana-starter) - Starter project with TypeScript and Rust components
- [Prereq2](./prereq2) - Prerequisite learning materials and exercises

## Detailed Project Descriptions

### Anchor Vault

A simple vault program that allows users to:

- Initialize a vault
- Deposit SOL into the vault
- Withdraw SOL from the vault
- Close the vault and reclaim rent

**Key Features:**

- PDA-based vault accounts
- Secure deposit/withdrawal operations
- Complete test suite

### Anchor AMM

An automated market maker (AMM) implementation that enables:

- Liquidity pool creation
- Token deposits and withdrawals
- Token swaps with fee mechanism

**Key Features:**

- Constant product curve implementation
- Fee collection system
- Slippage protection

### Dice Game

A gambling program that allows users to:

- Place bets with specified roll values
- Refund bets if needed
- Resolve bets with cryptographic verification

**Key Features:**

- Secure random number generation
- Ed25519 signature verification
- Bet state management

### Anchor Marketplace

An NFT marketplace implementation for:

- Listing NFTs for sale
- Purchasing NFTs
- Managing listings and sales

### Anchor Escrow

A secure escrow service for:

- Creating escrow agreements
- Executing trades between parties
- Cancelling escrow agreements

### Airdrop

A token distribution system for:

- Managing token airdrops
- Verifying recipient eligibility
- Tracking distribution status

### Capstone Project: SolanaBets Prediction Market

SolanaBets is a decentralized prediction market platform built on Solana that enables users to create, trade, and settle prediction markets for various events. The platform leverages Solana's high speed and low transaction costs to provide a seamless betting experience.

**Project Overview:**

- **Name:** SolanaBets - Decentralized Prediction Market Platform
- **Focus:** Binary outcome markets (yes/no predictions)
- **Target Audience:** Crypto-native users, sports betting enthusiasts, DeFi users

**Key Features:**

- Permissionless market creation for any verifiable future event
- Non-custodial design where users maintain control of their funds
- Low 5% fee only on profits (compared to 10-15% on traditional platforms)
- Fast settlement and transparent market resolution
- Integration with Solana DeFi ecosystem

**Technical Architecture:**

- **Smart Contract Components:**

  - Market creation and initialization
  - Order matching engine
  - Settlement and dispute resolution
  - Treasury management
  - Oracle integration for result verification

- **Account Structure:**

  - Market PDA: Stores market state
  - Position PDA: Tracks user positions
  - User State PDA: Manages user-specific data

- **Core Instructions:**
  ```rust
  pub fn init_market(ctx: Context<InitMarket>, question: String, expires_at: i64, question_hash: Vec<u8>) -> Result<()>
  pub fn place_bet(ctx: Context<PlaceBet>, bet: Bet, amount: u64) -> Result<()>
  pub fn resolve_market(ctx: Context<ResolveMarket>, winner: Bet) -> Result<()>
  ```

**Business Model:**

- 5% fee on winning profits (not on principal)
- Small market creation fee to prevent spam (refundable for active markets)
- Future premium features for professional market makers

**Development Timeline:**

- Initial smart contract development
- Core market creation and order matching logic
- Settlement and dispute resolution mechanisms
- Testing and security audits
- UI/UX refinements
- Testnet deployment and community testing
- Mainnet launch preparation

**Competitive Advantages:**

- Lower fees than centralized alternatives
- Faster and cheaper than Ethereum-based prediction markets
- User custody of funds
- Permissionless market creation
- Seamless integration with Solana ecosystem

## Getting Started

### Prerequisites

- Rust and Cargo
- Solana CLI tools
- Node.js and npm/yarn
- Anchor framework

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/turbin3.git
cd turbin3
```

2. Navigate to a specific project:

```bash
cd anchor-vault
```

3. Install dependencies:

```bash
yarn install
```

4. Build the program:

```bash
anchor build
```

5. Run tests:

```bash
anchor test
```

## About Turbin3 Q1 2025 Builders Cohort

The Turbin3 Q1 2025 Builders Cohort is an intensive program for developers looking to build on the Solana blockchain. The cohort provides hands-on experience with Solana development, focusing on real-world applications and best practices.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Solana Foundation
- Anchor Framework team
- Turbin3 mentors and instructors
