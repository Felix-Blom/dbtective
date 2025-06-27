use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaFile {
    pub version: Option<u8>,
    pub models: Option<Vec<ModelYamlConfig>>,
    pub sources: Option<Vec<SourceYamlConfig>>,
    pub snapshots: Option<Vec<SnapshotYamlConfig>>,
    pub seeds: Option<Vec<SeedYamlConfig>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelYamlConfig {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub meta: Option<HashMap<String, serde_yaml::Value>>,
    pub columns: Option<Vec<ColumnYamlConfig>>,
    pub tests: Option<Vec<TestYamlConfig>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceYamlConfig {
    pub name: String,
    pub description: Option<String>,
    pub tables: Option<Vec<TableYamlConfig>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableYamlConfig {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub meta: Option<HashMap<String, serde_yaml::Value>>,
    pub columns: Option<Vec<ColumnYamlConfig>>,
    pub tests: Option<Vec<TestYamlConfig>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SnapshotYamlConfig {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub meta: Option<HashMap<String, serde_yaml::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeedYamlConfig {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub meta: Option<HashMap<String, serde_yaml::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColumnYamlConfig {
    pub name: String,
    pub description: Option<String>,
    pub tests: Option<Vec<TestYamlConfig>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TestYamlConfig {
    Simple(String),
    Complex(HashMap<String, serde_yaml::Value>),
}

#[derive(Debug, Clone)]
pub struct YamlResourceConfig {
    pub name: String,
    pub resource_type: crate::types::dbt::ResourceType,
    pub file_path: std::path::PathBuf,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
    pub tests: Vec<String>,
    pub source_name: Option<String>, // For source tables
}

pub fn parse_schema_file(content: &str) -> Result<SchemaFile> {
    let schema: SchemaFile = serde_yaml::from_str(content)?;
    Ok(schema)
}

pub fn extract_resource_configs(
    schema: SchemaFile,
    file_path: &Path,
) -> Result<HashMap<String, YamlResourceConfig>> {
    let mut configs = HashMap::new();

    // Extract model configs
    if let Some(models) = schema.models {
        for model in models {
            let config = YamlResourceConfig {
                name: model.name.clone(),
                resource_type: crate::types::dbt::ResourceType::Model,
                file_path: file_path.to_path_buf(),
                description: model.description,
                tags: model.tags.unwrap_or_default(),
                meta: convert_meta_to_string_map(model.meta.unwrap_or_default()),
                tests: extract_test_names(model.tests.unwrap_or_default()),
                source_name: None,
            };
            configs.insert(model.name, config);
        }
    }

    // Extract source table configs
    if let Some(sources) = schema.sources {
        for source in sources {
            if let Some(tables) = source.tables {
                for table in tables {
                    let config = YamlResourceConfig {
                        name: table.name.clone(),
                        resource_type: crate::types::dbt::ResourceType::Source,
                        file_path: file_path.to_path_buf(),
                        description: table.description,
                        tags: table.tags.unwrap_or_default(),
                        meta: convert_meta_to_string_map(table.meta.unwrap_or_default()),
                        tests: extract_test_names(table.tests.unwrap_or_default()),
                        source_name: Some(source.name.clone()),
                    };
                    // Use "source.table" as the key for source tables
                    let key = format!("{}.{}", source.name, table.name);
                    configs.insert(key, config);
                }
            }
        }
    }

    // TODO: Extract snapshots, seeds, etc.

    Ok(configs)
}

fn convert_meta_to_string_map(meta: HashMap<String, serde_yaml::Value>) -> HashMap<String, String> {
    meta.into_iter()
        .map(|(k, v)| (k, value_to_string(v)))
        .collect()
}

fn value_to_string(value: serde_yaml::Value) -> String {
    match value {
        serde_yaml::Value::String(s) => s,
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        _ => serde_yaml::to_string(&value).unwrap_or_default(),
    }
}

fn extract_test_names(tests: Vec<TestYamlConfig>) -> Vec<String> {
    tests
        .into_iter()
        .map(|test| match test {
            TestYamlConfig::Simple(name) => name,
            TestYamlConfig::Complex(map) => {
                // For complex tests, use the first key as the test name
                map.keys().next().unwrap_or(&"unknown".to_string()).clone()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_model_schema() {
        let yaml_content = r#"
version: 2

models:
  - name: users
    description: "User data from our application"
    tags: ["core", "daily"]
    columns:
      - name: id
        description: "Primary key"
        tests:
          - unique
          - not_null
"#;

        let schema = parse_schema_file(yaml_content).unwrap();
        assert!(schema.models.is_some());

        let models = schema.models.unwrap();
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].name, "users");
        assert_eq!(
            models[0].description,
            Some("User data from our application".to_string())
        );
    }

    #[test]
    fn test_parse_sources_schema() {
        let yaml_content = r#"
version: 2

sources:
  - name: raw_database
    tables:
      - name: users
        description: "Raw user data"
      - name: orders
        description: "Raw order data"
"#;

        let schema = parse_schema_file(yaml_content).unwrap();
        assert!(schema.sources.is_some());

        let sources = schema.sources.unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].name, "raw_database");

        let tables = sources[0].tables.as_ref().unwrap();
        assert_eq!(tables.len(), 2);
        assert_eq!(tables[0].name, "users");
    }
}
