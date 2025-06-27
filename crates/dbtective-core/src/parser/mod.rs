pub mod merger;
pub mod sql;
pub mod yaml;

use crate::types::dbt::{DbtResource, Model, ResourceType, Source};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct DbtProjectParser {
    project_path: PathBuf,
}

impl DbtProjectParser {
    pub fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }

    pub fn parse_project(&self) -> Result<Vec<Box<dyn DbtResource>>> {
        let mut resources: Vec<Box<dyn DbtResource>> = Vec::new();

        // Step 1: Parse all SQL files to get base resources + SQL configs
        let sql_resources = self.parse_sql_files()?;

        // Step 2: Parse all YAML files to get YAML configs
        let yaml_configs = self.parse_yaml_files()?;

        // Step 3: Merge SQL + YAML configs using precedence rules
        let merged_resources = merger::merge_configurations(sql_resources, yaml_configs)?;

        Ok(merged_resources)
    }

    fn parse_sql_files(&self) -> Result<HashMap<String, sql::SqlResourceInfo>> {
        let mut sql_resources = HashMap::new();

        for entry in WalkDir::new(&self.project_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "sql"))
        {
            let content = std::fs::read_to_string(entry.path())?;
            let resource_info = sql::parse_sql_file(entry.path(), &content)?;
            sql_resources.insert(resource_info.name.clone(), resource_info);
        }

        Ok(sql_resources)
    }

    fn parse_yaml_files(&self) -> Result<HashMap<String, yaml::YamlResourceConfig>> {
        let mut yaml_configs = HashMap::new();

        for entry in WalkDir::new(&self.project_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map_or(false, |ext| ext == "yml" || ext == "yaml")
            })
        {
            let content = std::fs::read_to_string(entry.path())?;
            let schema = yaml::parse_schema_file(&content)?;

            // Extract individual resource configs from the schema file
            let resource_configs = yaml::extract_resource_configs(schema, entry.path())?;

            for (name, config) in resource_configs {
                yaml_configs.insert(name, config);
            }
        }

        Ok(yaml_configs)
    }
}
