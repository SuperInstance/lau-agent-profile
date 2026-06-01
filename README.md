# lau-agent-profile

> The captain's log for agent souls and armor — behavioral profiling for PLATO agents

Part of the **PLATO/LAU** mathematical agent framework.

---

## What This Does

Every agent has a **soul** (personality) and **armor** (capabilities). `lau-agent-profile` is a behavioral profiling system that tracks both, records mission outcomes, and answers the fleet-commander's question: *"Which agent should I send on this mission?"*

It models:

- **AgentSoul** — eight behavioral axes (risk tolerance, creativity, reliability, speed, conservation adherence, social tendency, learning rate, stress response) plus free-text notes, specializations, and known weaknesses
- **AgentArmor** — hard constraints (compute budget, sensor range, memory capacity, composition limit, known breaking points)
- **AgentProfile** — the union of soul + armor + mission history + genealogy, with running success-rate tracking and soul-metric adaptation after each mission
- **CrewRoster** — a fleet-level collection with mission-fit scoring, team composition, at-risk detection, and specialization coverage

Agents learn from experience: every mission outcome adjusts reliability, conservation adherence, and learning rate via exponential moving averages. The profiling system then uses these adapted metrics to score how well an agent fits future mission requirements.

---

## Key Idea

> Agents are not interchangeable. An agent that *thrives* under stress and has high conservation adherence is a terrible fit for a high-risk creative task — but the perfect fit for a stability-critical monitoring mission.

The `fits_mission()` scoring function weighs six dimensions — reliability, speed, risk compatibility, conservation adherence, specialization overlap, and stress compatibility — into a single 0–1 score. The `CrewRoster` uses this to find the best agent or compose a balanced team.

---

## Install

```toml
[dependencies]
lau-agent-profile = { git = "https://github.com/SuperInstance/lau-agent-profile" }
```

Or, if published to crates.io:

```bash
cargo add lau-agent-profile
```

### Requirements

- Rust **2024 edition** (requires Rust 1.85+)
- `serde` with `derive` feature

---

## Quick Start

```rust
use lau_agent_profile::*;

// Create a profile
let mut profile = AgentProfile::new(AgentId("scout-7".into()));
profile.soul.risk_tolerance = 0.3;
profile.soul.creativity = 0.8;
profile.soul.stress_response = StressResponse::Adapts;
profile.soul.specializations = vec!["scouting".into(), "navigation".into()];

// Record a mission
profile.record_mission(MissionRecord {
    mission_id: "op-neptune".into(),
    agent_id: AgentId("scout-7".into()),
    outcome: MissionOutcome::Success,
    tick: 42,
    duration: 100,
    conservation_error: 0.02,
    notes: "Clean navigation, minor detour".into(),
});

// Check reliability
println!("Reliable? {}", profile.is_reliable()); // true (high reliability + 100% success)

// Build a crew roster
let mut roster = CrewRoster::new();
roster.add(profile);

// Find the best agent for a mission
let req = MissionRequirements {
    min_reliability: 0.7,
    min_speed: 0.5,
    required_specializations: vec!["scouting".into()],
    max_risk: 0.5,
    min_conservation: 0.5,
    stress_level: 0.7,
};
let best = roster.best_for_mission(&req);
println!("Best fit: {:?}", best);

// View the profile
println!("{}", profile.summary());
```

---

## API Reference

### Core Types

| Type | Description |
|------|-------------|
| `AgentId(String)` | Newtype identifier, `Hash + Eq + Display` |
| `AgentSoul` | 8 behavioral axes (0–1 f64), stress response enum, specializations, weaknesses, notes |
| `AgentArmor` | Hard limits: compute ticks, sensor range/channels, actuator channels, memory, conservation margin, composition limit, breaking points |
| `AgentProfile` | `id + soul + armor + mission_history + genealogy`, with mutation methods |
| `CrewRoster` | `HashMap<AgentId, AgentProfile>`, fleet-level queries |
| `MissionRecord` | A single mission: ID, outcome, tick, duration, conservation error, notes |
| `MissionRequirements` | Min thresholds for reliability, speed, conservation; max risk; required specializations; stress level |

### Enums

| Enum | Values |
|------|--------|
| `Trend` | `Improving`, `Stable`, `Declining` |
| `StressResponse` | `Thrives`, `Degrades`, `Freezes`, `Adapts`, `Panics` |
| `MissionOutcome` | `Success`, `PartialSuccess`, `Failed`, `Abandoned` |
| `BehavioralMetric` | Named metric with value, trend, sample count, last-updated tick |
| `BreakingPoint` | Condition string + threshold + consequence |

### Key Methods

#### `AgentProfile`

- `new(id: AgentId) → Self` — default soul (all 0.5, `Adapts` stress), default armor
- `record_mission(record: MissionRecord)` — updates success rate, reliability, conservation adherence, learning rate via EMA
- `update_soul(metric: &str, value: f64)` — set a named axis (clamped to 0–1)
- `add_note(note: String)` — append behavioral observation
- `summary() → String` — formatted multi-line profile report
- `is_reliable() → bool` — `reliability > 0.7 && success_rate > 0.8`
- `fits_mission(req: &MissionRequirements) → f64` — 0–1 compatibility score

#### `CrewRoster`

- `new() → Self` / `default() → Self`
- `add(profile: AgentProfile)` — insert into roster
- `get(id: &AgentId) → Option<&AgentProfile>`
- `best_for_mission(req: &MissionRequirements) → Option<AgentId>` — max fit score
- `team_for_mission(req: &MissionRequirements, size: usize) → Vec<AgentId>` — greedy selection
- `at_risk_agents() → Vec<&AgentProfile>` — success rate < 0.5
- `specialization_coverage() → HashMap<String, usize>` — count per specialization
- `roster_summary() → String` — formatted fleet overview

### Serialization

All types derive `Serialize + Deserialize` via serde. Round-trip tested.

---

## How It Works

### Mission Fit Scoring

The `fits_mission()` function computes a weighted average over up to six dimensions:

```
score = Σ(dimension_scores) / Σ(weights)
```

Each dimension contributes 1.0 weight. The specialization dimension is only included when requirements specify non-empty `required_specializations`. The stress dimension is only included when `stress_level > 0.5`.

| Dimension | Score if met | Score if not met |
|-----------|-------------|------------------|
| Reliability ≥ min | 1.0 | `reliability / min_reliability` |
| Speed ≥ min | 1.0 | `speed / min_speed` |
| Risk ≤ max | 1.0 | `(1 - risk) / (1 - max_risk)` |
| Conservation ≥ min | 1.0 | `adherence / min_conservation` |
| Specializations | — | `matches / required` |
| Stress fit | — | Lookup table (Thrives=1.0, Adapts=0.8, Degrades=0.4, Freezes=0.2, Panics=0.1) |

### Soul Adaptation

After each mission, the soul self-adjusts:

- **Success rate** — running average: `new_rate = old_rate × (n-1)/n + outcome/n`
- **Conservation adherence** — EMA with α=0.1: `new = old × 0.9 + (1 - error) × 0.1`
- **Reliability** — ±0.05 per mission outcome (success/failure)
- **Learning rate** — EMA with α=0.05: `new = old × 0.95 + error × 0.05`

All values are clamped to [0, 1].

### Team Selection

`team_for_mission()` uses greedy selection: sort all agents by `fits_mission()` score descending, take the top N. This prefers distinct agents ranked by fit rather than optimizing for complementary coverage.

---

## The Math

### Behavioral Space

Each agent occupies a point in **R⁷** (seven continuous axes):

```
B(agent) = (risk, creativity, reliability, speed, conservation, social, learning) ∈ [0,1]⁷
```

The discrete stress response and categorical specializations extend this into a mixed continuous-categorical space.

### Mission Fit as Inner Product

The fit score can be interpreted as a normalized inner product between the agent's behavioral vector and the mission's requirement vector:

```
fit(agent, mission) = (1/w) Σᵢ sᵢ(agent, mission)
```

where `sᵢ` is the per-dimension scoring function (binary or ratio) and `w` is the number of active dimensions.

### Success Rate as Bayesian Update

The running-average update:

```
pₙ = pₙ₋₁ · (n-1)/n + xₙ/n
```

is equivalent to a maximum-likelihood estimate of Bernoulli parameter p after n observations, with uniform prior.

### Exponential Moving Average

Conservation adherence and learning rate use EMA:

```
vₙ = (1 - α) · vₙ₋₁ + α · xₙ
```

with α = 0.1 for conservation and α = 0.05 for learning. This gives an effective window of roughly 1/α observations, emphasizing recent performance while retaining history.

---

## Test Suite

**44 tests** covering:

- AgentId hashing, equality, display
- Trend, StressResponse, MissionOutcome display formatting
- AgentSoul defaults
- BreakingPoint and AgentArmor construction
- AgentProfile creation, mission recording, success rate updates
- Soul metric adaptation (conservation, reliability, learning rate)
- Value clamping, unknown metric no-op
- Summary formatting
- `is_reliable()` edge cases (no missions, low reliability, low success rate)
- `fits_mission()` perfect match, poor match, no specializations, extreme values
- CrewRoster add/get, best-for-mission, team selection, at-risk detection
- Specialization coverage
- Serde round-trip (profile, roster, all enums)
- Genealogical tracking
- Smooth metric drift over many missions

Run: `cargo test`

---

## License

MIT
