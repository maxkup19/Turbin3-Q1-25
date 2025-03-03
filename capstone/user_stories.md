# SolanaBets: MVP Prediction Market

## Core Value Proposition

A minimal viable prediction market platform on Solana enabling binary (yes/no) market creation and trading. Focus on core smart contract functionality with a basic frontend interface.

## MVP Scope

- Binary prediction markets only (yes/no outcomes)
- Single oracle resolution (admin controlled for MVP)
- Basic market creation and trading
- Simple order matching (no AMM in MVP)
- Minimal frontend for interaction

## Target User for MVP

### The Basic Trader

**Demographics:**

- Solana developers and early adopters
- Technical users comfortable with blockchain interactions

**Core Needs:**

- Create and trade in simple prediction markets
- View basic market information
- Execute trades with SOL

## MVP User Stories

### User Story ID: MVP-001

**Priority:** High
**Goal:** Create Binary Market

**User Story:** As a market creator, I want to create a simple yes/no prediction market.

**Functionality:**

- Create market with:
  - Question text
  - Expiration timestamp
  - Initial liquidity amount

**Technical Scope:**

- Anchor program instruction for market creation
- PDA for market state
- Basic validation checks

### User Story ID: MVP-002

**Priority:** High
**Goal:** Place Trade

**User Story:** As a trader, I want to place a yes/no position in a market.

**Functionality:**

- Place order with:
  - Position side (yes/no)
  - Amount in SOL
  - Market ID

**Technical Scope:**

- Trade instruction in Anchor
- Order state handling
- Basic position tracking

### User Story ID: MVP-003

**Priority:** High
**Goal:** Resolve Market

**User Story:** As an admin, I want to resolve a market with the final outcome.

**Functionality:**

- Submit final outcome (yes/no)
- Trigger settlement calculations

**Technical Scope:**

- Resolution instruction
- Settlement logic
- Winner payment distribution

## Technical Implementation Plan

### Anchor Program (3 days)

**Core Instructions:**

```rust
pub fn create_market(
    ctx: Context<CreateMarket>,
    question: String,
    expires_at: i64,
) -> Result<()>

pub fn place_trade(
    ctx: Context<PlaceTrade>,
    side: Side,  // Yes/No
    amount: u64,
) -> Result<()>

pub fn resolve_market(
    ctx: Context<ResolveMarket>,
    outcome: bool,
) -> Result<()>
```

**Key Accounts:**

```rust
#[account]
pub struct Market {
    pub question: String,
    pub expires_at: i64,
    pub yes_amount: u64,
    pub no_amount: u64,
    pub resolved: bool,
    pub outcome: Option<bool>,
}

#[account]
pub struct Position {
    pub owner: Pubkey,
    pub market: Pubkey,
    pub side: Side,
    pub amount: u64,
}
```

### Basic Frontend (2 days)

- Wallet connection
- Market list view
- Simple trade form
- Position display

### Testing & Deployment (2 days)

- Unit tests for instructions
- Integration tests for key flows
- Devnet deployment
- Frontend deployment

## Technical Requirements

**Dependencies:**

- Anchor framework
- Wallet adapter
- Basic frontend

**Security Considerations:**

- Input validation
- Access control
- Basic error handling

**Performance Requirements:**

- Sub-second trade confirmation
- Support for multiple concurrent users

## Out of Scope for MVP

- AMM/Automated pricing
- Multiple resolution sources
- Market categories/tags
- Advanced trading features
- Mobile optimization
- Governance features

This MVP focuses on core smart contract functionality to validate the basic prediction market mechanics. Future iterations will add more sophisticated features and improved UX.
