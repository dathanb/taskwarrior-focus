#![feature(is_some_with)]

mod model;

use clap::{Arg, ArgMatches, Command};
use anyhow::{anyhow, Result};
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
        Some(("gc", _)) => handle_gc()?,
        Some(("prioritize", sub_matches)) => handle_prioritize_cmd(sub_matches)?,
        Some(("deprioritize", sub_matches)) => handle_deprioritize_cmd(sub_matches)?,
        _ => panic!("Unhandled subcommand!")
    };

    Ok(())
}

fn handle_gc() -> Result<()> {
    let tasks = get_all_tasks()?;
    // clean up sortOrder UDAs on non-focus tasks
    clean_up_non_focus_tasks(&tasks)?;
    // compact sortOrder values on focus tasks
    compact_sort_order(&tasks)?;

    Ok(())
}

fn handle_prioritize_cmd(sub_matches: &ArgMatches) -> Result<()> {
    let id: &String = sub_matches.get_one::<String>("id").expect("id is a required option, so should always have a value");

    let tasks = get_all_tasks()?;

    let target_task = tasks.iter()
        .find(|task| &task.uuid == id || task.id.is_some_and(|tid| &tid.to_string() == id))
        .ok_or(anyhow!("No task with id {} found", id))?;

    let focused_tasks: Vec<&Task> = tasks.iter()
                                         .filter(|t| t.tags.contains(&"focus".to_string()))
                                         .collect();

    let mut min_sort_order = f64::MAX;
    for task in focused_tasks {
        let sort_order = task.sort_order()?;
        min_sort_order = f64::min(min_sort_order, sort_order);
    }

    let current_sort_order = target_task.sort_order()?;

    if current_sort_order == min_sort_order {
        // it's already the highest-priority focused item, so no need to do anything
        return Ok(());
    }

    prioritize(id.as_str(), min_sort_order - 1.0)?;

    Ok(())
}

fn handle_deprioritize_cmd(sub_matches: &ArgMatches) -> Result<()> {
    let id: &String = sub_matches.get_one::<String>("id").expect("id is a required option, so should always have a value");

    let tasks = get_all_tasks()?;

    let target_task = tasks.iter()
                           .find(|task| &task.uuid == id || task.id.is_some_and(|tid| &tid.to_string() == id))
                           .ok_or(anyhow!("No task with id {} found", id))?;

    let focused_tasks: Vec<&Task> = tasks.iter()
                                         .filter(|t| t.tags.contains(&"focus".to_string()))
                                         .collect();

    let mut max_sort_order = f64::MIN;
    for task in focused_tasks {
        let sort_order = task.sort_order()?;
        max_sort_order = f64::max(max_sort_order, sort_order);
    }

    let current_sort_order = target_task.sort_order()?;

    if current_sort_order == max_sort_order {
        // it's already the highest-priority focused item, so no need to do anything
        return Ok(());
    }

    prioritize(id.as_str(), max_sort_order + 1.0)?;

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

fn prioritize(id: &str, new_sort_order: f64) -> Result<()> {
    std::process::Command::new("task")
        .args([id, "mod", format!("sortOrder:{}", new_sort_order).as_str()])
        .output()?;
    // TODO: detect and handle the case where it didn't work

    Ok(())
}