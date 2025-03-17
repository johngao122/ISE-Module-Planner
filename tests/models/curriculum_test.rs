use module_planner::models::{Curriculum, CurriculumComponent, Requirement};

#[test]
fn test_curriculum_creation() {
    let curriculum = Curriculum {
        name: "Industrial Systems Engineering".to_string(),
        academic_year: "2023/2024".to_string(),
        total_units_required: 160,
        max_level1000_units: 60,
        components: vec![],
    };

    assert_eq!(curriculum.name, "Industrial Systems Engineering");
    assert_eq!(curriculum.academic_year, "2023/2024");
    assert_eq!(curriculum.total_units_required, 160);
    assert_eq!(curriculum.max_level1000_units, 60);
    assert!(curriculum.components.is_empty());
}

#[test]
fn test_curriculum_with_components() {
    let component = CurriculumComponent {
        name: "ISE Foundation".to_string(),
        min_units: 36,
        requirements: vec![
            Requirement::FixedModule {
                module_code: "IE1111R".to_string(),
                name: "Industrial Engineering Principles and Practice".to_string(),
                units: 4,
            },
            Requirement::FixedModule {
                module_code: "IE2130".to_string(),
                name: "Quality Engineering".to_string(),
                units: 4,
            },
            Requirement::FixedModule {
                module_code: "IE3100".to_string(),
                name: "Systems Design & Analysis".to_string(),
                units: 4,
            },
        ],
    };

    let curriculum = Curriculum {
        name: "Industrial Systems Engineering".to_string(),
        academic_year: "2023/2024".to_string(),
        total_units_required: 160,
        max_level1000_units: 60,
        components: vec![component],
    };

    assert_eq!(curriculum.components.len(), 1);
    assert_eq!(curriculum.components[0].name, "ISE Foundation");
    assert_eq!(curriculum.components[0].min_units, 36);
    assert_eq!(curriculum.components[0].requirements.len(), 3);
}

#[test]
fn test_fixed_module_requirement() {
    let requirement = Requirement::FixedModule {
        module_code: "IE1111R".to_string(),
        name: "Industrial Engineering Principles and Practice".to_string(),
        units: 4,
    };

    match requirement {
        Requirement::FixedModule {
            module_code,
            name,
            units,
        } => {
            assert_eq!(module_code, "IE1111R");
            assert_eq!(name, "Industrial Engineering Principles and Practice");
            assert_eq!(units, 4);
        }
        _ => panic!("Expected Requirement::FixedModule"),
    }
}

#[test]
fn test_module_group_requirement() {
    let requirement = Requirement::ModuleGroup {
        name: "Engineering Mathematics".to_string(),
        description: Some("Choose one of the following modules".to_string()),
        min_units: 4,
        possible_modules: vec!["MA1301".to_string(), "MA1312".to_string()],
    };

    match requirement {
        Requirement::ModuleGroup {
            name,
            description,
            min_units,
            possible_modules,
        } => {
            assert_eq!(name, "Engineering Mathematics");
            assert!(description.is_some());
            assert_eq!(min_units, 4);
            assert_eq!(possible_modules.len(), 2);
            assert!(possible_modules.contains(&"MA1301".to_string()));
            assert!(possible_modules.contains(&"MA1312".to_string()));
        }
        _ => panic!("Expected Requirement::ModuleGroup"),
    }
}

#[test]
fn test_elective_requirement() {
    let requirement = Requirement::Elective {
        name: "ISE Technical Electives".to_string(),
        description: "Choose modules from the Industrial Systems Engineering department"
            .to_string(),
        min_units: 24,
        level_constraint: Some(vec![4000, 5000]),
        department_constraint: Some(vec!["IE".to_string()]),
    };

    match requirement {
        Requirement::Elective {
            name,
            description,
            min_units,
            level_constraint,
            department_constraint,
        } => {
            assert_eq!(name, "ISE Technical Electives");
            assert_eq!(
                description,
                "Choose modules from the Industrial Systems Engineering department"
            );
            assert_eq!(min_units, 24);
            assert!(level_constraint.is_some());
            assert_eq!(level_constraint.unwrap(), vec![4000, 5000]);
            assert!(department_constraint.is_some());
            assert_eq!(department_constraint.unwrap(), vec!["IE".to_string()]);
        }
        _ => panic!("Expected Requirement::Elective"),
    }
}
