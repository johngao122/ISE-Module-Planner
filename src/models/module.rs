use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub module_code: String,
    pub title: String,
    pub description: Option<String>,
    pub module_credit: String,
    pub department: Option<String>,
    pub faculty: Option<String>,
    pub workload: Option<Vec<f32>>,
    pub prerequisite: Option<String>,
    pub preclusion: Option<String>,
    pub corequisite: Option<String>,
    pub semester_data: Vec<SemesterData>,
    pub prereq_tree: Option<PrereqTree>,
    pub fulfill_requirements: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemesterData {
    pub semester: u8,
    pub exam_date: Option<String>,
    pub exam_duration: Option<u32>,
    pub timetable: Option<Vec<Lesson>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub class_no: String,
    pub lesson_type: String,
    pub week_type: WeekType,
    pub day: String,
    pub start_time: String,
    pub end_time: String,
    pub venue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WeekType {
    Weeks(Vec<u8>),
    WeekRange {
        start: String,
        end: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        week_interval: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        weeks: Option<Vec<u8>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrereqTree {
    ModuleCode(String),
    And { and: Vec<PrereqTree> },
    Or { or: Vec<PrereqTree> },
    String(String),
}
