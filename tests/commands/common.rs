use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use module_planner::models::{
    ActivityType, CandidatureType, Curriculum, CurriculumComponent, Module, ModuleRegistry,
    ModuleStatus, Plan, PlannedModule, Requirement, SemesterData, SemesterPlan, SpecialActivity,
    Student,
};

// Mock implementation of the Storage trait
pub struct MockStorage {
    plans: Arc<Mutex<HashMap<String, Plan>>>,
    students: Arc<Mutex<HashMap<String, Student>>>,
    curricula: Arc<Mutex<HashMap<String, Curriculum>>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            plans: Arc::new(Mutex::new(HashMap::new())),
            students: Arc::new(Mutex::new(HashMap::new())),
            curricula: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_plan(mut self, plan: Plan) -> Self {
        self.plans.lock().unwrap().insert(plan.id.clone(), plan);
        self
    }

    pub fn with_student(mut self, student: Student) -> Self {
        self.students
            .lock()
            .unwrap()
            .insert(student.name.clone(), student);
        self
    }

    pub fn with_curriculum(mut self, curriculum: Curriculum) -> Self {
        self.curricula
            .lock()
            .unwrap()
            .insert(curriculum.name.clone(), curriculum);
        self
    }
}

impl module_planner::storage::Storage for MockStorage {
    fn save_plan(&self, plan: &Plan) -> Result<()> {
        self.plans
            .lock()
            .unwrap()
            .insert(plan.id.clone(), plan.clone());
        Ok(())
    }

    fn get_plan(&self, id: &str) -> Result<Option<Plan>> {
        Ok(self.plans.lock().unwrap().get(id).cloned())
    }

    fn list_plans(&self) -> Result<Vec<Plan>> {
        Ok(self.plans.lock().unwrap().values().cloned().collect())
    }

    fn delete_plan(&self, id: &str) -> Result<bool> {
        Ok(self.plans.lock().unwrap().remove(id).is_some())
    }

    fn save_student(&self, student: &Student) -> Result<()> {
        self.students
            .lock()
            .unwrap()
            .insert(student.name.clone(), student.clone());
        Ok(())
    }

    fn get_student(&self, name: &str) -> Result<Option<Student>> {
        Ok(self.students.lock().unwrap().get(name).cloned())
    }

    fn list_students(&self) -> Result<Vec<Student>> {
        Ok(self.students.lock().unwrap().values().cloned().collect())
    }

    fn get_curriculum(&self, name: &str) -> Result<Option<Curriculum>> {
        Ok(self.curricula.lock().unwrap().get(name).cloned())
    }
}

// Mock implementation of the NusmodsApi
pub struct MockNusmodsApi {
    modules: Arc<Mutex<ModuleRegistry>>,
}

impl MockNusmodsApi {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(Mutex::new(ModuleRegistry::new())),
        }
    }

    pub fn with_module(self, module: Module) -> Self {
        self.modules.lock().unwrap().add_module(module);
        self
    }
}

impl module_planner::api::NusmodsApi for MockNusmodsApi {
    fn get_module_registry(&self) -> Result<ModuleRegistry> {
        // This is a mock implementation for testing
        let mut registry = ModuleRegistry::new();

        // Add some test modules
        let module1 = Module {
            module_code: "IE1111R".to_string(),
            title: "Industrial Engineering Principles and Practice".to_string(),
            description: Some(
                "This module introduces the concepts of industrial engineering.".to_string(),
            ),
            module_credit: "4".to_string(),
            department: Some("Industrial Systems Engineering".to_string()),
            faculty: Some("College of Design and Engineering".to_string()),
            semester_data: vec![SemesterData {
                semester: 1,
                exam_date: None,
                exam_duration: None,
                timetable: None,
            }],
            prerequisite: None,
            preclusion: None,
            corequisite: None,
            workload: Some(vec![2.0, 1.0, 1.0, 3.0, 3.0]),
            prereq_tree: None,
            fulfill_requirements: None,
        };

        let module2 = Module {
            module_code: "IE2101".to_string(),
            title: "Systems Design & Analysis".to_string(),
            description: Some("This module covers systems thinking and design.".to_string()),
            module_credit: "4".to_string(),
            department: Some("Industrial Systems Engineering".to_string()),
            faculty: Some("College of Design and Engineering".to_string()),
            semester_data: vec![
                SemesterData {
                    semester: 1,
                    exam_date: None,
                    exam_duration: None,
                    timetable: None,
                },
                SemesterData {
                    semester: 2,
                    exam_date: None,
                    exam_duration: None,
                    timetable: None,
                },
            ],
            prerequisite: Some("IE1111R".to_string()),
            preclusion: None,
            corequisite: None,
            workload: Some(vec![2.0, 1.0, 1.0, 3.0, 3.0]),
            prereq_tree: None,
            fulfill_requirements: None,
        };

        let module3 = Module {
            module_code: "IE3101".to_string(),
            title: "Statistics for Engineering".to_string(),
            description: Some(
                "This module covers statistical methods for engineering.".to_string(),
            ),
            module_credit: "4".to_string(),
            department: Some("Industrial Systems Engineering".to_string()),
            faculty: Some("College of Design and Engineering".to_string()),
            semester_data: vec![SemesterData {
                semester: 2,
                exam_date: None,
                exam_duration: None,
                timetable: None,
            }],
            prerequisite: None,
            preclusion: None,
            corequisite: None,
            workload: Some(vec![3.0, 1.0, 0.0, 3.0, 3.0]),
            prereq_tree: None,
            fulfill_requirements: None,
        };

        registry.add_module(module1);
        registry.add_module(module2);
        registry.add_module(module3);

        Ok(registry)
    }

    fn search_modules(&self, query: &str) -> Result<Vec<Module>> {
        let registry = self.get_module_registry()?;
        let mut results = Vec::new();

        for module_code in ["IE1111R", "IE2101", "IE3101"].iter() {
            if module_code.contains(query) {
                if let Some(module) = registry.get_module(module_code) {
                    results.push(module.clone());
                }
            }
        }

        Ok(results)
    }
}

// Helper function to create a test plan
pub fn create_test_plan() -> Plan {
    let mut plan = Plan::new(
        "test-plan-id".to_string(),
        "Test Plan".to_string(),
        "Test Student".to_string(),
    );

    plan.target_graduation = "2025/2026 Semester 2".to_string();

    // Add some semesters
    let semester1 = SemesterPlan {
        year: "2023/2024".to_string(),
        semester: 1,
        modules: vec![PlannedModule {
            module_code: "IE1111R".to_string(),
            status: ModuleStatus::Completed,
            grade: Some("A".to_string()),
            s_u_option: false,
        }],
        total_units: 4,
        special_activities: Vec::new(),
    };

    let semester2 = SemesterPlan {
        year: "2023/2024".to_string(),
        semester: 2,
        modules: vec![
            PlannedModule {
                module_code: "IE2101".to_string(),
                status: ModuleStatus::Current,
                grade: None,
                s_u_option: false,
            },
            PlannedModule {
                module_code: "IE3101".to_string(),
                status: ModuleStatus::Current,
                grade: None,
                s_u_option: true,
            },
        ],
        total_units: 8,
        special_activities: vec![SpecialActivity {
            activity_type: ActivityType::Research,
            description: "Summer Research Program".to_string(),
            credits: Some(4),
        }],
    };

    plan.semesters.push(semester1);
    plan.semesters.push(semester2);

    plan
}

// Helper function to create a test student
pub fn create_test_student() -> Student {
    Student {
        name: "Test Student".to_string(),
        matriculation_year: "2023/2024".to_string(),
        faculty: "College of Design and Engineering".to_string(),
        major: "Industrial Systems Engineering".to_string(),
        second_major: None,
        minors: Vec::new(),
        completed_modules: HashSet::new(),
        exempted_modules: HashSet::new(),
        advanced_placement_credits: 0,
        current_semester: 2,
        candidature_type: CandidatureType::Standard,
    }
}

// Helper function to create a test curriculum
pub fn create_test_curriculum() -> Curriculum {
    Curriculum {
        name: "Computer Science".to_string(),
        academic_year: "2023/2024".to_string(),
        total_units_required: 160,
        max_level1000_units: 40,
        components: vec![CurriculumComponent {
            name: "University Level Requirements".to_string(),
            min_units: 20,
            requirements: vec![Requirement::FixedModule {
                module_code: "GER1000".to_string(),
                name: "Quantitative Reasoning".to_string(),
                units: 4,
            }],
        }],
    }
}
