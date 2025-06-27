use super::resource::{DbtResource, ResourceType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub tests: Vec<String>,
    pub meta: HashMap<String, String>,
}

impl Model {
    pub fn new(name: String) -> Self {
        Self {
            name,
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

impl DbtResource for Model {
    fn name(&self) -> &str {
        &self.name
    }

    fn resource_type(&self) -> ResourceType {
        ResourceType::Model
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
