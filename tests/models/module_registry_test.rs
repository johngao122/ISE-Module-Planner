use module_planner::models::{Module, ModuleRegistry};

#[test]
fn test_module_registry_creation() {
    let registry = ModuleRegistry::new();
    assert!(registry.contains_module("IE1111R") == false);
}

#[test]
fn test_module_registry_add_and_get() {
    let mut registry = ModuleRegistry::new();

    let module = Module {
        module_code: "IE1111R".to_string(),
        title: "Industrial Engineering Principles and Practice".to_string(),
        description: Some(
            "This module introduces the fundamental principles of industrial engineering."
                .to_string(),
        ),
        module_credit: "4".to_string(),
        department: Some("Industrial Systems Engineering".to_string()),
        faculty: Some("Faculty of Engineering".to_string()),
        workload: Some(vec![2.0, 1.0, 1.0, 3.0, 3.0]),
        prerequisite: None,
        preclusion: None,
        corequisite: None,
        semester_data: vec![],
        prereq_tree: None,
        fulfill_requirements: Some(vec!["ISE Foundation".to_string()]),
    };

    registry.add_module(module);

    assert!(registry.contains_module("IE1111R"));
    assert!(!registry.contains_module("IE2130"));

    let retrieved_module = registry.get_module("IE1111R");
    assert!(retrieved_module.is_some());

    let retrieved_module = retrieved_module.unwrap();
    assert_eq!(retrieved_module.module_code, "IE1111R");
    assert_eq!(
        retrieved_module.title,
        "Industrial Engineering Principles and Practice"
    );

    let non_existent_module = registry.get_module("IE2130");
    assert!(non_existent_module.is_none());
}

#[test]
fn test_module_registry_multiple_modules() {
    let mut registry = ModuleRegistry::new();

    let module1 = Module {
        module_code: "IE1111R".to_string(),
        title: "Industrial Engineering Principles and Practice".to_string(),
        description: Some(
            "This module introduces the fundamental principles of industrial engineering."
                .to_string(),
        ),
        module_credit: "4".to_string(),
        department: Some("Industrial Systems Engineering".to_string()),
        faculty: Some("Faculty of Engineering".to_string()),
        workload: Some(vec![2.0, 1.0, 1.0, 3.0, 3.0]),
        prerequisite: None,
        preclusion: None,
        corequisite: None,
        semester_data: vec![],
        prereq_tree: None,
        fulfill_requirements: Some(vec!["ISE Foundation".to_string()]),
    };

    let module2 = Module {
        module_code: "IE2130".to_string(),
        title: "Quality Engineering".to_string(),
        description: Some(
            "This module covers statistical quality control and quality management systems."
                .to_string(),
        ),
        module_credit: "4".to_string(),
        department: Some("Industrial Systems Engineering".to_string()),
        faculty: Some("Faculty of Engineering".to_string()),
        workload: Some(vec![2.0, 1.0, 1.0, 3.0, 3.0]),
        prerequisite: Some("IE1111R".to_string()),
        preclusion: None,
        corequisite: None,
        semester_data: vec![],
        prereq_tree: None,
        fulfill_requirements: Some(vec!["ISE Foundation".to_string()]),
    };

    registry.add_module(module1);
    registry.add_module(module2);

    assert!(registry.contains_module("IE1111R"));
    assert!(registry.contains_module("IE2130"));

    let retrieved_module1 = registry.get_module("IE1111R").unwrap();
    let retrieved_module2 = registry.get_module("IE2130").unwrap();

    assert_eq!(retrieved_module1.module_code, "IE1111R");
    assert_eq!(retrieved_module2.module_code, "IE2130");
    assert_eq!(retrieved_module2.prerequisite, Some("IE1111R".to_string()));
}
