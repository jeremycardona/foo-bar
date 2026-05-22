# Implementation Plan: Gas Smash

**Branch**: 001-gas-smash-core | **Spec**: specs/gas-smash/spec.md

## Technical Context

- **Language**: Rust 1.75+
    
- **Backend**: Axum (Server), Alloy (Alchemy/Bitcoin RPC)
    
- **Frontend**: Yew (Wasm Framework), Stylist (CSS-in-Rust)
    
- **Tooling**: trunk (for Wasm bundling)
    

## Project Structure
├── Cargo.toml          # Workspace: backend, frontend, shared
├── shared/             # Common structs (CongestionData)
├── backend/            # Axum + Alloy (Bitcoin RPC logic)
└── frontend/           # Yew App (Game loop + Rendering)
## Constitution Check
- **Violation**: None. Using Workspace for shared types.
- **Violation**: None. Axum + Yew provides 100% Rust coverage.