use structopt::StructOpt;
use rusqlite::{params, Connection, Result};
use std::fs;

#[derive(StructOpt, Debug)]
enum Cli {
    Init,
    #[structopt(alias = "t")]
    Tasks(Tasks),
    #[structopt(alias = "l")]
    Lists(Lists)
}

#[derive(StructOpt, Debug)]
enum Tasks {
    #[structopt(name = "get", alias = "g")]
    Get(GetOperation),
    #[structopt(name = "add", alias = "a")]
    Add(AddOperation),
}

#[derive(StructOpt, Debug)]
enum Lists {
    #[structopt(name = "get", alias = "g")]
    Get(GetOperation),
    #[structopt(name = "add", alias = "a")]
    Add(AddOperation),
}

#[derive(StructOpt, Debug)]
struct AddOperation {
    #[structopt(short)]
    message: String
}

#[derive(StructOpt, Debug)]
struct GetOperation {
    #[structopt(short)]
    due: Option<String>
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
            Cli::Tasks(tasks) => {
                match tasks {
                    Tasks::Get(_cfg) => {
                        let conn = Connection::open(".tasklist/default.db")?;
                        let _ = setup_db(&conn);

                        let task_iter = get_tasks(&conn);
                        for task in &task_iter.unwrap() {
                            println!("Task: {}", task);
                        }
                    },
                    Tasks::Add(cfg) => {
                        let conn = Connection::open(".tasklist/default.db")?;
                        let _ = setup_db(&conn);

                        add_task(&conn, cfg.message).expect("failed to add task");
                    }
                }
            }
            Cli::Lists(lists) => {
                match lists {
                    Lists::Get(_cfg) => {
                        println!("Lists Get");
                    },
                    Lists::Add(_cfg) => {
                        println!("Lists Add");
                    }
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