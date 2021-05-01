use structopt::StructOpt;
use rusqlite::{params, Connection, Result};
use std::fs;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    operation: Option<Operation>,
    object: String
}

#[derive(StructOpt)]
enum Operation {
    Get,
    Add,
    Init
}

#[derive(Debug)]
struct Task {
    name: String
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let _ = handle_subcommand(args);

    Ok(())
}

fn setup_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE task (
                  name            TEXT NOT NULL
                  )",
        [],
    )?;

    Ok(())
}

fn handle_subcommand(cli: Cli) -> Result<()> {
    if let Some(op) = cli.operation{
        match op {
            Operation::Get => {
                let conn = Connection::open(".tasklist/default.db")?;
                let _ = setup_db(&conn);

                println!("Get {}", cli.object);
                let task_iter = get_tasks(&conn);
                for task in &task_iter.unwrap() {
                    println!("Found task {}", task);
                }
            },
            Operation::Add => {
                let conn = Connection::open(".tasklist/default.db")?;
                let _ = setup_db(&conn);

                println!("Add {}", cli.object);
                add_task(&conn, "Sample task".to_string()).expect("failed to add task");
            },
            Operation::Init => {
                println!("Created dir");
                let _ = fs::create_dir(".tasklist");
            }
        }
    }

    Ok(())
}

fn add_task(conn: &Connection, task: String) -> Result<()>{
    let me = Task {
        name: task
    };
    conn.execute(
        "INSERT INTO task (name) VALUES (?1)",
        params![me.name],
    )?;

    Ok(())
}

fn get_tasks(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT rowid, name FROM task")?;
    let rows = stmt.query_map([], |row| row.get(1))?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}