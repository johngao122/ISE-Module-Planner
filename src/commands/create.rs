use crate::api::NusmodsApi;
use crate::models::{Plan, Student};
use crate::storage::Storage;
use anyhow::{anyhow, Result};
use dialoguer::{Input, Select};
use uuid::Uuid;

pub struct CreateCommand {
    pub storage: Box<dyn Storage>,
    pub api: Box<dyn NusmodsApi>,
}

impl super::Command for CreateCommand {
    fn run(&self) -> Result<()> {
        println!("Creating a new academic plan");

        let student = self.get_or_create_student()?;

        let plan_name: String = Input::new()
            .with_prompt("Enter a name for your academic plan")
            .interact()?;

        let plan_id = Uuid::new_v4().to_string();

        let mut plan = Plan::new(plan_id, plan_name, student.name.clone());

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

        self.initialize_plan_semesters(&mut plan, &student)?;

        self.storage.save_plan(&plan)?;

        println!("✅ Created new academic plan: {}", plan.name);
        println!("Plan ID: {}", plan.id);

        Ok(())
    }
}

impl CreateCommand {
    fn get_or_create_student(&self) -> Result<Student> {
        let students = self.storage.list_students()?;

        if students.is_empty() {
            self.create_student_profile()
        } else if students.len() == 1 {
            Ok(students[0].clone())
        } else {
            let names: Vec<String> = students.iter().map(|s| s.name.clone()).collect();
            let selection = Select::new()
                .with_prompt("Select a student profile")
                .items(&names)
                .interact()?;

            Ok(students[selection].clone())
        }
    }

    fn create_student_profile(&self) -> Result<Student> {
        println!("Creating a new student profile");

        let name: String = Input::new().with_prompt("Enter your name").interact()?;

        let matriculation_years = ["2025/2026", "2024/2025", "2023/2024", "2022/2023"];
        let year_index = Select::new()
            .with_prompt("Select matriculation academic year")
            .items(&matriculation_years)
            .default(0)
            .interact()?;

        let faculties = [
            "College of Design and Engineering",
            "Computing",
            "Science",
            "Arts and Social Sciences",
            "Business",
        ];
        let faculty_index = Select::new()
            .with_prompt("Select your faculty")
            .items(&faculties)
            .default(0)
            .interact()?;

        let major = if faculty_index == 0 {
            let majors = [
                "Industrial & Systems Engineering",
                "Computer Engineering",
                "Electrical Engineering",
                "Mechanical Engineering",
            ];
            let major_index = Select::new()
                .with_prompt("Select your major")
                .items(&majors)
                .default(0)
                .interact()?;

            majors[major_index].to_string()
        } else {
            Input::new().with_prompt("Enter your major").interact()?
        };

        let current_sem: u8 = Input::new()
            .with_prompt("Enter your current semester (1-8)")
            .validate_with(|input: &u8| {
                if *input >= 1 && *input <= 8 {
                    Ok(())
                } else {
                    Err("Please enter a value between 1 and 8")
                }
            })
            .interact()?;

        let student = Student {
            name,
            matriculation_year: matriculation_years[year_index].to_string(),
            faculty: faculties[faculty_index].to_string(),
            major,
            second_major: None,
            minors: Vec::new(),
            completed_modules: std::collections::HashSet::new(),
            exempted_modules: std::collections::HashSet::new(),
            advanced_placement_credits: 0,
            current_semester: current_sem,
            candidature_type: crate::models::CandidatureType::Standard,
        };

        self.storage.save_student(&student)?;

        println!("✅ Created new student profile");

        Ok(student)
    }

    fn initialize_plan_semesters(&self, plan: &mut Plan, student: &Student) -> Result<()> {
        let remaining_semesters = student.remaining_semesters();

        let mat_year_parts: Vec<&str> = student.matriculation_year.split('/').collect();
        if mat_year_parts.len() != 2 {
            return Err(anyhow!("Invalid matriculation year format"));
        }

        let start_year = mat_year_parts[0].parse::<u32>()?;
        let mut current_year = start_year + u32::from(student.current_semester - 1) / 2;
        let mut current_sem = student.current_semester % 2;
        if current_sem == 0 {
            current_sem = 2;
        }

        for _ in 0..remaining_semesters {
            let year_str = format!("{}/{}", current_year, current_year + 1);

            let semester_plan = crate::models::SemesterPlan {
                year: year_str,
                semester: current_sem,
                modules: Vec::new(),
                total_units: 0,
                special_activities: Vec::new(),
            };

            plan.semesters.push(semester_plan);

            if current_sem == 2 {
                current_sem = 1;
                current_year += 1;
            } else {
                current_sem += 1;
            }
        }

        Ok(())
    }
}
