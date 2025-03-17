use crate::api::{DefaultNusmodsApi, NusmodsApi};
use crate::models::Plan;
use crate::storage::Storage;
use anyhow::{anyhow, Result};
use dialoguer::{Input, Select};
use std::fs::File;
use std::io::Write;

pub struct ExportCommand {
    pub storage: Box<dyn Storage>,
    pub plan_id: String,
}

impl super::Command for ExportCommand {
    fn run(&self) -> Result<()> {
        let plan = self
            .storage
            .get_plan(&self.plan_id)?
            .ok_or_else(|| anyhow!("Plan not found"))?;

        let formats = vec!["JSON", "CSV", "Markdown", "Plain Text"];
        let format_index = Select::new()
            .with_prompt("Select export format")
            .items(&formats)
            .default(0)
            .interact()?;

        let default_filename = format!("{}_plan", plan.name.replace(" ", "_").to_lowercase());
        let filename: String = Input::new()
            .with_prompt("Enter filename (without extension)")
            .with_initial_text(&default_filename)
            .interact()?;

        match format_index {
            0 => self.export_json(&plan, &filename)?,
            1 => self.export_csv(&plan, &filename)?,
            2 => self.export_markdown(&plan, &filename)?,
            3 => self.export_text(&plan, &filename)?,
            _ => unreachable!(),
        }

        Ok(())
    }
}

impl ExportCommand {
    pub fn export_json(&self, plan: &Plan, filename: &str) -> Result<()> {
        let path = format!("{}.json", filename);
        let json = serde_json::to_string_pretty(plan)?;

        let mut file = File::create(&path)?;
        file.write_all(json.as_bytes())?;

        println!("✅ Exported plan to {}", path);
        Ok(())
    }

    pub fn export_csv(&self, plan: &Plan, filename: &str) -> Result<()> {
        let path = format!("{}.csv", filename);
        let mut file = File::create(&path)?;

        writeln!(
            file,
            "Semester,Year,Module Code,Module Title,Credits,Status"
        )?;

        let api = crate::api::DefaultNusmodsApi::new()?;
        let registry = api.get_module_registry()?;

        for semester in &plan.semesters {
            for module in &semester.modules {
                let title = registry
                    .get_module(&module.module_code)
                    .map(|m| m.title.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                let credits = registry
                    .get_module(&module.module_code)
                    .map(|m| m.module_credit.clone())
                    .unwrap_or_else(|| "0".to_string());

                let status = match module.status {
                    crate::models::ModuleStatus::Planned => "Planned",
                    crate::models::ModuleStatus::Current => "Current",
                    crate::models::ModuleStatus::Completed => "Completed",
                    crate::models::ModuleStatus::Failed => "Failed",
                };

                writeln!(
                    file,
                    "{},{},{},{},{},{}",
                    semester.semester, semester.year, module.module_code, title, credits, status
                )?;
            }
        }

        println!("✅ Exported plan to {}", path);
        Ok(())
    }

    pub fn export_markdown(&self, plan: &Plan, filename: &str) -> Result<()> {
        let path = format!("{}.md", filename);
        let mut file = File::create(&path)?;

        let api = crate::api::DefaultNusmodsApi::new()?;
        let registry = api.get_module_registry()?;

        writeln!(file, "# Academic Plan: {}", plan.name)?;
        writeln!(file, "\nTarget Graduation: {}", plan.target_graduation)?;
        writeln!(file, "Total Units: {}", plan.total_units())?;

        if let Some(ref notes) = plan.notes {
            writeln!(file, "\n## Notes\n\n{}", notes)?;
        }

        writeln!(file, "\n## Semester Plan\n")?;

        for (i, semester) in plan.semesters.iter().enumerate() {
            writeln!(
                file,
                "### Semester {}: {} Semester {} ({} units)\n",
                i + 1,
                semester.year,
                semester.semester,
                semester.total_units
            )?;

            if !semester.modules.is_empty() {
                writeln!(file, "| Module Code | Title | Credits | Status |")?;
                writeln!(file, "|------------|-------|---------|--------|")?;

                for module in &semester.modules {
                    let title = registry
                        .get_module(&module.module_code)
                        .map(|m| m.title.clone())
                        .unwrap_or_else(|| "Unknown".to_string());

                    let credits = registry
                        .get_module(&module.module_code)
                        .map(|m| m.module_credit.clone())
                        .unwrap_or_else(|| "0".to_string());

                    let status = match module.status {
                        crate::models::ModuleStatus::Planned => "Planned",
                        crate::models::ModuleStatus::Current => "Current",
                        crate::models::ModuleStatus::Completed => "Completed",
                        crate::models::ModuleStatus::Failed => "Failed",
                    };

                    writeln!(
                        file,
                        "| {} | {} | {} | {} |",
                        module.module_code, title, credits, status
                    )?;
                }
            } else {
                writeln!(file, "No modules planned for this semester.\n")?;
            }

            // Write special activities
            if !semester.special_activities.is_empty() {
                writeln!(file, "\n#### Special Activities\n")?;

                for activity in &semester.special_activities {
                    let activity_type = match &activity.activity_type {
                        crate::models::ActivityType::IndustrialAttachment => {
                            "Industrial Attachment"
                        }
                        crate::models::ActivityType::InternationalExchange => {
                            "International Exchange"
                        }
                        crate::models::ActivityType::Research => "Research",
                        crate::models::ActivityType::CommunityService => "Community Service",
                        crate::models::ActivityType::Other(ref s) => s,
                    };

                    let credits_info = if let Some(credits) = activity.credits {
                        format!(" ({} units)", credits)
                    } else {
                        String::new()
                    };

                    writeln!(
                        file,
                        "- **{}**: {}{}",
                        activity_type, activity.description, credits_info
                    )?;
                }
            }

            writeln!(file)?; // Add spacing between semesters
        }

        println!("✅ Exported plan to {}", path);
        Ok(())
    }

    pub fn export_text(&self, plan: &Plan, filename: &str) -> Result<()> {
        let path = format!("{}.txt", filename);
        let mut file = File::create(&path)?;

        let api = crate::api::DefaultNusmodsApi::new()?;
        let registry = api.get_module_registry()?;

        writeln!(file, "ACADEMIC PLAN: {}", plan.name)?;
        writeln!(file, "Target Graduation: {}", plan.target_graduation)?;
        writeln!(file, "Total Units: {}", plan.total_units())?;

        if let Some(ref notes) = plan.notes {
            writeln!(file, "\nNOTES:\n{}", notes)?;
        }

        writeln!(file, "\nSEMESTER PLAN:")?;

        for (i, semester) in plan.semesters.iter().enumerate() {
            writeln!(
                file,
                "\nSemester {}: {} Semester {} ({} units)",
                i + 1,
                semester.year,
                semester.semester,
                semester.total_units
            )?;

            if !semester.modules.is_empty() {
                for module in &semester.modules {
                    let title = registry
                        .get_module(&module.module_code)
                        .map(|m| m.title.clone())
                        .unwrap_or_else(|| "Unknown".to_string());

                    let credits = registry
                        .get_module(&module.module_code)
                        .map(|m| m.module_credit.clone())
                        .unwrap_or_else(|| "0".to_string());

                    let status = match module.status {
                        crate::models::ModuleStatus::Planned => "Planned",
                        crate::models::ModuleStatus::Current => "Current",
                        crate::models::ModuleStatus::Completed => "Completed",
                        crate::models::ModuleStatus::Failed => "Failed",
                    };

                    writeln!(
                        file,
                        "  - {} | {} | {} units | {}",
                        module.module_code, title, credits, status
                    )?;
                }
            } else {
                writeln!(file, "  (No modules planned for this semester)")?;
            }
            if !semester.special_activities.is_empty() {
                writeln!(file, "\n  Special Activities:")?;

                for activity in &semester.special_activities {
                    let activity_type = match &activity.activity_type {
                        crate::models::ActivityType::IndustrialAttachment => {
                            "Industrial Attachment"
                        }
                        crate::models::ActivityType::InternationalExchange => {
                            "International Exchange"
                        }
                        crate::models::ActivityType::Research => "Research",
                        crate::models::ActivityType::CommunityService => "Community Service",
                        crate::models::ActivityType::Other(ref s) => s,
                    };

                    let credits_info = if let Some(credits) = activity.credits {
                        format!(" ({} units)", credits)
                    } else {
                        String::new()
                    };

                    writeln!(
                        file,
                        "  - {}: {}{}",
                        activity_type, activity.description, credits_info
                    )?;
                }
            }
        }

        println!("✅ Exported plan to {}", path);
        Ok(())
    }
}
