use module_planner::models::{CandidatureType, Student};
use std::collections::HashSet;

#[test]
fn test_student_creation() {
    let mut completed_modules = HashSet::new();
    completed_modules.insert("IE1111R".to_string());
    completed_modules.insert("MA1301".to_string());

    let mut exempted_modules = HashSet::new();
    exempted_modules.insert("GER1000".to_string());

    let student = Student {
        name: "John Doe".to_string(),
        matriculation_year: "2022".to_string(),
        faculty: "Faculty of Engineering".to_string(),
        major: "Industrial Systems Engineering".to_string(),
        second_major: None,
        minors: vec!["Business".to_string()],
        completed_modules,
        exempted_modules,
        advanced_placement_credits: 8,
        current_semester: 2,
        candidature_type: CandidatureType::Standard,
    };

    assert_eq!(student.name, "John Doe");
    assert_eq!(student.matriculation_year, "2022");
    assert_eq!(student.faculty, "Faculty of Engineering");
    assert_eq!(student.major, "Industrial Systems Engineering");
    assert!(student.second_major.is_none());
    assert_eq!(student.minors.len(), 1);
    assert_eq!(student.minors[0], "Business");
    assert_eq!(student.completed_modules.len(), 2);
    assert!(student.completed_modules.contains("IE1111R"));
    assert!(student.completed_modules.contains("MA1301"));
    assert_eq!(student.exempted_modules.len(), 1);
    assert!(student.exempted_modules.contains("GER1000"));
    assert_eq!(student.advanced_placement_credits, 8);
    assert_eq!(student.current_semester, 2);
}

#[test]
fn test_remaining_semesters_standard() {
    let student = Student {
        name: "John Doe".to_string(),
        matriculation_year: "2022".to_string(),
        faculty: "Faculty of Engineering".to_string(),
        major: "Industrial Systems Engineering".to_string(),
        second_major: None,
        minors: vec![],
        completed_modules: HashSet::new(),
        exempted_modules: HashSet::new(),
        advanced_placement_credits: 0,
        current_semester: 2,
        candidature_type: CandidatureType::Standard,
    };

    assert_eq!(student.remaining_semesters(), 6);
}

#[test]
fn test_remaining_semesters_double_honours() {
    let student = Student {
        name: "Jane Smith".to_string(),
        matriculation_year: "2022".to_string(),
        faculty: "Faculty of Engineering".to_string(),
        major: "Industrial Systems Engineering".to_string(),
        second_major: Some("Business".to_string()),
        minors: vec![],
        completed_modules: HashSet::new(),
        exempted_modules: HashSet::new(),
        advanced_placement_credits: 0,
        current_semester: 4,
        candidature_type: CandidatureType::DoubleHonours,
    };

    assert_eq!(student.remaining_semesters(), 6);
}

#[test]
fn test_remaining_semesters_double_degree() {
    let student = Student {
        name: "Alex Johnson".to_string(),
        matriculation_year: "2022".to_string(),
        faculty: "School of Computing".to_string(),
        major: "Computer Science".to_string(),
        second_major: None,
        minors: vec![],
        completed_modules: HashSet::new(),
        exempted_modules: HashSet::new(),
        advanced_placement_credits: 0,
        current_semester: 6,
        candidature_type: CandidatureType::DoubleDegreeProgramme,
    };

    assert_eq!(student.remaining_semesters(), 4);
}

#[test]
fn test_remaining_semesters_concurrent_degree() {
    let student = Student {
        name: "Sarah Lee".to_string(),
        matriculation_year: "2022".to_string(),
        faculty: "School of Computing".to_string(),
        major: "Computer Science".to_string(),
        second_major: None,
        minors: vec![],
        completed_modules: HashSet::new(),
        exempted_modules: HashSet::new(),
        advanced_placement_credits: 0,
        current_semester: 8,
        candidature_type: CandidatureType::ConcurrentDegree,
    };

    assert_eq!(student.remaining_semesters(), 2);
}

#[test]
fn test_remaining_semesters_engineering_scholars() {
    let student = Student {
        name: "Michael Wong".to_string(),
        matriculation_year: "2022".to_string(),
        faculty: "Faculty of Engineering".to_string(),
        major: "Electrical Engineering".to_string(),
        second_major: None,
        minors: vec![],
        completed_modules: HashSet::new(),
        exempted_modules: HashSet::new(),
        advanced_placement_credits: 0,
        current_semester: 7,
        candidature_type: CandidatureType::EngineeringScholars,
    };

    assert_eq!(student.remaining_semesters(), 1);
}
