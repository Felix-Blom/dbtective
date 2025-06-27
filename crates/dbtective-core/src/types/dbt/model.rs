use super::resource::{DbtResource, ResourceType};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub tests: Vec<String>,
    pub meta: HashMap<String, String>,
    pub materialized: Option<String>,
    pub sql_file: Option<PathBuf>,
    pub yaml_file: Option<PathBuf>,
}
impl Model {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            tags: Vec::new(),
            tests: Vec::new(),
            meta: HashMap::new(),
            materialized: None,
            sql_file: None,
            yaml_file: None,
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
