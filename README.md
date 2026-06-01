# lau-agent-profile

> Part of the PLATO/LAU mathematical agent framework

## What This Does

Part of the PLATO/LAU mathematical agent framework. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-agent-profile
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_agent_profile::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub struct AgentId(pub String);
pub enum Trend 
pub struct BehavioralMetric 
pub enum StressResponse 
pub struct AgentSoul 
pub struct BreakingPoint 
pub struct AgentArmor 
pub enum MissionOutcome 
pub struct MissionRecord 
pub struct MissionRequirements 
pub struct AgentProfile 
    pub fn new(id: AgentId) -> Self 
    pub fn record_mission(&mut self, record: MissionRecord) 
    pub fn update_soul(&mut self, metric: &str, value: f64) 
    pub fn add_note(&mut self, note: String) 
    pub fn summary(&self) -> String 
    pub fn is_reliable(&self) -> bool 
    pub fn fits_mission(&self, requirements: &MissionRequirements) -> f64 
pub struct CrewRoster 
    pub fn new() -> Self 
    pub fn add(&mut self, profile: AgentProfile) 
    pub fn get(&self, id: &AgentId) -> Option<&AgentProfile> 
    pub fn best_for_mission(&self, requirements: &MissionRequirements) -> Option<AgentId> 
    pub fn team_for_mission(
    pub fn roster_summary(&self) -> String 
    pub fn at_risk_agents(&self) -> Vec<&AgentProfile> 
    pub fn specialization_coverage(&self) -> HashMap<String, usize> 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**44 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
