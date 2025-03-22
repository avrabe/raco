//! Workflow step definitions
//!
//! This module provides the workflow step trait and implementations.

use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;
use serde_json;
#[cfg(test)]
use uuid::Uuid;

use crate::StepId;
use crate::StepStatus;

/// Context for step execution
#[derive(Debug, Clone)]
pub struct StepContext {
    /// Input data for the step
    pub input: serde_json::Value,

    /// Output from previous steps
    pub previous_outputs: std::collections::HashMap<StepId, serde_json::Value>,

    /// Global workflow context
    pub global: std::collections::HashMap<String, serde_json::Value>,
}

/// Result of step execution
#[derive(Debug, Clone)]
pub struct StepResult {
    /// Output data from the step
    pub output: serde_json::Value,

    /// Status after execution
    pub status: StepStatus,

    /// Error message, if any
    pub error: Option<String>,
}

/// Trait defining a workflow step
#[async_trait]
pub trait Step: Debug + Send + Sync {
    /// Get the unique identifier for this step
    fn id(&self) -> StepId;

    /// Get the name of this step
    fn name(&self) -> &str;

    /// Get the description of this step
    fn description(&self) -> &str;

    /// Get the expected input schema for this step
    fn input_schema(&self) -> Option<serde_json::Value>;

    /// Get the expected output schema for this step
    fn output_schema(&self) -> Option<serde_json::Value>;

    /// Execute the step
    async fn execute(&self, context: StepContext) -> Result<StepResult>;

    /// Validate step input
    ///
    /// # Errors
    ///
    /// Returns an error if the input does not meet the step's requirements
    /// or if the input schema validation fails
    fn validate_input(&self, input: &serde_json::Value) -> Result<()>;

    /// Check if this step requires human input
    fn requires_human_input(&self) -> bool {
        false
    }

    /// Get human input prompt, if needed
    fn human_input_prompt(&self) -> Option<String> {
        None
    }
}

/// A simple human input step
#[derive(Debug)]
pub struct HumanInputStep {
    /// Step ID
    id: StepId,

    /// Step name
    name: String,

    /// Step description
    description: String,

    /// Input prompt
    prompt: String,
}

impl HumanInputStep {
    /// Create a new human input step
    pub fn new(id: StepId, name: String, description: String, prompt: String) -> Self {
        Self {
            id,
            name,
            description,
            prompt,
        }
    }
}

#[async_trait]
impl Step for HumanInputStep {
    fn id(&self) -> StepId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn input_schema(&self) -> Option<serde_json::Value> {
        None // Human input step doesn't require specific input schema
    }

    fn output_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "human_input": { "type": "string" }
            }
        }))
    }

    async fn execute(&self, _context: StepContext) -> Result<StepResult> {
        // In a real implementation, this would wait for human input
        // For now, we'll just return a dummy result
        Ok(StepResult {
            output: serde_json::json!({"human_input": "dummy response"}),
            status: StepStatus::Completed,
            error: None,
        })
    }

    fn validate_input(&self, _input: &serde_json::Value) -> Result<()> {
        // No specific validation for human input
        Ok(())
    }

    fn requires_human_input(&self) -> bool {
        true
    }

    fn human_input_prompt(&self) -> Option<String> {
        Some(self.prompt.clone())
    }
}

/// A simple code generation step
#[derive(Debug)]
pub struct CodeGenerationStep {
    /// Step ID
    id: StepId,

    /// Step name
    name: String,

    /// Step description
    description: String,

    /// Template for code generation
    #[allow(dead_code)]
    template: String,
}

impl CodeGenerationStep {
    /// Create a new code generation step
    pub fn new(id: StepId, name: String, description: String, template: String) -> Self {
        Self {
            id,
            name,
            description,
            template,
        }
    }
}

#[async_trait]
impl Step for CodeGenerationStep {
    fn id(&self) -> StepId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn input_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "parameters": { "type": "object" }
            }
        }))
    }

    fn output_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "generated_code": { "type": "string" }
            }
        }))
    }

    async fn execute(&self, _context: StepContext) -> Result<StepResult> {
        // In a real implementation, this would generate code based on the template
        // For now, we'll just return a dummy result
        Ok(StepResult {
            output: serde_json::json!({"generated_code": "// Generated code\nfn main() {\n    println!(\"Hello, world!\");\n}"}),
            status: StepStatus::Completed,
            error: None,
        })
    }

    fn validate_input(&self, _input: &serde_json::Value) -> Result<()> {
        // Validate that we have the necessary inputs for code generation
        Ok(())
    }
}

// For testing purposes
#[cfg(test)]
#[derive(Debug)]
pub(crate) struct MockStep {
    id: StepId,
}

#[cfg(test)]
impl MockStep {
    pub fn new(id: StepId) -> Self {
        Self { id }
    }
}

#[cfg(test)]
#[async_trait]
impl Step for MockStep {
    fn id(&self) -> StepId {
        self.id
    }

    fn name(&self) -> &str {
        "Mock Step"
    }

    fn description(&self) -> &str {
        "A mock step for testing"
    }

    fn input_schema(&self) -> Option<serde_json::Value> {
        None // Mock step doesn't have a specific input schema
    }

    fn output_schema(&self) -> Option<serde_json::Value> {
        None // Mock step doesn't have a specific output schema
    }

    async fn execute(&self, _context: StepContext) -> Result<StepResult> {
        Ok(StepResult {
            output: serde_json::json!({}),
            status: StepStatus::Completed,
            error: None,
        })
    }

    fn validate_input(&self, _input: &serde_json::Value) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_input_step() {
        let step = HumanInputStep::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "Test description".to_string(),
            "Please provide input".to_string(),
        );

        assert!(step.requires_human_input());
        assert_eq!(
            step.human_input_prompt(),
            Some("Please provide input".to_string())
        );
    }

    #[tokio::test]
    async fn test_step_execution() {
        let step = HumanInputStep::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "Test description".to_string(),
            "Please provide input".to_string(),
        );

        let context = StepContext {
            input: serde_json::json!({}),
            previous_outputs: std::collections::HashMap::new(),
            global: std::collections::HashMap::new(),
        };

        let result = step.execute(context).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.status, StepStatus::Completed);
        assert!(result.error.is_none());
    }
}
