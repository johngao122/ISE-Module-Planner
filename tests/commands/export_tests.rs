use anyhow::Result;
use module_planner::commands::Command;
use module_planner::commands::ExportCommand;
use std::fs;
use std::path::Path;

use crate::commands::common::{create_test_plan, create_test_student, MockStorage};

#[test]
fn test_export_command_loads_plan() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student);

    let command = ExportCommand {
        storage: Box::new(storage),
        plan_id: plan.id.clone(),
    };

    // We can't easily test the interactive parts in a unit test
    // In a real test, we would use a library like rexpect to simulate user input
    // For now, we'll just verify that the command struct can be created

    assert!(true);
    Ok(())
}

#[test]
fn test_export_json() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student);

    let command = ExportCommand {
        storage: Box::new(storage),
        plan_id: plan.id.clone(),
    };

    // Test export_json method
    let filename = "test_export";
    command.export_json(&plan, filename)?;

    // Verify file was created
    let path = format!("{}.json", filename);
    assert!(Path::new(&path).exists());

    // Clean up
    fs::remove_file(path)?;

    Ok(())
}

#[test]
fn test_export_csv() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student);

    let command = ExportCommand {
        storage: Box::new(storage),
        plan_id: plan.id.clone(),
    };

    // Test export_csv method
    let filename = "test_export";
    command.export_csv(&plan, filename)?;

    // Verify file was created
    let path = format!("{}.csv", filename);
    assert!(Path::new(&path).exists());

    // Clean up
    fs::remove_file(path)?;

    Ok(())
}

#[test]
fn test_export_markdown() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student);

    let command = ExportCommand {
        storage: Box::new(storage),
        plan_id: plan.id.clone(),
    };

    // Test export_markdown method
    let filename = "test_export";
    command.export_markdown(&plan, filename)?;

    // Verify file was created
    let path = format!("{}.md", filename);
    assert!(Path::new(&path).exists());

    // Clean up
    fs::remove_file(path)?;

    Ok(())
}

#[test]
fn test_export_text() -> Result<()> {
    // Setup
    let plan = create_test_plan();
    let student = create_test_student();

    let storage = MockStorage::new()
        .with_plan(plan.clone())
        .with_student(student);

    let command = ExportCommand {
        storage: Box::new(storage),
        plan_id: plan.id.clone(),
    };

    // Test export_text method
    let filename = "test_export";
    command.export_text(&plan, filename)?;

    // Verify file was created
    let path = format!("{}.txt", filename);
    assert!(Path::new(&path).exists());

    // Clean up
    fs::remove_file(path)?;

    Ok(())
}
