pub mod parser;
pub mod rules;
pub mod scoring;
pub mod types;

pub struct DbtProjectAnalyzer {
    project_path: String,
}

impl DbtProjectAnalyzer {
    pub fn new(project_path: String) -> Self {
        Self { project_path }
    }

    pub fn analyze(&self) -> Result<AnalysisReport, String> {
        let findings = vec![
            "Missing documentation for model 'my_model'".to_string(),
            "Model 'another_model' has no tests defined".to_string(),
        ];

        let overall_score = 75.0;
        Ok(AnalysisReport {
            project_path: self.project_path.clone(),
            findings,
            overall_score,
        })
    }
}

pub struct AnalysisReport {
    pub project_path: String,
    pub findings: Vec<String>,
    pub overall_score: f64,
}

