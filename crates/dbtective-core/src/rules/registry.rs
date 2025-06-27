#[derive(Debug, Clone)]
pub enum RuleCategory {
    Documentation,
    Testing,
    Metadata,
    Convention,
}

impl RuleCategory {
    pub fn prefix(&self) -> &'static str {
        match self {
            RuleCategory::Documentation => "DOC",
            RuleCategory::Testing => "TEST",
            RuleCategory::Metadata => "META",
            RuleCategory::Convention => "CONV",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleId {
    DOC001,
    TEST001,
    TEST002,
    TEST003,
}

impl RuleId {
    pub fn as_str(&self) -> &'static str {
        match self {
            RuleId::DOC001 => "DOC001",
            RuleId::TEST001 => "TEST001",
            RuleId::TEST002 => "TEST002",
            RuleId::TEST003 => "TEST003",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "DOC001" => Some(RuleId::DOC001),
            "TEST001" => Some(RuleId::TEST001),
            "TEST002" => Some(RuleId::TEST002),
            "TEST003" => Some(RuleId::TEST003),
            _ => None,
        }
    }
}

pub struct RuleInfo {
    pub id: RuleId,
    pub category: RuleCategory,
    pub name: &'static str,
    pub short_description: &'static str,
    pub long_description: &'static str,
}

pub struct RuleRegistry;
impl RuleRegistry {
    pub fn get_rule(id: &RuleId) -> Option<&'static RuleInfo> {
        match id {
            RuleId::DOC001 => Some(&Self::DESCRIPTION),
            RuleId::TEST001 => Some(&Self::TESTS),
            RuleId::TEST002 => Some(&Self::MODEL_TESTS),
            RuleId::TEST003 => Some(&Self::COLUMN_TESTS),
        }
    }

    pub fn get_rule_by_str(id: &str) -> Option<&'static RuleInfo> {
        RuleId::from_str(id).and_then(|rule_id| Self::get_rule(&rule_id))
    }
    pub fn get_all_rules() -> Vec<&'static RuleInfo> {
        vec![
            &Self::DESCRIPTION,
            &Self::TESTS,
            &Self::MODEL_TESTS,
            &Self::COLUMN_TESTS,
        ]
    }

    pub fn get_rules_by_category(category: &RuleCategory) -> Vec<&'static RuleInfo> {
        Self::get_all_rules()
            .into_iter()
            .filter(|rule| {
                std::mem::discriminant(&rule.category) == std::mem::discriminant(category)
            })
            .collect()
    }

    pub const DESCRIPTION: RuleInfo = RuleInfo {
        id: RuleId::DOC001,
        category: RuleCategory::Documentation,
        name: "description",
        short_description: "Resources must have description",
        long_description: include_str!("definitions/DOC/DOC001.md"),
    };

    pub const TESTS: RuleInfo = RuleInfo {
        id: RuleId::TEST001,
        category: RuleCategory::Testing,
        name: "tests",
        short_description: "Models must have tests (either column or model-level",
        long_description: include_str!("definitions/TEST/TEST001.md"),
    };

    pub const MODEL_TESTS: RuleInfo = RuleInfo {
        id: RuleId::TEST002,
        category: RuleCategory::Testing,
        name: "model-tests",
        short_description: "Models must have model-level tests",
        long_description: include_str!("definitions/TEST/TEST002.md"),
    };

    pub const COLUMN_TESTS: RuleInfo = RuleInfo {
        id: RuleId::TEST003,
        category: RuleCategory::Testing,
        name: "column-tests",
        short_description: "Specific columns must have tests",
        long_description: include_str!("definitions/TEST/TEST003.md"),
    };
}
