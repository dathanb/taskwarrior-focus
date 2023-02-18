mod model;

use clap::{Arg, Command};
use anyhow::Result;
use crate::model::Task;

fn main() -> Result<()> {
    let gc_cmd = Command::new("gc")
        .about("Clean up focus task metadata");
    let prioritize_cmd = Command::new("prioritize")
        .about("Prioritize a task by id")
        .arg(Arg::new("id").index(1).required(true));
    let deprioritize_cmd = Command::new("deprioritize")
        .about("Deprioritized a focused task by id")
        .arg(Arg::new("id").index(1).required(true));
    let matches = Command::new("main")
        .subcommand_required(true)
        .infer_subcommands(true)
        .subcommand(gc_cmd)
        .subcommand(prioritize_cmd)
        .subcommand(deprioritize_cmd)
        .get_matches();

    match matches.subcommand() {
        Some(("gc", _)) => gc()?,
        Some(("prioritize", _)) => panic!("Prioritize not implement"),
        Some(("deprioritize", _)) => panic!("Deprioritize not implemented"),
        _ => panic!("Unhandled subcommand!")
    };

    Ok(())
}

fn gc() -> Result<()> {
    let tasks = get_all_tasks()?;
    // clean up sortOrder UDAs on non-focus tasks
    clean_up_non_focus_tasks(&tasks)?;
    // assign sortOrder on focus tasks
    assign_sort_order_where_missing()?;
    // compact sortOrder values on focus tasks
    compact_sort_order()?;

    Ok(())
}

fn clean_up_non_focus_tasks(tasks: &Vec<Task>) -> Result<()> {
    let mut task_ids_to_clean: Vec<String> = Vec::new();

    for task in tasks {
        if !task.tags.contains(&"focus".to_string()) && task.udas.contains_key("sortOrder") {
            task_ids_to_clean.push(task.uuid.clone());
        }
    }

    for task_id in task_ids_to_clean {
        remove_sort_order(task_id)?;
    }

    Ok(())
}

fn assign_sort_order_where_missing() -> Result<()> {
    Ok(())
}

fn compact_sort_order() -> Result<()> {
    Ok(())
}

fn get_all_tasks() -> Result<Vec<Task>> {
    let output = std::process::Command::new("task")
        .args(["+PENDING", "export"])
        .output()?;
    let command_output = String::from_utf8(output.stdout)?;
    Ok(serde_json::from_str(command_output.as_str())?)
}

fn remove_sort_order(uuid: String) -> Result<()> {
    std::process::Command::new("task")
        .args([uuid.as_str(), "mod", "sortOrder:"])
        .output()?;
    // TODO: detect and handle the case where it didn't work
    Ok(())
}