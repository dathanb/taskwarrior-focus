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
    // compact sortOrder values on focus tasks
    compact_sort_order(&tasks)?;

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

fn compact_sort_order(tasks: &Vec<Task>) -> Result<()> {
    let focused_tasks: Vec<&Task> = tasks.iter()
                                         .filter(|t| t.tags.contains(&"focus".to_string()))
                                         .collect();

    struct TaskSortOrder(String, f64);
    let mut sort_orders = Vec::new();
    for task in focused_tasks {
        let sort_order = match task.udas.get("sortOrder") {
            Some(v) => serde_json::to_string(v)?,
            None => "0.0".to_string()
        };
        let sort_order: f64 = sort_order.parse()?;
        sort_orders.push(TaskSortOrder(task.uuid.clone(), sort_order));
    }

    sort_orders.sort_by(|a, b| a.1.total_cmp(&b.1));

    let mut index = 1.0;
    for task in sort_orders {
        update_sort_order(task.0.as_str(), index)?;
        index += 1.0;
    }

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

fn update_sort_order(uuid: &str, sort_order: f64) -> Result<()> {
    std::process::Command::new("task")
        .args([uuid, "mod", format!("sortOrder:{}", sort_order).as_str()])
        .output()?;
    // TODO: detect and handle the case where it didn't work
    Ok(())
}