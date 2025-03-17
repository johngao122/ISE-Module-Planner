use super::module::Module;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub student_id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub target_graduation: String,
    pub semesters: Vec<SemesterPlan>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemesterPlan {
    pub year: String,
    pub semester: u8,
    pub modules: Vec<PlannedModule>,
    pub total_units: u32,
    pub special_activities: Vec<SpecialActivity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedModule {
    pub module_code: String,
    pub status: ModuleStatus,
    pub grade: Option<String>,
    pub s_u_option: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleStatus {
    Planned,
    Current,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialActivity {
    pub activity_type: ActivityType,
    pub description: String,
    pub credits: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    InternationalExchange,
    IndustrialAttachment,
    Research,
    CommunityService,
    Other(String),
}

impl Plan {
    pub fn new(id: String, name: String, student_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            student_id,
            created_at: now,
            modified_at: now,
            target_graduation: String::new(),
            semesters: Vec::new(),
            notes: None,
        }
    }

    pub fn total_units(&self) -> u32 {
        self.semesters.iter().map(|s| s.total_units).sum()
    }

    pub fn all_modules(&self) -> HashSet<String> {
        self.semesters
            .iter()
            .flat_map(|s| s.modules.iter().map(|m| m.module_code.clone()))
            .collect()
    }
}
