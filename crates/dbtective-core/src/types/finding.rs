use crate::rules::registry::{RuleInfo, RuleRegistry};

use super::severity::RuleSeverity;

#[derive(Debug, Clone)]
pub struct Finding {
    pub rule_id: String,
    pub severity: RuleSeverity,
    pub message: String,
    pub resource_name: String,
    pub file_path: String,
}

impl Finding {
    pub fn new(
        rule_id: String,
        severity: RuleSeverity,
        message: String,
        resource_name: String,
        file_path: String,
    ) -> Self {
        Self {
            rule_id,
            severity,
            message,
            resource_name,
            file_path: file_path,
        }
    }

    pub fn get_rule_info(&self) -> Option<&'static RuleInfo> {
        RuleRegistry::get_rule_by_str(&self.rule_id)
    }

    pub fn get_rule_name(&self) -> Option<&'static str> {
        self.get_rule_info().map(|info| info.name)
    }
}
