use std::io;
use clap::{Arg, Command};
use anyhow::Result;

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

    println!("Hello, world!");

    Ok(())
}

fn gc() -> Result<()> {
    // clean up sortOrder UDAs on non-focus tasks
    clean_up_non_focus_tasks()?;
    // assign sortOrder on focus tasks
    assign_sort_order_where_missing()?;
    // compact sortOrder values on focus tasks
    compact_sort_order()?;

    Ok(())
}

fn clean_up_non_focus_tasks() -> Result<()> {
    Ok(())
}

fn assign_sort_order_where_missing() -> Result<()> {
    Ok(())
}

fn compact_sort_order() -> Result<()> {
    Ok(())
}