# Solana Automated Market Maker (AMM)

A decentralized exchange implementation on Solana using the Anchor framework. This AMM uses the constant product formula (x \* y = k) to facilitate automated token trading without traditional order books.

## Features

- **Automated Token Swaps**: Trade between any two SPL tokens in the pool
- **Liquidity Provision**: Earn fees by providing liquidity to trading pairs
- **Constant Product Formula**: Ensures efficient price discovery and slippage protection
- **Fee System**: Configurable trading fees that reward liquidity providers
- **Security Features**: Optional authority control and emergency lock mechanism

## Architecture

### Core Components

1. **Pool Management**

   - Initialize new trading pools
   - Configure trading fees
   - Optional authority control
   - Pool locking mechanism

2. **Liquidity Operations**

   - Deposit tokens to provide liquidity
   - Withdraw liquidity with accrued fees
   - LP token minting/burning

3. **Trading Functions**
   - Token swaps with slippage protection
   - Automatic price calculation
   - Fee collection and distribution

### Smart Contract Structure

```rust
// Main operations
initialize(seed: u64, fee: u16, authority: Option<Pubkey>)
deposit(lp_amount: u64, max_x: u64, max_y: u64)
withdraw(lp_amount: u64, min_x: u64, min_y: u64)
swap(SwapArgs)

// Configuration state
struct Config {
    seed: u64,                    // Pool identifier
    authority: Option<Pubkey>,    // Admin control
    mint_x: Pubkey,              // Token X mint
    mint_y: Pubkey,              // Token Y mint
    fee: u16,                    // Trading fee
    locked: bool,                // Safety lock
    config_bump: u8,             // PDA bumps
    lp_bump: u8,
}
```

## Getting Started

### Prerequisites

- Solana Tool Suite
- Anchor Framework
- Node.js and Yarn
- Rust and Cargo

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd anchor-amm
   ```

2. Install dependencies:

   ```bash
   yarn install
   ```

3. Build the program:
   ```bash
   anchor build
   ```

### Testing

Run the test suite:

```bash
anchor test
```

## Usage Guide

### Creating a New Pool

1. Initialize a new trading pool:
   ```typescript
   await program.methods
     .initialize(seed, fee, authority)
     .accounts({...})
     .rpc();
   ```

### Providing Liquidity

1. Approve token transfers
2. Call deposit function:
   ```typescript
   await program.methods
     .deposit(lpAmount, maxX, maxY)
     .accounts({...})
     .rpc();
   ```

### Trading Tokens

1. Prepare swap parameters
2. Execute swap:
   ```typescript
   await program.methods
     .swap(swapArgs)
     .accounts({...})
     .rpc();
   ```

### Withdrawing Liquidity

1. Calculate withdrawal amounts
2. Execute withdrawal:
   ```typescript
   await program.methods
     .withdraw(lpAmount, minX, minY)
     .accounts({...})
     .rpc();
   ```

## Security Considerations

- **Slippage Protection**: All operations include maximum/minimum amount parameters
- **Authority Control**: Optional admin authority for controlled deployments
- **Emergency Lock**: Pool can be locked to prevent operations in emergencies
- **PDA Security**: Uses Program Derived Addresses for secure account management

## Mathematical Model

The AMM uses the constant product formula as its core pricing mechanism. This mathematical model ensures automated price discovery and liquidity provision.

### Constant Product Formula

The fundamental equation is:

```
x * y = k
```

Where:

- x: Reserve of token X
- y: Reserve of token Y
- k: Constant product value

### Price Calculation

The spot price (P) for trading dx amount of token X for dy amount of token Y is:

```
P = dy/dx = x/y
```

### Slippage and Price Impact

1. **Effective Price Calculation**
   For a trade of size Δx:

   ```
   Δy = y * Δx / (x + Δx)
   Effective Price = Δy/Δx
   ```

2. **Price Impact**
   The price impact increases with trade size:
   ```
   Price Impact = |1 - (y * Δx) / ((x + Δx) * Δy)|
   ```

### Liquidity Provider Returns

1. **LP Token Calculation**
   When providing liquidity:

   ```
   LP tokens = sqrt(x * y)
   ```

2. **Fee Earnings**
   For each trade with fee rate f:
   ```
   Fee Amount = Δx * f
   LP Share = (Your LP tokens / Total LP tokens) * Fee Amount
   ```

### Trading Bounds

1. **Minimum Output**
   For a given slippage tolerance s:

   ```
   Minimum Output = Expected Output * (1 - s)
   ```

2. **Maximum Input**
   ```
   Maximum Input = Input Amount * (1 + s)
   ```

### Pool Depth and Price Sensitivity

1. **Price Sensitivity Coefficient**

   ```
   Sensitivity = Δ% Price / Δ% Quantity = x * y / (x + y)²
   ```

2. **Liquidity Depth Impact**
   - Larger pools (higher k) = Lower price impact
   - Smaller pools (lower k) = Higher price impact

### Arbitrage Bounds

The AMM maintains price equilibrium through arbitrage opportunities:

```
External Price < AMM Price: Buy from external, sell to AMM
External Price > AMM Price: Buy from AMM, sell to external
```

These mathematical principles ensure:

- Predictable pricing
- Automated market making
- Fair value for liquidity providers
- Efficient price discovery through arbitrage
- Protection against manipulation through slippage bounds

![alt text](https://miro.medium.com/v2/resize%3Afit%3A1400/format%3Awebp/0%2AVGonDJYq5p4-dqrv.png)

## AMM-related attacks

### Flash loan Oracle attacks

![Flash loan Oracle attacks](https://miro.medium.com/v2/resize%3Afit%3A720/format%3Awebp/1%2AAqwxaL77Pj0TQ7YtLXKrhg.png)

### Rug pull

![alt text](https://miro.medium.com/v2/resize%3Afit%3A720/format%3Awebp/1%2Ao6wjq9H9FaNyJ6aOvhQISg.png)

## Sandwich attacks
Combing front- and back-running, a “sandwich attacker” places his orders immediately before and after the victim’s trade transaction. The attacker uses front-running to cause victim losses, and then uses back-running to pocket benefits.

One example is a sandwich price attack, in which for example a large buy order is seen by a malicious actor. That actor buys before the initial transaction can go through, thus moving the price up. The original transaction is now executed. Afterwards, the malicious actor back-runs the transaction and sells the previously bought tokens for a higher price, pocketing the price difference.

## Acknowledgments

- Solana Foundation
- Anchor Framework
