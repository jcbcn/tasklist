use structopt::clap::arg_enum;
use structopt::StructOpt;
use rusqlite::{params, Connection, Result};
use std::fs;

arg_enum! {
    #[derive(Debug)]
    enum Operation {
        Get,
        Add
    }
}

#[derive(StructOpt, Debug)]
enum Cli {
    Init,
    Tasks {
        #[structopt(possible_values = &Operation::variants(), case_insensitive = true)]
        operation: Option<Operation>
    }
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
        match cli {
            Cli::Init => {
                println!("Created dir");
                let _ = fs::create_dir(".tasklist");
            }
            Cli::Tasks{ operation } => {
                //println!("Get {}", operation.unwrap());
                match operation {
                    Some(Operation::Get) => {
                        let conn = Connection::open(".tasklist/default.db")?;
                        let _ = setup_db(&conn);

                        let task_iter = get_tasks(&conn);
                        for task in &task_iter.unwrap() {
                            println!("Found task {}", task);
                        }
                    },
                    Some(Operation::Add) => {
                        let conn = Connection::open(".tasklist/default.db")?;
                        let _ = setup_db(&conn);

                        add_task(&conn, "Sample task".to_string()).expect("failed to add task");
                    },
                    None => {}
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