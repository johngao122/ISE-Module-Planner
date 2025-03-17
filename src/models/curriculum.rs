use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curriculum {
    pub name: String,
    pub academic_year: String,
    pub total_units_required: u32,
    pub max_level1000_units: u32,
    pub components: Vec<CurriculumComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurriculumComponent {
    pub name: String,
    pub min_units: u32,
    pub requirements: Vec<Requirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Requirement {
    FixedModule {
        module_code: String,
        name: String,
        units: u32,
    },
    ModuleGroup {
        name: String,
        description: Option<String>,
        min_units: u32,
        possible_modules: Vec<String>,
    },
    Elective {
        name: String,
        description: String,
        min_units: u32,
        level_constraint: Option<Vec<u32>>,
        department_constraint: Option<Vec<String>>,
    },
}
