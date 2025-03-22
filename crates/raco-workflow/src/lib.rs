//! Workflow engine for RACO
//!
//! This crate provides the workflow engine functionality for RACO,
//! including workflow definition, execution, and state management.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

use uuid::Uuid;

pub mod engine;
pub mod steps;

/// Initialize the workflow engine
pub fn init() -> Result<(), raco_core::error::CoreError> {
    tracing::info!("Initializing workflow engine");
    Ok(())
}

/// The unique identifier for a workflow
pub type WorkflowId = Uuid;

/// The unique identifier for a workflow step
pub type StepId = Uuid;

/// The status of a workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowStatus {
    /// Workflow is pending execution
    Pending,
    /// Workflow is currently running
    Running,
    /// Workflow is paused, waiting for human input
    WaitingForInput,
    /// Workflow has completed successfully
    Completed,
    /// Workflow has failed
    Failed,
    /// Workflow has been cancelled
    Cancelled,
}

/// The status of a workflow step
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    /// Step is pending execution
    Pending,
    /// Step is currently running
    Running,
    /// Step is paused, waiting for human input
    WaitingForInput,
    /// Step has completed successfully
    Completed,
    /// Step has failed
    Failed,
    /// Step has been skipped
    Skipped,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
