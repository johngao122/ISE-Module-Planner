use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub matriculation_year: String,
    pub faculty: String,
    pub major: String,
    pub second_major: Option<String>,
    pub minors: Vec<String>,
    pub completed_modules: HashSet<String>,
    pub exempted_modules: HashSet<String>,
    pub advanced_placement_credits: u32,
    pub current_semester: u8,
    pub candidature_type: CandidatureType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CandidatureType {
    Standard,
    DoubleHonours,
    DoubleDegreeProgramme,
    ConcurrentDegree,
    EngineeringScholars,
}

impl Student {
    pub fn remaining_semesters(&self) -> u8 {
        match self.candidature_type {
            CandidatureType::Standard => 8 - self.current_semester,
            CandidatureType::DoubleHonours => 10 - self.current_semester,
            CandidatureType::DoubleDegreeProgramme => 10 - self.current_semester,
            CandidatureType::ConcurrentDegree => 10 - self.current_semester,
            CandidatureType::EngineeringScholars => 8 - self.current_semester,
        }
    }
}
