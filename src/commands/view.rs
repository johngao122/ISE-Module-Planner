use crate::api::NusmodsApi;
use crate::models::{ModuleRegistry, ModuleStatus, Plan};
use crate::storage::Storage;
use anyhow::{anyhow, Result};
use colored::Colorize;
use dialoguer::Select;

pub struct ViewCommand {
    pub storage: Box<dyn Storage>,
    pub api: Box<dyn NusmodsApi>,
    pub plan_id: Option<String>,
}

impl super::Command for ViewCommand {
    fn run(&self) -> Result<()> {
        let plan = if let Some(id) = &self.plan_id {
            self.storage
                .get_plan(id)?
                .ok_or_else(|| anyhow!("Plan not found"))?
        } else {
            let plans = self.storage.list_plans()?;

            if plans.is_empty() {
                println!("No plans found. Create a plan first.");
                return Ok(());
            }

            let plan_options: Vec<String> = plans
                .iter()
                .map(|p| format!("{} (Target: {})", p.name, p.target_graduation))
                .collect();

            let plan_index = Select::new()
                .with_prompt("Select a plan to view")
                .items(&plan_options)
                .default(0)
                .interact()?;

            plans[plan_index].clone()
        };

        let view_modes = vec![
            "Summary View",
            "Detailed View",
            "Curriculum Progress View",
            "Module List View",
        ];

        let mode_index = Select::new()
            .with_prompt("Select view mode")
            .items(&view_modes)
            .default(0)
            .interact()?;

        match mode_index {
            0 => self.display_summary_view(&plan)?,
            1 => self.display_detailed_view(&plan)?,
            2 => self.display_curriculum_view(&plan)?,
            3 => self.display_module_list_view(&plan)?,
            _ => unreachable!(),
        }

        Ok(())
    }
}

impl ViewCommand {
    fn display_summary_view(&self, plan: &Plan) -> Result<()> {
        println!("\n{}", "📚 ACADEMIC PLAN SUMMARY".bold());
        println!("{}:{} {}", "Plan Name".bold(), " ".repeat(10), plan.name);
        println!(
            "{}:  {}",
            "Target Graduation".bold(),
            plan.target_graduation
        );
        println!(
            "{}:  {} units",
            "Total Planned Units".bold(),
            plan.total_units()
        );

        let student = self
            .storage
            .get_student(&plan.student_id)?
            .ok_or_else(|| anyhow!("Student profile not found"))?;

        println!("{}:{} {}", "Student".bold(), " ".repeat(12), student.name);
        println!("{}:{} {}", "Major".bold(), " ".repeat(14), student.major);

        if let Some(ref second_major) = student.second_major {
            println!(
                "{}:{} {}",
                "Second Major".bold(),
                " ".repeat(7),
                second_major
            );
        }

        if !student.minors.is_empty() {
            println!(
                "{}:{} {}",
                "Minors".bold(),
                " ".repeat(13),
                student.minors.join(", ")
            );
        }

        println!("\n{}", "Semester Overview:".bold());

        let mut most_units_sem = (0, 0);
        let mut total_modules = 0;

        for (i, semester) in plan.semesters.iter().enumerate() {
            if semester.total_units > most_units_sem.1 {
                most_units_sem = (i, semester.total_units);
            }

            total_modules += semester.modules.len();

            println!(
                "  {} {} Semester {}: {} units, {} modules{}",
                "•".bold(),
                semester.year,
                semester.semester,
                semester.total_units,
                semester.modules.len(),
                if !semester.special_activities.is_empty() {
                    format!(
                        " (+ {} special activities)",
                        semester.special_activities.len()
                    )
                } else {
                    String::new()
                }
            );
        }

        println!("\n{}", "Statistics:".bold());
        println!("  Total modules planned: {}", total_modules);
        println!(
            "  Average units per semester: {:.1}",
            if plan.semesters.is_empty() {
                0.0
            } else {
                plan.total_units() as f64 / plan.semesters.len() as f64
            }
        );
        println!(
            "  Heaviest semester: {} Semester {} ({} units)",
            plan.semesters[most_units_sem.0].year,
            plan.semesters[most_units_sem.0].semester,
            most_units_sem.1
        );

        if let Some(ref notes) = plan.notes {
            println!("\n{}", "Notes:".bold());
            println!("  {}", notes);
        }

        Ok(())
    }

    fn display_detailed_view(&self, plan: &Plan) -> Result<()> {
        println!("\n{}", "📚 ACADEMIC PLAN: DETAILED VIEW".bold());
        println!("{}:{} {}", "Plan Name".bold(), " ".repeat(10), plan.name);
        println!(
            "{}:  {}",
            "Target Graduation".bold(),
            plan.target_graduation
        );

        let registry = self.api.get_module_registry()?;

        for (i, semester) in plan.semesters.iter().enumerate() {
            println!(
                "\n{} {} {}",
                format!("[{}]", i + 1).bold(),
                format!("{} Semester {}", semester.year, semester.semester)
                    .bold()
                    .underline(),
                format!("({} units)", semester.total_units).bold()
            );

            if semester.modules.is_empty() {
                println!("  No modules planned for this semester");
            } else {
                // Group modules by type/level for better organization
                let mut level_1000 = Vec::new();
                let mut level_2000 = Vec::new();
                let mut level_3000 = Vec::new();
                let mut level_4000 = Vec::new();
                let mut level_5000 = Vec::new();
                let mut other = Vec::new();

                for module in &semester.modules {
                    let code = &module.module_code;
                    if code.contains("1") && code.len() >= 6 {
                        level_1000.push(module);
                    } else if code.contains("2") && code.len() >= 6 {
                        level_2000.push(module);
                    } else if code.contains("3") && code.len() >= 6 {
                        level_3000.push(module);
                    } else if code.contains("4") && code.len() >= 6 {
                        level_4000.push(module);
                    } else if code.contains("5") && code.len() >= 6 {
                        level_5000.push(module);
                    } else {
                        other.push(module);
                    }
                }

                // Helper function to print module group
                let print_module_group =
                    |level: &str,
                     modules: &[&crate::models::PlannedModule],
                     registry: &ModuleRegistry| {
                        if !modules.is_empty() {
                            println!("  {} Modules:", level.bold());
                            for module in modules {
                                let title = registry
                                    .get_module(&module.module_code)
                                    .map(|m| m.title.clone())
                                    .unwrap_or_else(|| "Unknown Module".to_string());

                                let credits = registry
                                    .get_module(&module.module_code)
                                    .map(|m| m.module_credit.clone())
                                    .unwrap_or_else(|| "?".to_string());

                                let status = match module.status {
                                    ModuleStatus::Planned => "📝 Planned".normal(),
                                    ModuleStatus::Current => "🔄 Current".blue(),
                                    ModuleStatus::Completed => "✅ Completed".green(),
                                    ModuleStatus::Failed => "❌ Failed".red(),
                                };

                                let su_info = if module.s_u_option {
                                    " (S/U option)".dimmed()
                                } else {
                                    "".normal()
                                };

                                println!(
                                    "    {} {} - {} ({} units) {} {}",
                                    "•".bold(),
                                    module.module_code.bold(),
                                    title,
                                    credits,
                                    status,
                                    su_info
                                );
                            }
                        }
                    };

                // Print each module group
                print_module_group("Level 1000", &level_1000, &registry);
                print_module_group("Level 2000", &level_2000, &registry);
                print_module_group("Level 3000", &level_3000, &registry);
                print_module_group("Level 4000", &level_4000, &registry);
                print_module_group("Level 5000", &level_5000, &registry);
                print_module_group("Other", &other, &registry);
            }

            // Print special activities
            if !semester.special_activities.is_empty() {
                println!("\n  {} Special Activities:", "📋".bold());
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

                    println!(
                        "    {} {}: {}{}",
                        "•".bold(),
                        activity_type.bold(),
                        activity.description,
                        credits_info
                    );
                }
            }
        }

        Ok(())
    }

    fn display_curriculum_view(&self, plan: &Plan) -> Result<()> {
        let student = self
            .storage
            .get_student(&plan.student_id)?
            .ok_or_else(|| anyhow!("Student profile not found"))?;

        let curriculum = self
            .storage
            .get_curriculum(&student.major)?
            .ok_or_else(|| anyhow!("Curriculum not found"))?;

        println!("\n{}", "📚 CURRICULUM PROGRESS".bold());
        println!("{}:{} {}", "Plan Name".bold(), " ".repeat(10), plan.name);
        println!("{}:{} {}", "Major".bold(), " ".repeat(14), student.major);
        println!(
            "{}:{} {}",
            "Curriculum".bold(),
            " ".repeat(9),
            curriculum.name
        );

        let all_modules: std::collections::HashSet<String> = plan.all_modules();

        println!("\n{}", "Degree Requirements:".bold());
        println!(
            "Total units required: {}/{}",
            plan.total_units(),
            curriculum.total_units_required
        );

        if plan.total_units() >= curriculum.total_units_required {
            println!("  {} Unit requirement met", "✅".green());
        } else {
            println!(
                "  {} Need {} more units",
                "❌".red(),
                curriculum.total_units_required - plan.total_units()
            );
        }

        let level_1000_units = self.calculate_level_units(plan, 1000)?;
        println!(
            "Level 1000 units: {}/{} maximum",
            level_1000_units, curriculum.max_level1000_units
        );

        if level_1000_units <= curriculum.max_level1000_units {
            println!("  {} Within Level 1000 limit", "✅".green());
        } else {
            println!(
                "  {} Exceeded Level 1000 limit by {} units",
                "❌".red(),
                level_1000_units - curriculum.max_level1000_units
            );
        }

        for component in &curriculum.components {
            println!(
                "\n{} ({}/{} units):",
                component.name.bold(),
                self.calculate_component_units(plan, component)?,
                component.min_units
            );

            for requirement in &component.requirements {
                match requirement {
                    crate::models::Requirement::FixedModule {
                        module_code,
                        name,
                        units,
                    } => {
                        let fulfilled = all_modules.contains(module_code);
                        let status = if fulfilled {
                            "✅".green()
                        } else {
                            "⬜".normal()
                        };

                        println!(
                            "  {} {} {} ({} units)",
                            status,
                            module_code.bold(),
                            name,
                            units
                        );
                    }
                    crate::models::Requirement::ModuleGroup {
                        name,
                        description,
                        min_units,
                        possible_modules,
                    } => {
                        let fulfilled_modules: Vec<&String> = possible_modules
                            .iter()
                            .filter(|m| plan.all_modules().contains(*m))
                            .collect();

                        let _fulfilled_count = fulfilled_modules.len();
                        let fulfilled_units =
                            self.calculate_group_units(fulfilled_modules.clone())?;

                        let status = if fulfilled_units >= *min_units {
                            "✅".green()
                        } else {
                            "⬜".normal()
                        };

                        println!(
                            "  {} {} - {}/{} units",
                            status,
                            name.bold(),
                            fulfilled_units,
                            min_units
                        );

                        if let Some(desc) = description {
                            println!("     {}", desc.dimmed());
                        }

                        if !fulfilled_modules.is_empty() {
                            println!("     Modules selected:");
                            for module_code in fulfilled_modules {
                                println!("       • {}", module_code);
                            }
                        }
                    }
                    crate::models::Requirement::Elective {
                        name,
                        description,
                        min_units,
                        ..
                    } => {
                        println!(
                            "  {} {} - Need {} units",
                            "ℹ️".blue(),
                            name.bold(),
                            min_units
                        );
                        println!("     {}", description.dimmed());
                    }
                }
            }
        }

        Ok(())
    }

    fn display_module_list_view(&self, plan: &Plan) -> Result<()> {
        println!("\n{}", "📚 MODULE LIST VIEW".bold());
        println!("{}:{} {}", "Plan Name".bold(), " ".repeat(10), plan.name);

        let registry = self.api.get_module_registry()?;

        let all_module_codes = plan.all_modules();

        let mut modules = Vec::new();
        for code in all_module_codes {
            if let Some(module) = registry.get_module(&code) {
                modules.push(module);
            }
        }

        modules.sort_by(|a, b| a.module_code.cmp(&b.module_code));

        println!("\nTotal modules: {}", modules.len());

        println!(
            "\n{:<10} {:<30} {:<10} {:<20}",
            "Code".bold(),
            "Title".bold(),
            "Credits".bold(),
            "Department".bold()
        );
        println!("{}", "-".repeat(70));

        for module in modules {
            println!(
                "{:<10} {:<30} {:<10} {:<20}",
                module.module_code,
                truncate_string(&module.title, 28),
                module.module_credit,
                module.department.as_deref().unwrap_or("")
            );
        }

        Ok(())
    }

    pub fn calculate_level_units(&self, plan: &Plan, level: u32) -> Result<u32> {
        let level_digit = (level / 1000).to_string();
        let mut total = 0;

        for semester in &plan.semesters {
            for module in &semester.modules {
                // Find the first digit in the module code
                if let Some(pos) = module.module_code.chars().position(|c| c.is_ascii_digit()) {
                    // Get the numeric part (e.g., "1111" from "IE1111R")
                    let numeric_part: String = module.module_code[pos..]
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect();

                    // Check if it's a module of the specified level
                    if numeric_part.starts_with(&level_digit) {
                        total += 4; // Each module is 4 units
                    }
                }
            }
        }

        Ok(total)
    }

    pub fn calculate_component_units(
        &self,
        plan: &Plan,
        component: &crate::models::CurriculumComponent,
    ) -> Result<u32> {
        let mut total = 0;

        for requirement in &component.requirements {
            match requirement {
                crate::models::Requirement::FixedModule {
                    module_code, units, ..
                } => {
                    if plan.all_modules().contains(module_code) {
                        total += units;
                    }
                }
                crate::models::Requirement::ModuleGroup {
                    possible_modules, ..
                } => {
                    let fulfilled_modules: Vec<&String> = possible_modules
                        .iter()
                        .filter(|m| plan.all_modules().contains(*m))
                        .collect();

                    total += self.calculate_group_units(fulfilled_modules.clone())?;
                }
                _ => {}
            }
        }

        Ok(total)
    }

    pub fn calculate_group_units(&self, modules: Vec<&String>) -> Result<u32> {
        let registry = self.api.get_module_registry()?;
        let mut total = 0;

        for &module_code in &modules {
            if let Some(module) = registry.get_module(module_code) {
                if let Ok(credits) = module.module_credit.parse::<u32>() {
                    total += credits;
                }
            }
        }

        Ok(total)
    }
}

// Helper function to truncate long strings
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len - 3])
    }
}
