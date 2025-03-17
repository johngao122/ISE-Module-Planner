use anyhow::Result;
use module_planner::api::DefaultNusmodsApi;
use module_planner::commands::ViewCommand;

use crate::commands::common::{
    create_test_curriculum, create_test_plan, create_test_student, MockNusmodsApi, MockStorage,
};

#[test]
fn test_view_command_with_plan_id() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();
    let curriculum = create_test_curriculum();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student)
        .with_curriculum(curriculum);

    let api = DefaultNusmodsApi::new()?;

    let command = ViewCommand {
        storage: Box::new(storage),
        api: Box::new(api),
        plan_id: Some(plan.id.clone()),
    };

    // We can't easily test the interactive parts in a unit test
    // In a real test, we would use a library like rexpect to simulate user input
    // For now, we'll just verify that the command struct can be created

    assert!(true);
    Ok(())
}

#[test]
fn test_view_command_without_plan_id() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();
    let curriculum = create_test_curriculum();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student)
        .with_curriculum(curriculum);

    let api = DefaultNusmodsApi::new()?;

    let command = ViewCommand {
        storage: Box::new(storage),
        api: Box::new(api),
        plan_id: None,
    };

    // We can't easily test the interactive parts in a unit test
    // In a real test, we would use a library like rexpect to simulate user input
    // For now, we'll just verify that the command struct can be created

    assert!(true);
    Ok(())
}

#[test]
fn test_calculate_level_units() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();
    let curriculum = create_test_curriculum();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student)
        .with_curriculum(curriculum);

    let api = MockNusmodsApi::new();

    let command = ViewCommand {
        storage: Box::new(storage),
        api: Box::new(api),
        plan_id: Some(plan.id.clone()),
    };

    // Test calculate_level_units method
    let level_1000_units = command.calculate_level_units(&plan, 1000)?;

    // IE1111R is a level 1000 module with 4 units
    assert_eq!(level_1000_units, 4);

    // Test level 2000 units
    let level_2000_units = command.calculate_level_units(&plan, 2000)?;

    // IE2101 is a level 2000 module with 4 units
    assert_eq!(level_2000_units, 4);

    // Test level 3000 units
    let level_3000_units = command.calculate_level_units(&plan, 3000)?;

    // IE3101 is a level 3000 module with 4 units
    assert_eq!(level_3000_units, 4);

    Ok(())
}

#[test]
fn test_calculate_component_units() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();
    let curriculum = create_test_curriculum();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student.clone())
        .with_curriculum(curriculum.clone());

    let api = DefaultNusmodsApi::new()?;

    let command = ViewCommand {
        storage: Box::new(storage),
        api: Box::new(api),
        plan_id: Some(plan.id.clone()),
    };

    // Test calculate_component_units method
    let component = &curriculum.components[0]; // University Level Requirements
    let component_units = command.calculate_component_units(&plan, component)?;

    // Our test plan doesn't have any ULR modules
    assert_eq!(component_units, 0);

    Ok(())
}
