use crate::rules::registry::RuleInfo;
use crate::types::*;

pub struct DescriptionRule;
impl DescriptionRule {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&self, resource: &DbtResource) -> RuleResult {
        match &resource.description {
            Some(desc) if !desc.trim().is_empty() => RuleResult {
                rule_id: RuleId::DOC001,
                passed: true,
                message: None,
                resource_name: resource.name.clone(),
            },
            _ => {
                rule_id = RuleId::DOC001;
                RuleResult {
                    rule_id: RuleId::DOC001,
                    passed: false,
                    message: Some(
                        format!("{}:Resource {} is missing a description"),
                        rule_id.as_str(),
                        resource.name.clone(),
                    ),
                }
            }
        }
    }
}

impl Rule for DescriptionRule {
    fn id(&self) -> RuleId {
        RuleId::DOC001
    }

    fn evaluate(&self, resource: &DbtResource) -> RuleResult {
        self.check(resource)
    }
}
