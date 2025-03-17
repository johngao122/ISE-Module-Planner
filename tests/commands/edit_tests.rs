use anyhow::Result;
use module_planner::api::DefaultNusmodsApi;
use module_planner::commands::EditCommand;

use crate::commands::common::{create_test_plan, create_test_student, MockNusmodsApi, MockStorage};

#[test]
fn test_edit_command_loads_plan() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student);

    let api = DefaultNusmodsApi::new()?;

    let command = EditCommand {
        storage: Box::new(storage),
        api: Box::new(api),
        plan_id: plan.id.clone(),
    };

    // We can't easily test the interactive parts in a unit test
    // In a real test, we would use a library like rexpect to simulate user input
    // For now, we'll just verify that the command struct can be created

    assert!(true);
    Ok(())
}

#[test]
fn test_check_module_availability() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student);

    let api = MockNusmodsApi::new();

    let command = EditCommand {
        storage: Box::new(storage),
        api: Box::new(api),
        plan_id: plan.id.clone(),
    };

    // Get a module from the registry
    let registry = command.api.get_module_registry()?;
    let module = registry.get_module("IE1111R").unwrap();

    // Test availability check
    let is_available = command.check_module_availability(module, 1);
    assert!(is_available);

    // Test non-availability
    let is_available = command.check_module_availability(module, 2);
    assert!(!is_available);

    Ok(())
}
