Hi Gemini.
"We want to build Gas Smash: a real-time blockchain congestion simulator disguised as a chaotic 8-bit arcade game, engineered with a 100% unified Rust stack., zero-JavaScript"
# (The Problem & Solution)

Most people look at block explorers, financial tickers, or gas charts and see cold, boring data. They don’t actually feel the stress of a network bottleneck, an NFT mint spike, or a sudden gas war until their own transaction fails.
We wanted to change that. Gas Smash translates algorithmic ledger metrics into raw physical adrenaline.
By pulling real-time network data directly from our Alchemy nodes, our backend scales the difficulty of a classic Whack-a-Mole game on the fly. When global transaction volume spikes or gas fees skyrocket, the moles speed up, change size, and pop up with absolute ferocity. You are quite literally playing against the real-time heartbeat of the blockchain.
# (How We Beat the Bloat)
What makes this project truly elite isn't just the concept—it's the architecture. While 99% of dApps on the market are bogged down by massive, heavy JavaScript framework chains, we are building this with zero lines of JS.

Our 1-man team engineering a fully compiled, type-safe pipeline across a single Rust workspace:

The Engine (Backend): Uses Alloy and Axum to stream raw blockchain metrics from Alchemy and serve them with sub-millisecond efficiency.

The Render (Frontend): Uses Yew to compile our game loops directly into native WebAssembly (Wasm) running inside the browser.

The Bridge: Because both tiers are written in Rust, they share the exact same data types. The compiler guarantees type-safety from our private RPC nodes all the way to the individual pixels rendering on the user's monitor.

---
# Context
I am a solo builder for a hackaton, I have 3 hours in real-time left and I have to demo a simple dapp using bitcoin testnet in Alchemy developer platform. I want to simplify most of the heavy load to get done by simply doing specification driven development.
# Identity
You are the expert at writing specifications in a technical language for engineers, developers, scientists, hackers and designers. 
# Task
You are a building out the speckit markdown files to develop a 2-hour real time decentralized application in the alchemy ecosystem.
# Output
Fillout the speckits ./specifiy template files to have a specification before implementing all the code.
