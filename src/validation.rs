use crate::models::{ModuleRegistry, Plan, Student};
use anyhow::Result;

pub mod prerequisites {
    use super::*;

    pub fn validate_prerequisites(
        _plan: &Plan,
        _registry: &ModuleRegistry,
        _student: &Student,
        _result: &mut ValidationResult,
    ) -> Result<()> {
        // In a real implementation, this would check prerequisites
        // For testing, we'll just return Ok
        Ok(())
    }
}

pub mod workload {
    use super::*;

    pub fn validate_workload(
        _plan: &Plan,
        _registry: &ModuleRegistry,
        _result: &mut ValidationResult,
    ) -> Result<()> {
        // In a real implementation, this would check workload
        // For testing, we'll just return Ok
        Ok(())
    }
}

pub mod availability {
    use super::*;

    pub fn validate_availability(
        _plan: &Plan,
        _registry: &ModuleRegistry,
        _result: &mut ValidationResult,
    ) -> Result<()> {
        // In a real implementation, this would check availability
        // For testing, we'll just return Ok
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub level: ValidationLevel,
    pub message: String,
    pub module_code: Option<String>,
    pub semester_index: Option<usize>,
}

#[derive(Debug, Default)]
pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    pub fn add_error(
        &mut self,
        message: &str,
        module_code: Option<&str>,
        semester_index: Option<usize>,
    ) {
        self.issues.push(ValidationIssue {
            level: ValidationLevel::Error,
            message: message.to_string(),
            module_code: module_code.map(|s| s.to_string()),
            semester_index,
        });
    }

    pub fn add_warning(
        &mut self,
        message: &str,
        module_code: Option<&str>,
        semester_index: Option<usize>,
    ) {
        self.issues.push(ValidationIssue {
            level: ValidationLevel::Warning,
            message: message.to_string(),
            module_code: module_code.map(|s| s.to_string()),
            semester_index,
        });
    }

    pub fn add_info(
        &mut self,
        message: &str,
        module_code: Option<&str>,
        semester_index: Option<usize>,
    ) {
        self.issues.push(ValidationIssue {
            level: ValidationLevel::Info,
            message: message.to_string(),
            module_code: module_code.map(|s| s.to_string()),
            semester_index,
        });
    }
}
