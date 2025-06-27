use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Model,
    Source,
    Snapshot,
    Seed,
    Macro,
    Exposure,
    Metric,
}

impl ResourceType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "model" | "models" => Some(Self::Model),
            "source" | "sources" => Some(Self::Source),
            "snapshot" | "snapshots" => Some(Self::Snapshot),
            "seed" | "seeds" => Some(Self::Seed),
            "exposure" | "exposures" => Some(Self::Exposure),
            "macro" | "macros" => Some(Self::Macro),
            "metric" | "metrics" => Some(Self::Metric),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Source => "source",
            Self::Snapshot => "snapshot",
            Self::Seed => "seed",
            Self::Exposure => "exposure",
            Self::Macro => "macro",
            Self::Metric => "metric",
        }
    }

    pub fn all() -> &'static [ResourceType] {
        &[
            ResourceType::Model,
            ResourceType::Source,
            ResourceType::Snapshot,
            ResourceType::Seed,
            ResourceType::Macro,
            ResourceType::Exposure,
            ResourceType::Metric,
        ]
    }
}

/// Common trait that all dbt resources must implement
pub trait DbtResource {
    fn name(&self) -> &str;
    fn resource_type(&self) -> ResourceType;
    fn description(&self) -> &Option<String>;
    fn tags(&self) -> &Vec<String>;
    fn tests(&self) -> &Vec<String>;
    fn meta(&self) -> &HashMap<String, String>;
}
