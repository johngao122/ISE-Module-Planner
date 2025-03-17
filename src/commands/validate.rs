// src/commands/validate.rs
use crate::api::NusmodsApi;
use crate::models::Plan;
use crate::storage::Storage;
use crate::validation::{availability, prerequisites, workload, ValidationLevel, ValidationResult};
use anyhow::{anyhow, Result};
use colored::Colorize;

pub struct ValidateCommand {
    pub storage: Box<dyn Storage>,
    pub api: Box<dyn NusmodsApi>,
    pub plan_id: String,
}

impl super::Command for ValidateCommand {
    fn run(&self) -> Result<()> {
        // Load the plan
        let plan = self
            .storage
            .get_plan(&self.plan_id)?
            .ok_or_else(|| anyhow!("Plan not found"))?;

        // Load student profile
        let student = self
            .storage
            .get_student(&plan.student_id)?
            .ok_or_else(|| anyhow!("Student profile not found"))?;

        // Load module registry
        let registry = self.api.get_module_registry()?;

        // Load curriculum
        let _curriculum = self
            .storage
            .get_curriculum(&student.major)?
            .ok_or_else(|| anyhow!("Curriculum not found"))?;

        println!("\n🔍 Validating plan: {}\n", plan.name.bold());

        // Run validations
        let mut result = ValidationResult::new();

        // Prerequisite validation
        prerequisites::validate_prerequisites(&plan, &registry, &student, &mut result)?;

        // Workload validation
        workload::validate_workload(&plan, &registry, &mut result)?;

        // Module availability validation
        availability::validate_availability(&plan, &registry, &mut result)?;

        // Graduation requirements validation
        // (Will be more complex in a full implementation)

        // Display validation results
        self.display_validation_results(&result, &plan);

        Ok(())
    }
}

impl ValidateCommand {
    pub fn display_validation_results(&self, result: &ValidationResult, plan: &Plan) {
        if result.issues.is_empty() {
            println!(
                "{}",
                "✅ No issues found! Your plan is valid.".green().bold()
            );
            return;
        }

        let errors = result
            .issues
            .iter()
            .filter(|i| matches!(i.level, ValidationLevel::Error))
            .count();

        let warnings = result
            .issues
            .iter()
            .filter(|i| matches!(i.level, ValidationLevel::Warning))
            .count();

        let infos = result
            .issues
            .iter()
            .filter(|i| matches!(i.level, ValidationLevel::Info))
            .count();

        println!(
            "Validation complete: {} errors, {} warnings, {} info items\n",
            errors.to_string().red().bold(),
            warnings.to_string().yellow().bold(),
            infos.to_string().blue().bold()
        );

        // Group issues by semester
        let mut by_semester: std::collections::HashMap<
            Option<usize>,
            Vec<&crate::validation::ValidationIssue>,
        > = std::collections::HashMap::new();

        for issue in &result.issues {
            by_semester
                .entry(issue.semester_index)
                .or_default()
                .push(issue);
        }

        // Display general issues (no specific semester)
        if let Some(issues) = by_semester.get(&None) {
            println!("{}", "General Issues:".bold());
            for issue in issues {
                let prefix = match issue.level {
                    ValidationLevel::Error => "❌ ERROR:".red().bold(),
                    ValidationLevel::Warning => "⚠️ WARNING:".yellow().bold(),
                    ValidationLevel::Info => "ℹ️ INFO:".blue().bold(),
                };

                let module_info = if let Some(module_code) = &issue.module_code {
                    format!(" ({})", module_code)
                } else {
                    String::new()
                };

                println!("  {} {}{}", prefix, issue.message, module_info);
            }
            println!();
        }

        for semester_idx in 0..plan.semesters.len() {
            if let Some(issues) = by_semester.get(&Some(semester_idx)) {
                if !issues.is_empty() {
                    let semester = &plan.semesters[semester_idx];
                    println!(
                        "{} {} Semester {}:",
                        "Issues for".bold(),
                        semester.year,
                        semester.semester
                    );

                    for issue in issues {
                        let prefix = match issue.level {
                            ValidationLevel::Error => "❌ ERROR:".red().bold(),
                            ValidationLevel::Warning => "⚠️ WARNING:".yellow().bold(),
                            ValidationLevel::Info => "ℹ️ INFO:".blue().bold(),
                        };

                        let module_info = if let Some(module_code) = &issue.module_code {
                            format!(" ({})", module_code)
                        } else {
                            String::new()
                        };

                        println!("  {} {}{}", prefix, issue.message, module_info);
                    }
                    println!();
                }
            }
        }

        if errors > 0 {
            println!(
                "{}",
                "This plan has critical issues that need to be resolved."
                    .red()
                    .bold()
            );
        } else if warnings > 0 {
            println!(
                "{}",
                "This plan has some potential issues to review.".yellow()
            );
        } else {
            println!(
                "{}",
                "This plan is valid but has some informational notes.".green()
            );
        }
    }
}
