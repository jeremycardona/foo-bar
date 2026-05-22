---

description: "Task list template for feature implementation"
---
# Tasks: Gas Smash MVP

## Phase 1: Setup (Shared Infrastructure)

T001 Initialize Cargo workspace with backend, frontend, and shared crates

T002 Define CongestionMetrics struct in shared/src/lib.rs

T003 Configure Trunk.toml for frontend asset management

## Phase 2: Foundational (Alchemy Integration)

T004 Implement Alchemy RPC client in backend using alloy for Bitcoin Testnet

T005 Create Axum endpoint GET /api/metrics that returns live Bitcoin stats

T006 Implement a background task in backend to poll Alchemy every 10 seconds

## Phase 3: User Story 1 & 2 - The Game (P1)

T007 [US1] Create Yew component to fetch and display CongestionMetrics

T008 [US2] Implement the 8-bit Game Loop in Yew (use gloo_timers for interval)

T009 [US2] Map CongestionMetrics.fee to mole_spawn_rate logic

T010 [US2] Add click-to-smash logic and score tracking

## Phase 4: Polish & Hackathon Demo (P3)

T011 Apply 8-bit CSS styles using Stylist or raw CSS (NES-style)

T012 Add "Block Hash" display to the game footer

T013 Final build validation with trunk build --release

---

## **Execution Strategy for Solo Builder (3 hrs left)**

1. **0:00 - 0:30**: T001-T003. Get the workspace building.
    
2. **0:30 - 1:15**: T004-T006. Get Bitcoin data flowing from Alchemy. If Bitcoin RPC is slow, mock it with an Env Var so you can keep moving.
    
3. **1:15 - 2:30**: T007-T010. The Yew Game Loop. This is where most time goes. Focus on "Clicking makes number go up."
    
4. **2:30 - 3:00**: T011-T013. CSS and Demo Prep. Deploy to Vercel/GitHub Pages if possible, or prep local demo.