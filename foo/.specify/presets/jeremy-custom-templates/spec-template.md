# Feature Specification: Gas Smash MVP

**Feature Branch**: 001-gas-smash-core  
**Created**: 2023-10-27  
**Status**: Final (Rapid Hackathon Mode)  
**Input**: User description: "8-bit Rust/Wasm Bitcoin congestion mole-smasher."

## User Scenarios & Testing

### User Story 1 - The Pulse (Priority: P1)

As a user, I want to see the real-time Bitcoin Testnet congestion reflected in the game's "Heat" level so I can visualize network activity.

**Why this priority**: Essential for the "Real-time" claim. Without live data, it's just a local game.  
**Independent Test**: Verify the Axum backend logs Bitcoin block heights/mempool stats from Alchemy.  
**Acceptance Scenarios**:

1. **Given** the app is running, **When** a new block or transaction spike occurs on Bitcoin Testnet, **Then** the "Network Heat" value increases.
    

---

### User Story 2 - The Smash (Priority: P1)

As a player, I want to click on "Mole" targets that appear at a speed dictated by the Network Heat.

**Why this priority**: Core gameplay loop.  
**Independent Test**: Game runs in browser via Wasm; moles appear; clicking increments a counter.  
**Acceptance Scenarios**:

1. **Given** high gas, **When** the game starts, **Then** moles move 2x faster than at low gas.
    

---

### User Story 3 - The Proof (Priority: P2)

As a hacker, I want to see my score displayed alongside the specific Block Hash that influenced the game.

**Why this priority**: Connects the game world to the blockchain world for the demo.  
**Acceptance Scenarios**:

1. **Given** a game over, **Then** show "Smashed during Block #[Height]".
    

## Requirements

### Functional Requirements

- **FR-001**: System MUST fetch Bitcoin Testnet stats using Alchemy's RPC via alloy.
    
- **FR-002**: Backend MUST expose a JSON endpoint /api/congestion returning current metrics.
    
- **FR-003**: Frontend MUST be compiled to Wasm using Yew.
    
- **FR-004**: System MUST share a GameState struct between Backend and Frontend.
    

### Key Entities

- **CongestionData**: Represents mempool count and fee estimates.
    
- **Mole**: A game object with coordinates and "alive" state.
    

## Success Criteria

- **SC-001**: Zero JavaScript used in the final build.
    
- **SC-002**: Successful data fetch from Alchemy Bitcoin Testnet RPC.
    
- **SC-003**: Playable 8-bit loop in browser at 60fps.