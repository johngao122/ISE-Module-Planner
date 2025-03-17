use module_planner::models::{
    ActivityType, ModuleStatus, Plan, PlannedModule, SemesterPlan, SpecialActivity,
};

#[test]
fn test_plan_creation() {
    let plan = Plan::new(
        "plan1".to_string(),
        "ISE Degree Plan".to_string(),
        "A0123456X".to_string(),
    );

    assert_eq!(plan.id, "plan1");
    assert_eq!(plan.name, "ISE Degree Plan");
    assert_eq!(plan.student_id, "A0123456X");
    assert!(plan.semesters.is_empty());
    assert_eq!(plan.total_units(), 0);
    assert!(plan.notes.is_none());
}

#[test]
fn test_plan_with_semesters() {
    let mut plan = Plan::new(
        "plan1".to_string(),
        "ISE Degree Plan".to_string(),
        "A0123456X".to_string(),
    );

    // Add a semester with modules
    let semester_plan = SemesterPlan {
        year: "2023/2024".to_string(),
        semester: 1,
        modules: vec![
            PlannedModule {
                module_code: "IE1111R".to_string(),
                status: ModuleStatus::Completed,
                grade: Some("A".to_string()),
                s_u_option: false,
            },
            PlannedModule {
                module_code: "IE2130".to_string(),
                status: ModuleStatus::Current,
                grade: None,
                s_u_option: false,
            },
        ],
        total_units: 8,
        special_activities: vec![],
    };

    plan.semesters.push(semester_plan);

    assert_eq!(plan.semesters.len(), 1);
    assert_eq!(plan.total_units(), 8);

    // Test all_modules method
    let all_modules = plan.all_modules();
    assert_eq!(all_modules.len(), 2);
    assert!(all_modules.contains("IE1111R"));
    assert!(all_modules.contains("IE2130"));
}

#[test]
fn test_semester_plan() {
    let semester_plan = SemesterPlan {
        year: "2023/2024".to_string(),
        semester: 2,
        modules: vec![PlannedModule {
            module_code: "IE3100".to_string(),
            status: ModuleStatus::Planned,
            grade: None,
            s_u_option: true,
        }],
        total_units: 4,
        special_activities: vec![SpecialActivity {
            activity_type: ActivityType::IndustrialAttachment,
            description: "Summer internship at Manufacturing Company".to_string(),
            credits: Some(12),
        }],
    };

    assert_eq!(semester_plan.year, "2023/2024");
    assert_eq!(semester_plan.semester, 2);
    assert_eq!(semester_plan.modules.len(), 1);
    assert_eq!(semester_plan.total_units, 4);
    assert_eq!(semester_plan.special_activities.len(), 1);
}

#[test]
fn test_module_status() {
    let planned = ModuleStatus::Planned;
    let current = ModuleStatus::Current;
    let completed = ModuleStatus::Completed;
    let failed = ModuleStatus::Failed;

    // Test pattern matching on module status
    match planned {
        ModuleStatus::Planned => assert!(true),
        _ => panic!("Expected ModuleStatus::Planned"),
    }

    match current {
        ModuleStatus::Current => assert!(true),
        _ => panic!("Expected ModuleStatus::Current"),
    }

    match completed {
        ModuleStatus::Completed => assert!(true),
        _ => panic!("Expected ModuleStatus::Completed"),
    }

    match failed {
        ModuleStatus::Failed => assert!(true),
        _ => panic!("Expected ModuleStatus::Failed"),
    }
}

#[test]
fn test_activity_type() {
    let exchange = ActivityType::InternationalExchange;
    let attachment = ActivityType::IndustrialAttachment;
    let research = ActivityType::Research;
    let service = ActivityType::CommunityService;
    let other = ActivityType::Other("Leadership Program".to_string());

    // Test pattern matching on activity type
    match exchange {
        ActivityType::InternationalExchange => assert!(true),
        _ => panic!("Expected ActivityType::InternationalExchange"),
    }

    match attachment {
        ActivityType::IndustrialAttachment => assert!(true),
        _ => panic!("Expected ActivityType::IndustrialAttachment"),
    }

    match research {
        ActivityType::Research => assert!(true),
        _ => panic!("Expected ActivityType::Research"),
    }

    match service {
        ActivityType::CommunityService => assert!(true),
        _ => panic!("Expected ActivityType::CommunityService"),
    }

    match other {
        ActivityType::Other(desc) => assert_eq!(desc, "Leadership Program"),
        _ => panic!("Expected ActivityType::Other"),
    }
}
