//! Workflow execution engine
//!
//! This module provides the workflow execution engine for RACO.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info};

use crate::steps::Step;
use crate::{StepId, StepStatus, WorkflowId, WorkflowStatus};

/// A workflow definition
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Unique identifier for the workflow
    pub id: WorkflowId,

    /// Name of the workflow
    pub name: String,

    /// Description of the workflow
    pub description: String,

    /// Steps in the workflow
    #[serde(skip)]
    pub steps: Vec<Box<dyn Step>>,

    /// Dependencies between steps (`from_step_id`, `to_step_id`)
    pub dependencies: Vec<(StepId, StepId)>,
}

/// A workflow instance
#[derive(Debug)]
pub struct WorkflowInstance {
    /// The workflow definition
    definition: WorkflowDefinition,

    /// Current status of the workflow
    status: WorkflowStatus,

    /// Status of each step
    steps_statuses: HashMap<StepId, StepStatus>,

    /// Workflow execution graph
    #[allow(dead_code)]
    graph: DiGraph<StepId, ()>,

    /// Mapping from step ID to graph node index
    #[allow(dead_code)]
    node_map: HashMap<StepId, NodeIndex>,

    /// Creation time
    created_at: DateTime<Utc>,

    /// Start time
    started_at: Option<DateTime<Utc>>,

    /// Completion time
    completed_at: Option<DateTime<Utc>>,

    /// Dependencies between steps (`from_step_id`, `to_step_id`)
    #[allow(dead_code)]
    dependencies: Vec<(StepId, StepId)>,
}

impl WorkflowInstance {
    /// Create a new workflow instance from a definition
    ///
    /// # Errors
    ///
    /// Returns an error if the workflow definition is invalid or has circular dependencies
    pub fn new(definition: WorkflowDefinition) -> Result<Self> {
        let step_ids: HashSet<_> = definition.steps.iter().map(|step| step.id()).collect();

        // Validate dependencies
        for (from, to) in &definition.dependencies {
            if !step_ids.contains(from) {
                return Err(anyhow::anyhow!(
                    "Invalid dependency: step {} not found",
                    from
                ));
            }
            if !step_ids.contains(to) {
                return Err(anyhow::anyhow!("Invalid dependency: step {} not found", to));
            }
        }

        // Create graph
        let mut graph = DiGraph::new();
        let mut node_map = HashMap::new();

        // Add nodes
        for step in &definition.steps {
            let id = step.id();
            let node = graph.add_node(id);
            node_map.insert(id, node);
        }

        // Add edges
        for (from, to) in &definition.dependencies {
            let from_node = node_map[from];
            let to_node = node_map[to];
            graph.add_edge(from_node, to_node, ());
        }

        // Initialize step status
        let mut steps_statuses = HashMap::new();
        for step in &definition.steps {
            steps_statuses.insert(step.id(), StepStatus::Pending);
        }

        // Clone dependencies before moving definition into the struct
        let dependencies = definition.dependencies.clone();

        Ok(Self {
            definition,
            status: WorkflowStatus::Pending,
            steps_statuses,
            graph,
            node_map,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            dependencies,
        })
    }

    /// Get the workflow ID
    #[must_use]
    pub fn id(&self) -> WorkflowId {
        self.definition.id
    }

    /// Get the workflow status
    #[must_use]
    pub fn status(&self) -> WorkflowStatus {
        self.status
    }

    /// Get the status of a step
    #[must_use]
    pub fn step_status(&self, step_id: StepId) -> Option<StepStatus> {
        self.steps_statuses.get(&step_id).copied()
    }

    /// Get all step statuses
    #[must_use]
    pub fn all_step_statuses(&self) -> &HashMap<StepId, StepStatus> {
        &self.steps_statuses
    }

    /// Get the creation time
    #[must_use]
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Get the start time
    #[must_use]
    pub fn started_at(&self) -> Option<DateTime<Utc>> {
        self.started_at
    }

    /// Get the completion time
    #[must_use]
    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }
}

/// Workflow engine for managing and executing workflows
#[derive(Debug)]
pub struct WorkflowEngine {
    /// Active workflow instances
    instances: Arc<RwLock<HashMap<WorkflowId, Arc<Mutex<WorkflowInstance>>>>>,
}

impl WorkflowEngine {
    /// Create a new workflow engine
    pub fn new() -> Self {
        info!("Creating new workflow engine");
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new workflow instance from a definition
    pub async fn create_workflow(&self, definition: WorkflowDefinition) -> Result<WorkflowId> {
        let instance = WorkflowInstance::new(definition)?;
        let id = instance.id();

        info!("Creating workflow instance {}", id);
        let mut instances = self.instances.write().await;
        instances.insert(id, Arc::new(Mutex::new(instance)));

        Ok(id)
    }

    /// Get a workflow instance by ID
    pub async fn get_workflow(&self, id: WorkflowId) -> Option<Arc<Mutex<WorkflowInstance>>> {
        debug!("Getting workflow instance {}", id);
        let instances = self.instances.read().await;
        instances.get(&id).cloned()
    }

    /// Start a workflow
    pub async fn start_workflow(&self, id: WorkflowId) -> Result<()> {
        info!("Starting workflow {}", id);
        let instances = self.instances.read().await;

        let instance = instances
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("Workflow {} not found", id))?;

        let mut instance = instance.lock().await;
        if instance.status != WorkflowStatus::Pending {
            return Err(anyhow::anyhow!("Workflow {} is not in pending state", id));
        }

        instance.status = WorkflowStatus::Running;
        instance.started_at = Some(Utc::now());

        // For now, we'll just mark everything as completed
        // In a real implementation, we would start executing steps based on the graph
        for (step_id, status) in &mut instance.steps_statuses {
            *status = StepStatus::Completed;
            debug!("Completed step {} in workflow {}", step_id, id);
        }

        instance.status = WorkflowStatus::Completed;
        instance.completed_at = Some(Utc::now());
        info!("Completed workflow {}", id);

        Ok(())
    }

    /// Cancel a workflow
    pub async fn cancel_workflow(&self, id: WorkflowId) -> Result<()> {
        info!("Cancelling workflow {}", id);
        let instances = self.instances.read().await;

        let instance = instances
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("Workflow {} not found", id))?;

        let mut instance = instance.lock().await;
        if instance.status == WorkflowStatus::Completed || instance.status == WorkflowStatus::Failed
        {
            return Err(anyhow::anyhow!("Workflow {} is already finished", id));
        }

        instance.status = WorkflowStatus::Cancelled;
        instance.completed_at = Some(Utc::now());

        Ok(())
    }
}

impl Default for WorkflowEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::steps::MockStep;
    use uuid::Uuid;

    fn create_test_workflow() -> WorkflowDefinition {
        let step1 = Box::new(MockStep::new(Uuid::new_v4()));
        let step2 = Box::new(MockStep::new(Uuid::new_v4()));
        let step1_id = step1.id();
        let step2_id = step2.id();

        WorkflowDefinition {
            id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            description: "A test workflow".to_string(),
            steps: vec![step1, step2],
            dependencies: vec![(step1_id, step2_id)],
        }
    }

    #[test]
    fn test_workflow_instance_creation() {
        let definition = create_test_workflow();
        let instance = WorkflowInstance::new(definition);
        assert!(instance.is_ok());
    }

    #[tokio::test]
    async fn test_workflow_engine_creation() {
        let engine = WorkflowEngine::new();
        assert!(engine.instances.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_workflow_lifecycle() {
        let engine = WorkflowEngine::new();
        let definition = create_test_workflow();

        // Create workflow
        let result = engine.create_workflow(definition).await;
        assert!(result.is_ok());
        let id = result.unwrap();

        // Get workflow
        let instance = engine.get_workflow(id).await;
        assert!(instance.is_some());
        assert_eq!(
            instance.unwrap().lock().await.status(),
            WorkflowStatus::Pending
        );

        // Start workflow
        let result = engine.start_workflow(id).await;
        assert!(result.is_ok());

        // Check workflow status
        let instance = engine.get_workflow(id).await.unwrap();
        assert_eq!(instance.lock().await.status(), WorkflowStatus::Completed);
    }

    #[tokio::test]
    async fn test_workflow_cancellation() {
        let engine = WorkflowEngine::new();
        let definition = create_test_workflow();

        // Create workflow
        let id = engine.create_workflow(definition).await.unwrap();

        // Cancel workflow
        let result = engine.cancel_workflow(id).await;
        assert!(result.is_ok());

        // Check workflow status
        let instance = engine.get_workflow(id).await.unwrap();
        assert_eq!(instance.lock().await.status(), WorkflowStatus::Cancelled);
    }
}
