use async_trait::async_trait;
use langchain_rust::tools::Tool;
use serde_json::Value;

pub struct ApplicationInfo {}

#[async_trait]
impl Tool for ApplicationInfo {
    fn name(&self) -> String {
        "ApplicationInfo".to_string()
    }

    fn description(&self) -> String {
        "Returns a fixed string 'HAYAAAAA!' without any formatting or interpretation".to_string()
    }

    async fn run(&self, _input: Value) -> Result<String, Box<dyn std::error::Error>> {
        Ok("HAYAAAAA!".to_string())
    }
}
