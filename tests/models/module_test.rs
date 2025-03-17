use module_planner::models::{Lesson, Module, PrereqTree, SemesterData, WeekType};

#[test]
fn test_module_creation() {
    let module = Module {
        module_code: "IE2130".to_string(),
        title: "Quality Engineering".to_string(),
        description: Some(
            "This module introduces the concepts of quality engineering and statistical process control."
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

    assert_eq!(module.module_code, "IE2130");
    assert_eq!(module.title, "Quality Engineering");
    assert_eq!(module.module_credit, "4");
    assert!(module.description.is_some());
    assert!(module.department.is_some());
    assert!(module.faculty.is_some());
    assert!(module.workload.is_some());
    assert!(module.fulfill_requirements.is_some());
}

#[test]
fn test_semester_data() {
    let lesson = Lesson {
        class_no: "1".to_string(),
        lesson_type: "Lecture".to_string(),
        week_type: WeekType::Weeks(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]),
        day: "Monday".to_string(),
        start_time: "10:00".to_string(),
        end_time: "12:00".to_string(),
        venue: "LT19".to_string(),
    };

    let semester_data = SemesterData {
        semester: 1,
        exam_date: Some("2023-12-01".to_string()),
        exam_duration: Some(120),
        timetable: Some(vec![lesson]),
    };

    assert_eq!(semester_data.semester, 1);
    assert_eq!(semester_data.exam_date, Some("2023-12-01".to_string()));
    assert_eq!(semester_data.exam_duration, Some(120));
    assert!(semester_data.timetable.is_some());
    assert_eq!(semester_data.timetable.as_ref().unwrap().len(), 1);
}

#[test]
fn test_week_type_weeks() {
    let week_type = WeekType::Weeks(vec![1, 2, 3, 4, 5]);

    if let WeekType::Weeks(weeks) = week_type {
        assert_eq!(weeks, vec![1, 2, 3, 4, 5]);
    } else {
        panic!("Expected WeekType::Weeks");
    }
}

#[test]
fn test_week_type_range() {
    let week_type = WeekType::WeekRange {
        start: "2023-08-14".to_string(),
        end: "2023-11-17".to_string(),
        week_interval: Some(1),
        weeks: None,
    };

    if let WeekType::WeekRange {
        start,
        end,
        week_interval,
        weeks,
    } = week_type
    {
        assert_eq!(start, "2023-08-14");
        assert_eq!(end, "2023-11-17");
        assert_eq!(week_interval, Some(1));
        assert_eq!(weeks, None);
    } else {
        panic!("Expected WeekType::WeekRange");
    }
}

#[test]
fn test_prereq_tree() {
    // Test ModuleCode variant
    let prereq_module = PrereqTree::ModuleCode("MA1301".to_string());
    if let PrereqTree::ModuleCode(code) = &prereq_module {
        assert_eq!(code, "MA1301");
    } else {
        panic!("Expected PrereqTree::ModuleCode");
    }

    // Test And variant
    let prereq_and = PrereqTree::And {
        and: vec![
            PrereqTree::ModuleCode("MA1301".to_string()),
            PrereqTree::ModuleCode("IE2110".to_string()),
        ],
    };
    if let PrereqTree::And { and } = &prereq_and {
        assert_eq!(and.len(), 2);
    } else {
        panic!("Expected PrereqTree::And");
    }

    // Test Or variant
    let prereq_or = PrereqTree::Or {
        or: vec![
            PrereqTree::ModuleCode("IE2110".to_string()),
            PrereqTree::ModuleCode("IE2111".to_string()),
        ],
    };
    if let PrereqTree::Or { or } = &prereq_or {
        assert_eq!(or.len(), 2);
    } else {
        panic!("Expected PrereqTree::Or");
    }

    // Test String variant
    let prereq_string = PrereqTree::String("Completed 80 MCs or more".to_string());
    if let PrereqTree::String(s) = &prereq_string {
        assert_eq!(s, "Completed 80 MCs or more");
    } else {
        panic!("Expected PrereqTree::String");
    }
}
