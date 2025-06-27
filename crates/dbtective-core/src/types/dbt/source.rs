use super::resource::{DbtResource, ResourceType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Source {
    pub name: String,
    pub source_name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub tests: Vec<String>,
    pub meta: HashMap<String, String>,
}

impl Source {
    pub fn new(name: String, source_name: String) -> Self {
        Self {
            name,
            source_name,
            description: None,
            tags: Vec::new(),
            tests: Vec::new(),
            meta: HashMap::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

impl DbtResource for Source {
    fn name(&self) -> &str {
        &self.name
    }

    fn resource_type(&self) -> ResourceType {
        ResourceType::Source
    }

    fn description(&self) -> &Option<String> {
        &self.description
    }

    fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    fn tests(&self) -> &Vec<String> {
        &self.tests
    }

    fn meta(&self) -> &HashMap<String, String> {
        &self.meta
    }
}
