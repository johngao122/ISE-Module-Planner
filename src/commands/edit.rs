use crate::api::NusmodsApi;
use crate::models::{ModuleRegistry, ModuleStatus, Plan, PlannedModule};
use crate::storage::Storage;
use anyhow::{anyhow, Result};
use dialoguer::{Confirm, Input, MultiSelect, Select};

pub struct EditCommand {
    pub storage: Box<dyn Storage>,
    pub api: Box<dyn NusmodsApi>,
    pub plan_id: String,
}

impl super::Command for EditCommand {
    fn run(&self) -> Result<()> {
        let mut plan = self
            .storage
            .get_plan(&self.plan_id)?
            .ok_or_else(|| anyhow!("Plan not found"))?;

        let registry = self.api.get_module_registry()?;

        self.display_plan_summary(&plan);

        loop {
            let options = vec![
                "Add modules to a semester",
                "Remove modules from a semester",
                "Move modules between semesters",
                "Add special activities (e.g., internship)",
                "Edit plan metadata",
                "Save and exit",
            ];

            let selection = Select::new()
                .with_prompt("Choose an action")
                .items(&options)
                .default(0)
                .interact()?;

            match selection {
                0 => self.add_modules(&mut plan, &registry)?,
                1 => self.remove_modules(&mut plan)?,
                2 => self.move_modules(&mut plan)?,
                3 => self.add_special_activities(&mut plan)?,
                4 => self.edit_metadata(&mut plan)?,
                5 => break,
                _ => unreachable!(),
            }
        }

        self.storage.save_plan(&plan)?;

        println!("✅ Plan updated successfully");

        Ok(())
    }
}

impl EditCommand {
    fn display_plan_summary(&self, plan: &Plan) {
        println!(
            "\n📝 PLAN: {} (Target graduation: {})",
            plan.name, plan.target_graduation
        );

        for (i, semester) in plan.semesters.iter().enumerate() {
            println!(
                "\n[{}] {} Semester {}: {} units",
                i + 1,
                semester.year,
                semester.semester,
                semester.total_units
            );

            if semester.modules.is_empty() {
                println!("  (No modules planned)");
            } else {
                for module in &semester.modules {
                    println!(
                        "  - {} ({})",
                        module.module_code,
                        match module.status {
                            ModuleStatus::Planned => "Planned",
                            ModuleStatus::Current => "Current",
                            ModuleStatus::Completed => "Completed",
                            ModuleStatus::Failed => "Failed",
                        }
                    );
                }
            }

            if !semester.special_activities.is_empty() {
                println!("  Special Activities:");
                for activity in &semester.special_activities {
                    println!(
                        "  - {} ({})",
                        activity.description,
                        match activity.activity_type {
                            crate::models::ActivityType::IndustrialAttachment =>
                                "Industrial Attachment",
                            crate::models::ActivityType::InternationalExchange =>
                                "International Exchange",
                            crate::models::ActivityType::Research => "Research",
                            crate::models::ActivityType::CommunityService => "Community Service",
                            crate::models::ActivityType::Other(ref s) => s,
                        }
                    );
                }
            }
        }
        println!("\nTotal units planned: {}", plan.total_units());
    }

    fn add_modules(&self, plan: &mut Plan, registry: &ModuleRegistry) -> Result<()> {
        let semester_options: Vec<String> = plan
            .semesters
            .iter()
            .enumerate()
            .map(|(i, s)| format!("[{}] {} Semester {}", i + 1, s.year, s.semester))
            .collect();

        let sem_index = Select::new()
            .with_prompt("Select a semester to add modules to")
            .items(&semester_options)
            .default(0)
            .interact()?;

        let module_code: String = Input::new()
            .with_prompt("Enter module code (or partial code to search)")
            .interact()?;

        if module_code.len() < 6 {
            let matching_modules = self.api.search_modules(&module_code)?;

            if matching_modules.is_empty() {
                println!("No modules found matching '{}'", module_code);
                return Ok(());
            }

            let module_options: Vec<String> = matching_modules
                .iter()
                .map(|m| format!("{}: {}", m.module_code, m.title))
                .collect();

            let module_index = Select::new()
                .with_prompt("Select a module")
                .items(&module_options)
                .default(0)
                .interact()?;

            let selected_module = &matching_modules[module_index];

            let is_available =
                self.check_module_availability(selected_module, plan.semesters[sem_index].semester);

            if !is_available {
                let proceed = Confirm::new()
                    .with_prompt(format!(
                        "⚠️ {} is not typically offered in Semester {}. Add anyway?",
                        selected_module.module_code, plan.semesters[sem_index].semester
                    ))
                    .default(false)
                    .interact()?;

                if !proceed {
                    return Ok(());
                }
            }

            let planned_module = PlannedModule {
                module_code: selected_module.module_code.clone(),
                status: ModuleStatus::Planned,
                grade: None,
                s_u_option: false,
            };

            plan.semesters[sem_index].modules.push(planned_module);

            if let Some(credits_str) = selected_module.module_credit.parse::<u32>().ok() {
                plan.semesters[sem_index].total_units += credits_str;
            }

            println!(
                "✅ Added {} to {} Semester {}",
                selected_module.module_code,
                plan.semesters[sem_index].year,
                plan.semesters[sem_index].semester
            );
        } else {
            if let Some(module) = registry.get_module(&module_code) {
                let is_available =
                    self.check_module_availability(module, plan.semesters[sem_index].semester);

                if !is_available {
                    let proceed = Confirm::new()
                        .with_prompt(format!(
                            "⚠️ {} is not typically offered in Semester {}. Add anyway?",
                            module.module_code, plan.semesters[sem_index].semester
                        ))
                        .default(false)
                        .interact()?;

                    if !proceed {
                        return Ok(());
                    }
                }

                let planned_module = PlannedModule {
                    module_code: module.module_code.clone(),
                    status: ModuleStatus::Planned,
                    grade: None,
                    s_u_option: false,
                };

                plan.semesters[sem_index].modules.push(planned_module);

                if let Some(credits) = module.module_credit.parse::<u32>().ok() {
                    plan.semesters[sem_index].total_units += credits;
                }

                println!(
                    "✅ Added {} to {} Semester {}",
                    module.module_code,
                    plan.semesters[sem_index].year,
                    plan.semesters[sem_index].semester
                );
            } else {
                println!("⚠️ Module '{}' not found in registry", module_code);
            }
        }

        Ok(())
    }

    fn remove_modules(&self, plan: &mut Plan) -> Result<()> {
        let semester_options: Vec<String> = plan
            .semesters
            .iter()
            .enumerate()
            .map(|(i, s)| format!("[{}] {} Semester {}", i + 1, s.year, s.semester))
            .collect();

        let sem_index = Select::new()
            .with_prompt("Select a semester to remove modules from")
            .items(&semester_options)
            .default(0)
            .interact()?;

        let semester = &plan.semesters[sem_index];

        if semester.modules.is_empty() {
            println!("No modules to remove in this semester");
            return Ok(());
        }

        let module_options: Vec<String> = semester
            .modules
            .iter()
            .map(|m| {
                format!(
                    "{} ({})",
                    m.module_code,
                    match m.status {
                        ModuleStatus::Planned => "Planned",
                        ModuleStatus::Current => "Current",
                        ModuleStatus::Completed => "Completed",
                        ModuleStatus::Failed => "Failed",
                    }
                )
            })
            .collect();

        let selected_indices = MultiSelect::new()
            .with_prompt("Select modules to remove")
            .items(&module_options)
            .interact()?;

        if selected_indices.is_empty() {
            return Ok(());
        }

        let registry = self.api.get_module_registry()?;

        let mut indices_to_remove: Vec<usize> = selected_indices;
        indices_to_remove.sort_unstable();
        indices_to_remove.reverse();

        for index in indices_to_remove {
            let module_code = plan.semesters[sem_index].modules[index].module_code.clone();

            if let Some(module) = registry.get_module(&module_code) {
                if let Some(credits) = module.module_credit.parse::<u32>().ok() {
                    plan.semesters[sem_index].total_units = plan.semesters[sem_index]
                        .total_units
                        .saturating_sub(credits);
                }
            }

            plan.semesters[sem_index].modules.remove(index);
            println!("Removed {}", module_code);
        }

        Ok(())
    }

    fn move_modules(&self, plan: &mut Plan) -> Result<()> {
        let semester_options: Vec<String> = plan
            .semesters
            .iter()
            .enumerate()
            .map(|(i, s)| format!("[{}] {} Semester {}", i + 1, s.year, s.semester))
            .collect();

        let source_index = Select::new()
            .with_prompt("Select source semester")
            .items(&semester_options)
            .default(0)
            .interact()?;

        let source_semester = &plan.semesters[source_index];

        if source_semester.modules.is_empty() {
            println!("No modules to move in this semester");
            return Ok(());
        }

        let module_options: Vec<String> = source_semester
            .modules
            .iter()
            .map(|m| m.module_code.clone())
            .collect();

        let selected_indices = MultiSelect::new()
            .with_prompt("Select modules to move")
            .items(&module_options)
            .interact()?;

        if selected_indices.is_empty() {
            return Ok(());
        }

        let target_index = Select::new()
            .with_prompt("Select target semester")
            .items(&semester_options)
            .default(0)
            .interact()?;

        if source_index == target_index {
            println!("Source and target semesters are the same");
            return Ok(());
        }

        let mut modules_to_move = Vec::new();
        let registry = self.api.get_module_registry()?;

        for &index in &selected_indices {
            let module = plan.semesters[source_index].modules[index].clone();
            modules_to_move.push(module);

            if let Some(registry_module) =
                registry.get_module(&plan.semesters[source_index].modules[index].module_code)
            {
                if let Some(credits) = registry_module.module_credit.parse::<u32>().ok() {
                    plan.semesters[source_index].total_units = plan.semesters[source_index]
                        .total_units
                        .saturating_sub(credits);
                }
            }
        }

        let mut indices_to_remove = selected_indices.clone();
        indices_to_remove.sort_unstable();
        indices_to_remove.reverse();

        for index in indices_to_remove {
            plan.semesters[source_index].modules.remove(index);
        }

        for module in modules_to_move {
            if let Some(registry_module) = registry.get_module(&module.module_code) {
                if let Some(credits) = registry_module.module_credit.parse::<u32>().ok() {
                    plan.semesters[target_index].total_units += credits;
                }
            }

            plan.semesters[target_index].modules.push(module.clone());
            println!(
                "Moved {} to {} Semester {}",
                module.module_code,
                plan.semesters[target_index].year,
                plan.semesters[target_index].semester
            );
        }

        Ok(())
    }

    fn add_special_activities(&self, plan: &mut Plan) -> Result<()> {
        let semester_options: Vec<String> = plan
            .semesters
            .iter()
            .enumerate()
            .map(|(i, s)| format!("[{}] {} Semester {}", i + 1, s.year, s.semester))
            .collect();

        let sem_index = Select::new()
            .with_prompt("Select a semester to add special activity to")
            .items(&semester_options)
            .default(0)
            .interact()?;

        let activity_types = [
            "Industrial Attachment",
            "International Exchange",
            "Research",
            "Community Service",
            "Other",
        ];

        let type_index = Select::new()
            .with_prompt("Select activity type")
            .items(&activity_types)
            .default(0)
            .interact()?;

        let description: String = Input::new()
            .with_prompt("Enter activity description")
            .interact()?;

        let credits_str: String = Input::new()
            .with_prompt("Enter credits (or leave empty for none)")
            .allow_empty(true)
            .interact()?;

        let credits = if credits_str.is_empty() {
            None
        } else {
            match credits_str.parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => {
                    println!("⚠️ Invalid number format, no credits assigned");
                    None
                }
            }
        };

        let activity_type = match type_index {
            0 => crate::models::ActivityType::IndustrialAttachment,
            1 => crate::models::ActivityType::InternationalExchange,
            2 => crate::models::ActivityType::Research,
            3 => crate::models::ActivityType::CommunityService,
            4 => {
                let name: String = Input::new()
                    .with_prompt("Enter custom activity type")
                    .interact()?;
                crate::models::ActivityType::Other(name)
            }
            _ => unreachable!(),
        };

        let activity = crate::models::SpecialActivity {
            activity_type,
            description,
            credits,
        };

        plan.semesters[sem_index].special_activities.push(activity);

        if let Some(credit_val) = credits {
            plan.semesters[sem_index].total_units += credit_val;
        }

        println!(
            "✅ Added special activity to {} Semester {}",
            plan.semesters[sem_index].year, plan.semesters[sem_index].semester
        );

        Ok(())
    }

    fn edit_metadata(&self, plan: &mut Plan) -> Result<()> {
        let options = vec![
            "Edit plan name",
            "Edit target graduation",
            "Edit notes",
            "Back to main menu",
        ];

        let selection = Select::new()
            .with_prompt("Choose what to edit")
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                let new_name: String = Input::new()
                    .with_prompt("Enter new plan name")
                    .with_initial_text(&plan.name)
                    .interact()?;

                plan.name = new_name;
                println!("✅ Plan name updated");
            }
            1 => {
                let years = ["2025/2026", "2026/2027", "2027/2028", "2028/2029"];
                let year_index = Select::new()
                    .with_prompt("Select target graduation academic year")
                    .items(&years)
                    .default(0)
                    .interact()?;

                let semesters = ["Semester 1", "Semester 2"];
                let sem_index = Select::new()
                    .with_prompt("Select target graduation semester")
                    .items(&semesters)
                    .default(1)
                    .interact()?;

                plan.target_graduation = format!("{} {}", years[year_index], semesters[sem_index]);
                println!("✅ Target graduation updated");
            }
            2 => {
                let current_notes = plan.notes.as_deref().unwrap_or("");
                let new_notes: String = Input::new()
                    .with_prompt("Enter plan notes")
                    .with_initial_text(current_notes)
                    .allow_empty(true)
                    .interact()?;

                plan.notes = if new_notes.is_empty() {
                    None
                } else {
                    Some(new_notes)
                };
                println!("✅ Notes updated");
            }
            3 => return Ok(()),
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn check_module_availability(&self, module: &crate::models::Module, semester: u8) -> bool {
        module
            .semester_data
            .iter()
            .any(|sem_data| sem_data.semester == semester)
    }
}
