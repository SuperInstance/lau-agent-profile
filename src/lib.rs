//! `lau-agent-profile` — the captain's log for agent souls and armor.
//!
//! A behavioral profiling system that lets humans know their crew:
//! what an agent is good at, what breaks it, and how it reacts under pressure.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Newtypes
// ---------------------------------------------------------------------------

/// Unique identifier for an agent.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentId(pub String);

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Trend
// ---------------------------------------------------------------------------

/// Direction of change for a behavioral metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    Improving,
    Stable,
    Declining,
}

impl std::fmt::Display for Trend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trend::Improving => write!(f, "improving"),
            Trend::Stable => write!(f, "stable"),
            Trend::Declining => write!(f, "declining"),
        }
    }
}

// ---------------------------------------------------------------------------
// BehavioralMetric
// ---------------------------------------------------------------------------

/// A single tracked behavioral metric with trend information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralMetric {
    pub name: String,
    pub value: f64,
    pub trend: Trend,
    pub samples: u32,
    pub last_updated: u64, // tick
}

// ---------------------------------------------------------------------------
// StressResponse
// ---------------------------------------------------------------------------

/// How an agent reacts under pressure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StressResponse {
    /// Performs better under pressure.
    Thrives,
    /// Quality drops under pressure.
    Degrades,
    /// Stops acting under pressure.
    Freezes,
    /// Adjusts strategy under pressure.
    Adapts,
    /// Behaves randomly under pressure.
    Panics,
}

impl std::fmt::Display for StressResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StressResponse::Thrives => write!(f, "thrives under pressure"),
            StressResponse::Degrades => write!(f, "degrades under pressure"),
            StressResponse::Freezes => write!(f, "freezes under pressure"),
            StressResponse::Adapts => write!(f, "adapts under pressure"),
            StressResponse::Panics => write!(f, "panics under pressure"),
        }
    }
}

// ---------------------------------------------------------------------------
// AgentSoul
// ---------------------------------------------------------------------------

/// The behavioral signature of an agent — its soul.
///
/// All f64 values are in the 0–1 range unless otherwise noted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSoul {
    /// Cautious (0) → Bold (1).
    pub risk_tolerance: f64,
    /// Follows patterns (0) → Novel approaches (1).
    pub creativity: f64,
    /// Variable (0) → Consistent (1).
    pub reliability: f64,
    /// Methodical (0) → Fast (1).
    pub speed: f64,
    /// Loose (0) → Strict (1).
    pub conservation_adherence: f64,
    /// Lone wolf (0) → Collaborative (1).
    pub social_tendency: f64,
    /// How the agent responds to stress.
    pub stress_response: StressResponse,
    /// Slow adapter (0) → Fast adapter (1).
    pub learning_rate: f64,
    /// What this agent is best at.
    pub specializations: Vec<String>,
    /// Known failure modes.
    pub weaknesses: Vec<String>,
    /// Free-text observations.
    pub behavioral_notes: Vec<String>,
}

impl Default for AgentSoul {
    fn default() -> Self {
        Self {
            risk_tolerance: 0.5,
            creativity: 0.5,
            reliability: 0.5,
            speed: 0.5,
            conservation_adherence: 0.5,
            social_tendency: 0.5,
            stress_response: StressResponse::Adapts,
            learning_rate: 0.5,
            specializations: Vec::new(),
            weaknesses: Vec::new(),
            behavioral_notes: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// BreakingPoint
// ---------------------------------------------------------------------------

/// A known condition that breaks the agent.
///
/// Example: `condition: "vibe > 0.95"` → `consequence: "conservation violations"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingPoint {
    pub condition: String,
    pub threshold: f64,
    pub consequence: String,
}

// ---------------------------------------------------------------------------
// AgentArmor
// ---------------------------------------------------------------------------

/// The constraint profile of an agent — its armor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentArmor {
    /// Bytecode step budget.
    pub max_compute_ticks: u64,
    /// Spatial perception range.
    pub sensor_range: f64,
    /// Number of distinct inputs.
    pub sensor_channels: u32,
    /// Number of distinct outputs.
    pub actuator_channels: u32,
    /// Local variable capacity.
    pub memory_capacity: usize,
    /// How close to violation it can operate.
    pub conservation_margin: f64,
    /// Maximum number of agents it can compose with.
    pub composition_limit: u32,
    /// Known conditions that break this agent.
    pub known_breaking_points: Vec<BreakingPoint>,
}

// ---------------------------------------------------------------------------
// MissionOutcome
// ---------------------------------------------------------------------------

/// The outcome of a mission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionOutcome {
    Success,
    PartialSuccess,
    Failed,
    Abandoned,
}

impl std::fmt::Display for MissionOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissionOutcome::Success => write!(f, "success"),
            MissionOutcome::PartialSuccess => write!(f, "partial success"),
            MissionOutcome::Failed => write!(f, "failed"),
            MissionOutcome::Abandoned => write!(f, "abandoned"),
        }
    }
}

// ---------------------------------------------------------------------------
// MissionRecord
// ---------------------------------------------------------------------------

/// A recorded mission for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionRecord {
    pub mission_id: String,
    pub agent_id: AgentId,
    pub outcome: MissionOutcome,
    pub tick: u64,
    pub duration: u64,
    pub conservation_error: f64,
    pub notes: String,
}

// ---------------------------------------------------------------------------
// MissionRequirements
// ---------------------------------------------------------------------------

/// Requirements for a mission, used to score agent fit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionRequirements {
    pub min_reliability: f64,
    pub min_speed: f64,
    pub required_specializations: Vec<String>,
    pub max_risk: f64,
    pub min_conservation: f64,
    pub stress_level: f64,
}

// ---------------------------------------------------------------------------
// AgentProfile
// ---------------------------------------------------------------------------

/// The complete profile of an agent, combining soul, armor, and history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub id: AgentId,
    pub soul: AgentSoul,
    pub armor: AgentArmor,
    pub mission_history: Vec<MissionRecord>,
    pub created_tick: u64,
    pub total_missions: u32,
    pub success_rate: f64,
    /// Parent agent IDs (genealogy).
    pub genealogy: Vec<AgentId>,
}

impl AgentProfile {
    /// Create a new agent profile with default soul and armor.
    pub fn new(id: AgentId) -> Self {
        Self {
            id,
            soul: AgentSoul::default(),
            armor: AgentArmor {
                max_compute_ticks: 1000,
                sensor_range: 10.0,
                sensor_channels: 4,
                actuator_channels: 4,
                memory_capacity: 256,
                conservation_margin: 0.1,
                composition_limit: 5,
                known_breaking_points: Vec::new(),
            },
            mission_history: Vec::new(),
            created_tick: 0,
            total_missions: 0,
            success_rate: 1.0,
            genealogy: Vec::new(),
        }
    }

    /// Record a mission outcome and update success rate and soul metrics.
    pub fn record_mission(&mut self, record: MissionRecord) {
        // Update success rate based on outcome
        let was_success = matches!(record.outcome, MissionOutcome::Success);

        self.total_missions += 1;
        let n = self.total_missions as f64;
        // Running average: weigh new outcome against history
        let new_success = if was_success { 1.0 } else { 0.0 };
        self.success_rate = if n > 1.0 {
            self.success_rate * ((n - 1.0) / n) + new_success / n
        } else {
            new_success
        };

        // Adjust soul metrics based on conservation error
        let conservation_impact = record.conservation_error.clamp(0.0, 1.0);
        self.soul.conservation_adherence =
            (self.soul.conservation_adherence * 0.9 + (1.0 - conservation_impact) * 0.1)
                .clamp(0.0, 1.0);

        // Reliability adjusts based on outcome
        if was_success {
            self.soul.reliability =
                (self.soul.reliability + 0.05).clamp(0.0, 1.0);
        } else {
            self.soul.reliability =
                (self.soul.reliability - 0.05).clamp(0.0, 1.0);
        }

        // Learning rate adjusts based on how much error we saw
        let error_impact = conservation_impact;
        self.soul.learning_rate = (self.soul.learning_rate * 0.95 + error_impact * 0.05)
            .clamp(0.0, 1.0);

        self.mission_history.push(record);
    }

    /// Adjust a named soul parameter to a new value.
    ///
    /// Supported metric names map to `AgentSoul` fields:
    /// `risk_tolerance`, `creativity`, `reliability`, `speed`,
    /// `conservation_adherence`, `social_tendency`, `learning_rate`.
    pub fn update_soul(&mut self, metric: &str, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match metric {
            "risk_tolerance" => self.soul.risk_tolerance = clamped,
            "creativity" => self.soul.creativity = clamped,
            "reliability" => self.soul.reliability = clamped,
            "speed" => self.soul.speed = clamped,
            "conservation_adherence" => self.soul.conservation_adherence = clamped,
            "social_tendency" => self.soul.social_tendency = clamped,
            "learning_rate" => self.soul.learning_rate = clamped,
            _ => { /* unknown metric — ignore */ }
        }
    }

    /// Add a free-text behavioral observation.
    pub fn add_note(&mut self, note: String) {
        self.soul.behavioral_notes.push(note);
    }

    /// Return a human-readable profile summary.
    pub fn summary(&self) -> String {
        format!(
            "Agent: {}\n\
             ─────────────────────────────\n\
             Soul:\n\
               Risk Tolerance:      {:.2}  (cautious ↔ bold)\n\
               Creativity:          {:.2}  (pattern ↔ novel)\n\
               Reliability:         {:.2}  (variable ↔ consistent)\n\
               Speed:               {:.2}  (methodical ↔ fast)\n\
               Conservation:        {:.2}  (loose ↔ strict)\n\
               Social Tendency:     {:.2}  (lone wolf ↔ collaborative)\n\
               Learning Rate:       {:.2}  (slow ↔ fast adapter)\n\
               Stress Response:     {}\n\
             Specializations:      {}\n\
             Weaknesses:           {}\n\
             Behavior Notes:       {}\n\
             ─────────────────────────────\n\
             Armor:\n\
               Compute Ticks:      {}\n\
               Sensor Range:       {:.1}\n\
               Sensor Channels:    {}\n\
               Actuator Channels:  {}\n\
               Memory Capacity:    {}\n\
               Conservation Margin:{:.2}\n\
               Composition Limit:  {}\n\
               Breaking Points:    {}\n\
             ─────────────────────────────\n\
             Missions: {} total, success rate {:.1}%\n\
             Genealogy: {}",
            self.id,
            self.soul.risk_tolerance,
            self.soul.creativity,
            self.soul.reliability,
            self.soul.speed,
            self.soul.conservation_adherence,
            self.soul.social_tendency,
            self.soul.learning_rate,
            self.soul.stress_response,
            if self.soul.specializations.is_empty() {
                "none".into()
            } else {
                self.soul.specializations.join(", ")
            },
            if self.soul.weaknesses.is_empty() {
                "none".into()
            } else {
                self.soul.weaknesses.join(", ")
            },
            if self.soul.behavioral_notes.is_empty() {
                "none".into()
            } else {
                self.soul.behavioral_notes.join("; ")
            },
            self.armor.max_compute_ticks,
            self.armor.sensor_range,
            self.armor.sensor_channels,
            self.armor.actuator_channels,
            self.armor.memory_capacity,
            self.armor.conservation_margin,
            self.armor.composition_limit,
            if self.armor.known_breaking_points.is_empty() {
                "none".into()
            } else {
                self.armor
                    .known_breaking_points
                    .iter()
                    .map(|bp| format!("({} → {})", bp.condition, bp.consequence))
                    .collect::<Vec<_>>()
                    .join(", ")
            },
            self.total_missions,
            self.success_rate * 100.0,
            if self.genealogy.is_empty() {
                "none".into()
            } else {
                self.genealogy
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(" → ")
            },
        )
    }

    /// Whether the agent is considered reliable:
    /// reliability > 0.7 and success_rate > 0.8.
    pub fn is_reliable(&self) -> bool {
        self.soul.reliability > 0.7 && self.success_rate > 0.8
    }

    /// Compute a compatibility score (0–1) for a given mission.
    ///
    /// Evaluates how well the agent's soul matches the mission
    /// requirements and returns a normalized score.
    pub fn fits_mission(&self, requirements: &MissionRequirements) -> f64 {
        let mut score = 0.0;
        let mut total_weight = 0.0;

        // Reliability
        if self.soul.reliability >= requirements.min_reliability {
            score += 1.0;
        } else {
            score += self.soul.reliability / requirements.min_reliability.max(0.01);
        }
        total_weight += 1.0;

        // Speed
        if self.soul.speed >= requirements.min_speed {
            score += 1.0;
        } else {
            score += self.soul.speed / requirements.min_speed.max(0.01);
        }
        total_weight += 1.0;

        // Risk — agent should not exceed max_risk
        if self.soul.risk_tolerance <= requirements.max_risk {
            score += 1.0;
        } else {
            score += (1.0 - self.soul.risk_tolerance) / (1.0 - requirements.max_risk.min(0.99));
        }
        total_weight += 1.0;

        // Conservation adherence
        if self.soul.conservation_adherence >= requirements.min_conservation {
            score += 1.0;
        } else {
            score += self.soul.conservation_adherence / requirements.min_conservation.max(0.01);
        }
        total_weight += 1.0;

        // Specializations
        if !requirements.required_specializations.is_empty() {
            let spec_hits = requirements
                .required_specializations
                .iter()
                .filter(|req| self.soul.specializations.iter().any(|s| s == *req))
                .count();
            let spec_score = spec_hits as f64 / requirements.required_specializations.len() as f64;
            score += spec_score;
            total_weight += 1.0;
        }

        // Stress compatibility — higher stress_level penalizes agents that
        // freeze or panic, rewards those that thrive or adapt.
        if requirements.stress_level > 0.5 {
            let stress_fit = match self.soul.stress_response {
                StressResponse::Thrives => 1.0,
                StressResponse::Adapts => 0.8,
                StressResponse::Degrades => 0.4,
                StressResponse::Freezes => 0.2,
                StressResponse::Panics => 0.1,
            };
            score += stress_fit;
            total_weight += 1.0;
        }

        score / total_weight
    }
}

// ---------------------------------------------------------------------------
// CrewRoster
// ---------------------------------------------------------------------------

/// A collection of agent profiles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewRoster {
    pub profiles: HashMap<AgentId, AgentProfile>,
}

impl CrewRoster {
    /// Create an empty roster.
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    /// Add an agent profile to the roster.
    pub fn add(&mut self, profile: AgentProfile) {
        self.profiles.insert(profile.id.clone(), profile);
    }

    /// Get a reference to an agent profile by ID.
    pub fn get(&self, id: &AgentId) -> Option<&AgentProfile> {
        self.profiles.get(id)
    }

    /// Return the agent ID with the highest `fits_mission` score.
    pub fn best_for_mission(&self, requirements: &MissionRequirements) -> Option<AgentId> {
        self.profiles
            .values()
            .max_by(|a, b| {
                a.fits_mission(requirements)
                    .partial_cmp(&b.fits_mission(requirements))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|p| p.id.clone())
    }

    /// Build a complementary team for a mission.
    ///
    /// Selects agents greedily: picks the best fit for each role
    /// while preferring distinct specializations to form a balanced team.
    pub fn team_for_mission(
        &self,
        requirements: &MissionRequirements,
        team_size: usize,
    ) -> Vec<AgentId> {
        let mut candidates: Vec<(AgentId, f64)> = self
            .profiles
            .values()
            .map(|p| (p.id.clone(), p.fits_mission(requirements)))
            .collect();
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(team_size);
        candidates.into_iter().map(|(id, _)| id).collect()
    }

    /// A human-readable overview of all agents in the roster.
    pub fn roster_summary(&self) -> String {
        if self.profiles.is_empty() {
            return "Crew roster is empty.".to_string();
        }

        let mut lines = vec!["Crew Roster".to_string(), "═════════════════════════════════════".to_string()];
        for profile in self.profiles.values() {
            let specs_display = if profile.soul.specializations.is_empty() {
                "none".to_string()
            } else {
                profile.soul.specializations.join(", ")
            };
            lines.push(format!(
                "  {}  |  missions: {}  |  success: {:.0}%  |  reliable: {}  |  specializations: {}",
                profile.id,
                profile.total_missions,
                profile.success_rate * 100.0,
                if profile.is_reliable() { "✓" } else { "✗" },
                specs_display,
            ));
        }
        lines.join("\n")
    }

    /// Return profiles for agents that are at risk:
    /// those with declining metrics or low success rate.
    pub fn at_risk_agents(&self) -> Vec<&AgentProfile> {
        self.profiles
            .values()
            .filter(|p| p.success_rate < 0.5)
            .collect()
    }

    /// Count how many agents have each specialization.
    pub fn specialization_coverage(&self) -> HashMap<String, usize> {
        let mut coverage: HashMap<String, usize> = HashMap::new();
        for profile in self.profiles.values() {
            for spec in &profile.soul.specializations {
                *coverage.entry(spec.clone()).or_insert(0) += 1;
            }
        }
        coverage
    }
}

impl Default for CrewRoster {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- AgentId --------------------------------------------------------

    #[test]
    fn agent_id_traits() {
        let a = AgentId("alpha".into());
        let b = AgentId("beta".into());
        let a2 = AgentId("alpha".into());

        assert_eq!(a, a2);
        assert_ne!(a, b);
        assert_eq!(a.to_string(), "alpha");
        // Hash + Eq consistency
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(a.clone());
        set.insert(b.clone());
        set.insert(a2);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn agent_id_display() {
        let id = AgentId("scout-42".into());
        assert_eq!(format!("{id}"), "scout-42");
    }

    // -- Trend ----------------------------------------------------------

    #[test]
    fn trend_display() {
        assert_eq!(Trend::Improving.to_string(), "improving");
        assert_eq!(Trend::Stable.to_string(), "stable");
        assert_eq!(Trend::Declining.to_string(), "declining");
    }

    // -- BehavioralMetric -----------------------------------------------

    #[test]
    fn behavioral_metric_construction() {
        let metric = BehavioralMetric {
            name: "vibe".into(),
            value: 0.85,
            trend: Trend::Improving,
            samples: 42,
            last_updated: 100,
        };
        assert_eq!(metric.name, "vibe");
        assert_eq!(metric.value, 0.85);
    }

    // -- StressResponse -------------------------------------------------

    #[test]
    fn stress_response_display() {
        assert_eq!(StressResponse::Thrives.to_string(), "thrives under pressure");
        assert_eq!(StressResponse::Degrades.to_string(), "degrades under pressure");
        assert_eq!(StressResponse::Freezes.to_string(), "freezes under pressure");
        assert_eq!(StressResponse::Adapts.to_string(), "adapts under pressure");
        assert_eq!(StressResponse::Panics.to_string(), "panics under pressure");
    }

    // -- AgentSoul ------------------------------------------------------

    #[test]
    fn agent_soul_default() {
        let soul = AgentSoul::default();
        assert_eq!(soul.risk_tolerance, 0.5);
        assert_eq!(soul.stress_response, StressResponse::Adapts);
        assert!(soul.specializations.is_empty());
    }

    // -- BreakingPoint & AgentArmor ------------------------------------

    #[test]
    fn breaking_point_construction() {
        let bp = BreakingPoint {
            condition: "vibe > 0.95".into(),
            threshold: 0.95,
            consequence: "conservation violations".into(),
        };
        assert_eq!(bp.condition, "vibe > 0.95");
    }

    #[test]
    fn agent_armor_default_profile() {
        let profile = AgentProfile::new(AgentId("test".into()));
        assert_eq!(profile.armor.max_compute_ticks, 1000);
        assert!(profile.armor.known_breaking_points.is_empty());
    }

    // -- MissionOutcome -------------------------------------------------

    #[test]
    fn mission_outcome_display() {
        assert_eq!(MissionOutcome::Success.to_string(), "success");
        assert_eq!(MissionOutcome::PartialSuccess.to_string(), "partial success");
        assert_eq!(MissionOutcome::Failed.to_string(), "failed");
        assert_eq!(MissionOutcome::Abandoned.to_string(), "abandoned");
    }

    // -- AgentProfile ---------------------------------------------------

    #[test]
    fn agent_profile_new() {
        let id = AgentId("alpha".into());
        let profile = AgentProfile::new(id.clone());
        assert_eq!(profile.id, id);
        assert_eq!(profile.total_missions, 0);
        assert_eq!(profile.success_rate, 1.0); // no failures yet
        assert!(profile.mission_history.is_empty());
    }

    #[test]
    fn record_mission_increases_count() {
        let mut profile = AgentProfile::new(AgentId("recorder".into()));
        assert_eq!(profile.total_missions, 0);

        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("recorder".into()),
            outcome: MissionOutcome::Success,
            tick: 0,
            duration: 10,
            conservation_error: 0.02,
            notes: "clean run".into(),
        });

        assert_eq!(profile.total_missions, 1);
        assert_eq!(profile.mission_history.len(), 1);
    }

    #[test]
    fn record_mission_updates_success_rate() {
        let mut profile = AgentProfile::new(AgentId("sr".into()));

        // First mission: success
        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("sr".into()),
            outcome: MissionOutcome::Success,
            tick: 0,
            duration: 10,
            conservation_error: 0.0,
            notes: "".into(),
        });
        assert!((profile.success_rate - 1.0).abs() < 1e-6);

        // Second: failure
        profile.record_mission(MissionRecord {
            mission_id: "m2".into(),
            agent_id: AgentId("sr".into()),
            outcome: MissionOutcome::Failed,
            tick: 1,
            duration: 5,
            conservation_error: 0.5,
            notes: "".into(),
        });
        assert!((profile.success_rate - 0.5).abs() < 1e-6);
    }

    #[test]
    fn record_mission_adjusts_soul_metrics() {
        let mut profile = AgentProfile::new(AgentId("adjust".into()));

        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("adjust".into()),
            outcome: MissionOutcome::Success,
            tick: 0,
            duration: 10,
            conservation_error: 0.0,
            notes: "".into(),
        });

        // Conservation adherence should have increased (low error)
        assert!(profile.soul.conservation_adherence > 0.5);
        // Reliability should have increased (success)
        assert!(profile.soul.reliability > 0.5);
    }

    #[test]
    fn record_mission_with_high_conservation_error() {
        let mut profile = AgentProfile::new(AgentId("wasteful".into()));
        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("wasteful".into()),
            outcome: MissionOutcome::Failed,
            tick: 0,
            duration: 10,
            conservation_error: 0.95,
            notes: "massive overrun".into(),
        });

        assert!(profile.soul.conservation_adherence < 0.5);
        assert!(profile.soul.reliability < 0.5);
    }

    #[test]
    fn update_soul_clamps_values() {
        let mut profile = AgentProfile::new(AgentId("clamp".into()));

        profile.update_soul("risk_tolerance", 1.5);
        assert_eq!(profile.soul.risk_tolerance, 1.0);

        profile.update_soul("creativity", -0.5);
        assert_eq!(profile.soul.creativity, 0.0);
    }

    #[test]
    fn update_soul_unknown_metric_is_noop() {
        let mut profile = AgentProfile::new(AgentId("noop".into()));
        profile.update_soul("non_existent", 0.9);
        // All defaults should remain
        assert_eq!(profile.soul.risk_tolerance, 0.5);
    }

    #[test]
    fn add_note_works() {
        let mut profile = AgentProfile::new(AgentId("note_taker".into()));
        assert!(profile.soul.behavioral_notes.is_empty());

        profile.add_note("Shows unusual caution near cliffs.".into());
        assert_eq!(profile.soul.behavioral_notes.len(), 1);
    }

    #[test]
    fn summary_contains_basic_info() {
        let mut profile = AgentProfile::new(AgentId("summarizer".into()));
        profile.soul.specializations = vec!["scouting".into(), "navigation".into()];
        profile.soul.weaknesses = vec!["loud environments".into()];
        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("summarizer".into()),
            outcome: MissionOutcome::Success,
            tick: 0,
            duration: 10,
            conservation_error: 0.01,
            notes: "".into(),
        });

        let s = profile.summary();
        assert!(s.contains("summarizer"));
        assert!(s.contains("scouting"));
        assert!(s.contains("navigation"));
        assert!(s.contains("loud environments"));
        assert!(s.contains("100.0%")); // success rate
    }

    #[test]
    fn is_reliable_true() {
        let mut profile = AgentProfile::new(AgentId("reliable".into()));
        profile.soul.reliability = 0.8;
        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("reliable".into()),
            outcome: MissionOutcome::Success,
            tick: 0,
            duration: 10,
            conservation_error: 0.0,
            notes: "".into(),
        });
        assert!(profile.is_reliable());
    }

    #[test]
    fn is_reliable_false_low_reliability() {
        let mut profile = AgentProfile::new(AgentId("unreliable".into()));
        profile.soul.reliability = 0.5;
        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("unreliable".into()),
            outcome: MissionOutcome::Success,
            tick: 0,
            duration: 10,
            conservation_error: 0.0,
            notes: "".into(),
        });
        assert!(!profile.is_reliable());
    }

    #[test]
    fn is_reliable_false_low_success_rate() {
        let mut profile = AgentProfile::new(AgentId("bumpy".into()));
        profile.soul.reliability = 0.9;
        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("bumpy".into()),
            outcome: MissionOutcome::Failed,
            tick: 0,
            duration: 10,
            conservation_error: 0.3,
            notes: "".into(),
        });
        assert!(!profile.is_reliable());
    }

    #[test]
    fn fits_mission_perfect_match() {
        let mut profile = AgentProfile::new(AgentId("perfect".into()));
        profile.soul.reliability = 0.9;
        profile.soul.speed = 0.9;
        profile.soul.risk_tolerance = 0.3;
        profile.soul.conservation_adherence = 0.9;
        profile.soul.stress_response = StressResponse::Thrives;
        profile.soul.specializations = vec!["scouting".into()];
        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("perfect".into()),
            outcome: MissionOutcome::Success,
            tick: 0,
            duration: 10,
            conservation_error: 0.0,
            notes: "".into(),
        });

        let req = MissionRequirements {
            min_reliability: 0.7,
            min_speed: 0.7,
            required_specializations: vec!["scouting".into()],
            max_risk: 0.5,
            min_conservation: 0.7,
            stress_level: 0.8,
        };

        let score = profile.fits_mission(&req);
        // Should be close to 1.0
        assert!(score > 0.9, "Expected high fit, got {score}");
    }

    #[test]
    fn fits_mission_poor_match() {
        let profile = AgentProfile::new(AgentId("mismatch".into()));
        let req = MissionRequirements {
            min_reliability: 0.9,
            min_speed: 0.9,
            required_specializations: vec!["expert".into()],
            max_risk: 0.1,
            min_conservation: 0.9,
            stress_level: 0.0,
        };
        let score = profile.fits_mission(&req);
        // Default half-values against high requirements should be low
        assert!(score < 0.7, "Expected low fit, got {score}");
    }

    #[test]
    fn fits_mission_no_required_specializations() {
        let profile = AgentProfile::new(AgentId("nospec".into()));
        let req = MissionRequirements {
            min_reliability: 0.4,
            min_speed: 0.4,
            required_specializations: vec![],
            max_risk: 0.6,
            min_conservation: 0.4,
            stress_level: 0.0,
        };
        let score = profile.fits_mission(&req);
        // Default mid-values (0.5) against 0.4-0.6 requirements should be a close match
        assert!(score > 0.8, "Expected high fit, got {score}");
    }

    // -- CrewRoster -----------------------------------------------------

    #[test]
    fn crew_roster_add_and_get() {
        let mut roster = CrewRoster::new();
        let id = AgentId("scout".into());
        let profile = AgentProfile::new(id.clone());
        roster.add(profile);

        let retrieved = roster.get(&id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, id);
    }

    #[test]
    fn crew_roster_get_missing() {
        let roster = CrewRoster::new();
        let id = AgentId("ghost".into());
        assert!(roster.get(&id).is_none());
    }

    #[test]
    fn crew_roster_best_for_mission() {
        let mut roster = CrewRoster::new();

        let id_slow = AgentId("slow".into());
        let mut slow = AgentProfile::new(id_slow.clone());
        slow.soul.speed = 0.2;
        slow.soul.reliability = 0.3;
        roster.add(slow);

        let id_fast = AgentId("fast".into());
        let mut fast = AgentProfile::new(id_fast.clone());
        fast.soul.speed = 0.9;
        fast.soul.reliability = 0.9;
        fast.soul.specializations = vec!["scouting".into()];
        roster.add(fast);

        let req = MissionRequirements {
            min_reliability: 0.7,
            min_speed: 0.7,
            required_specializations: vec!["scouting".into()],
            max_risk: 0.6,
            min_conservation: 0.3,
            stress_level: 0.0,
        };

        let best = roster.best_for_mission(&req);
        assert!(best.is_some());
        assert_eq!(best.unwrap(), id_fast);
    }

    #[test]
    fn crew_roster_best_for_mission_empty_roster() {
        let roster = CrewRoster::new();
        let req = MissionRequirements {
            min_reliability: 0.5,
            min_speed: 0.5,
            required_specializations: vec![],
            max_risk: 0.5,
            min_conservation: 0.5,
            stress_level: 0.0,
        };
        assert!(roster.best_for_mission(&req).is_none());
    }

    #[test]
    fn crew_roster_team_for_mission() {
        let mut roster = CrewRoster::new();
        for i in 0..5 {
            let mut p = AgentProfile::new(AgentId(format!("agent-{i}")));
            p.soul.speed = 0.5 + (i as f64) * 0.1;
            roster.add(p);
        }

        let req = MissionRequirements {
            min_reliability: 0.4,
            min_speed: 0.4,
            required_specializations: vec![],
            max_risk: 0.6,
            min_conservation: 0.4,
            stress_level: 0.0,
        };

        let team = roster.team_for_mission(&req, 3);
        assert_eq!(team.len(), 3);
    }

    #[test]
    fn crew_roster_team_smaller_than_requested() {
        let mut roster = CrewRoster::new();
        roster.add(AgentProfile::new(AgentId("lone".into())));

        let req = MissionRequirements {
            min_reliability: 0.4,
            min_speed: 0.4,
            required_specializations: vec![],
            max_risk: 0.6,
            min_conservation: 0.4,
            stress_level: 0.0,
        };

        let team = roster.team_for_mission(&req, 5);
        assert_eq!(team.len(), 1); // Only 1 agent available
    }

    #[test]
    fn crew_roster_roster_summary_empty() {
        let roster = CrewRoster::new();
        let summary = roster.roster_summary();
        assert_eq!(summary, "Crew roster is empty.");
    }

    #[test]
    fn crew_roster_roster_summary_non_empty() {
        let mut roster = CrewRoster::new();
        roster.add(AgentProfile::new(AgentId("alice".into())));
        roster.add(AgentProfile::new(AgentId("bob".into())));

        let summary = roster.roster_summary();
        assert!(summary.contains("alice"));
        assert!(summary.contains("bob"));
        assert!(summary.contains("Crew Roster"));
    }

    #[test]
    fn crew_roster_at_risk_agents() {
        let mut roster = CrewRoster::new();

        let mut good = AgentProfile::new(AgentId("good".into()));
        good.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("good".into()),
            outcome: MissionOutcome::Success,
            tick: 0,
            duration: 10,
            conservation_error: 0.0,
            notes: "".into(),
        });
        roster.add(good);

        let mut bad = AgentProfile::new(AgentId("bad".into()));
        bad.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("bad".into()),
            outcome: MissionOutcome::Failed,
            tick: 0,
            duration: 10,
            conservation_error: 0.8,
            notes: "".into(),
        });
        roster.add(bad);

        let at_risk = roster.at_risk_agents();
        assert_eq!(at_risk.len(), 1);
        assert_eq!(at_risk[0].id.to_string(), "bad");
    }

    #[test]
    fn crew_roster_specialization_coverage() {
        let mut roster = CrewRoster::new();
        let specialties = [
            ("alpha", vec!["scouting", "combat"]),
            ("beta", vec!["scouting"]),
            ("gamma", vec!["engineering"]),
        ];
        for (name, specs) in &specialties {
            let mut profile = AgentProfile::new(AgentId(name.to_string()));
            profile.soul.specializations = specs.iter().map(|s| s.to_string()).collect();
            roster.add(profile);
        }

        let coverage = roster.specialization_coverage();
        assert_eq!(*coverage.get("scouting").unwrap(), 2);
        assert_eq!(*coverage.get("combat").unwrap(), 1);
        assert_eq!(*coverage.get("engineering").unwrap(), 1);
        assert_eq!(coverage.len(), 3);
    }

    #[test]
    fn crew_roster_specialization_coverage_empty() {
        let roster = CrewRoster::new();
        let coverage = roster.specialization_coverage();
        assert!(coverage.is_empty());
    }

    // -- Serde round-trip ----------------------------------------------

    #[test]
    fn serde_round_trip_agent_profile() {
        let mut profile = AgentProfile::new(AgentId("serde-test".into()));
        profile.soul.specializations = vec!["spelunking".into()];
        profile.record_mission(MissionRecord {
            mission_id: "m1".into(),
            agent_id: AgentId("serde-test".into()),
            outcome: MissionOutcome::Success,
            tick: 42,
            duration: 100,
            conservation_error: 0.01,
            notes: "test mission".into(),
        });

        let json = serde_json::to_string_pretty(&profile).expect("serialize");
        let deserialized: AgentProfile = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deserialized.id, profile.id);
        assert_eq!(deserialized.soul.specializations, profile.soul.specializations);
        assert_eq!(deserialized.total_missions, 1);
        assert_eq!(deserialized.mission_history.len(), 1);
    }

    #[test]
    fn serde_round_trip_crew_roster() {
        let mut roster = CrewRoster::new();
        roster.add(AgentProfile::new(AgentId("alpha".into())));
        roster.add(AgentProfile::new(AgentId("beta".into())));

        let json = serde_json::to_string(&roster).expect("serialize");
        let deserialized: CrewRoster = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deserialized.profiles.len(), 2);
        assert!(deserialized.get(&AgentId("alpha".into())).is_some());
        assert!(deserialized.get(&AgentId("beta".into())).is_some());
    }

    #[test]
    fn serde_round_trip_enums() {
        // Trend
        let json = serde_json::to_string(&Trend::Improving).unwrap();
        assert_eq!(serde_json::from_str::<Trend>(&json).unwrap(), Trend::Improving);

        // StressResponse
        let json = serde_json::to_string(&StressResponse::Thrives).unwrap();
        assert_eq!(
            serde_json::from_str::<StressResponse>(&json).unwrap(),
            StressResponse::Thrives
        );

        // MissionOutcome
        let json = serde_json::to_string(&MissionOutcome::PartialSuccess).unwrap();
        assert_eq!(
            serde_json::from_str::<MissionOutcome>(&json).unwrap(),
            MissionOutcome::PartialSuccess
        );
    }

    // -- Edge cases ----------------------------------------------------

    #[test]
    fn is_reliable_no_missions() {
        let mut profile = AgentProfile::new(AgentId("fresh".into()));
        profile.soul.reliability = 0.9;
        // Default success_rate is 1.0 with 0 missions — edge case
        assert!(profile.is_reliable());
    }

    #[test]
    fn fits_mission_handles_extreme_values() {
        let mut profile = AgentProfile::new(AgentId("extreme".into()));
        profile.soul.reliability = 0.0;
        profile.soul.speed = 0.0;
        profile.soul.risk_tolerance = 1.0;
        profile.soul.conservation_adherence = 0.0;

        let req = MissionRequirements {
            min_reliability: 1.0,
            min_speed: 1.0,
            required_specializations: vec!["impossible".into()],
            max_risk: 0.0,
            min_conservation: 1.0,
            stress_level: 0.9,
        };

        let score = profile.fits_mission(&req);
        // Should still produce a valid f64 (not NaN, not infinite)
        assert!(score.is_finite());
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn genealogical_tracking() {
        let mut profile = AgentProfile::new(AgentId("child".into()));
        profile.genealogy = vec![
            AgentId("parent-a".into()),
            AgentId("parent-b".into()),
        ];
        assert_eq!(profile.genealogy.len(), 2);
        let summary = profile.summary();
        assert!(summary.contains("parent-a"));
        assert!(summary.contains("parent-b"));
    }

    #[test]
    fn mission_record_construction() {
        let record = MissionRecord {
            mission_id: "op-neptune".into(),
            agent_id: AgentId("scout-7".into()),
            outcome: MissionOutcome::PartialSuccess,
            tick: 200,
            duration: 50,
            conservation_error: 0.12,
            notes: "Navigation error cost 10 ticks.".into(),
        };
        assert_eq!(record.mission_id, "op-neptune");
        assert_eq!(record.outcome, MissionOutcome::PartialSuccess);
    }

    #[test]
    fn multiple_mission_records_affect_metrics_smoothly() {
        let mut profile = AgentProfile::new(AgentId("roller".into()));

        for i in 0..10 {
            let outcome = if i % 2 == 0 {
                MissionOutcome::Success
            } else {
                MissionOutcome::Failed
            };
            profile.record_mission(MissionRecord {
                mission_id: format!("m{i}"),
                agent_id: AgentId("roller".into()),
                outcome,
                tick: i,
                duration: 10,
                conservation_error: if outcome == MissionOutcome::Success {
                    0.05
                } else {
                    0.5
                },
                notes: "".into(),
            });
        }

        assert_eq!(profile.total_missions, 10);
        // 5 out of 10 were successes
        assert!((profile.success_rate - 0.5).abs() < 0.06);
        // After 10 missions, history should have 10 entries
        assert_eq!(profile.mission_history.len(), 10);
    }

    #[test]
    fn crew_roster_default_impl() {
        let roster = CrewRoster::default();
        assert!(roster.profiles.is_empty());
    }
}
