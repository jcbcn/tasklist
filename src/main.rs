use rusqlite::{params, Connection, Result};
use std::fs;
use structopt::StructOpt;
use chrono::{NaiveDateTime};

#[derive(StructOpt, Debug)]
enum Cli {
    Init,
    #[structopt(alias = "t")]
    Tasks(Tasks),
    #[structopt(alias = "l")]
    Lists(Lists),
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
    message: String,
    #[structopt(short, parse(try_from_str = parse_naivedatetime))]
    due: Option<NaiveDateTime>,
}

fn parse_naivedatetime(src: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(src,  "%Y-%m-%d %H:%M:%S")
}

#[derive(StructOpt, Debug)]
struct GetOperation {
    #[structopt(short)]
    due: Option<String>,
}

#[derive(Debug)]
struct Task {
    id: Option<u64>,
    name: String,
    due: Option<NaiveDateTime>
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let _ = handle_subcommand(args);

    Ok(())
}

fn setup_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE task (
            name            TEXT NOT NULL,
            due             DATETIME NULL
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
        Cli::Tasks(tasks) => match tasks {
            Tasks::Get(_cfg) => {
                let conn = Connection::open(".tasklist/default.db")?;
                let _ = setup_db(&conn);

                let task_iter = get_tasks(&conn);
                for task in &task_iter.unwrap() {
                    println!("[{}] {}", task.id.unwrap(), task.name);
                    if let Some(due) = task.due {
                        println!("[{}]", due);
                    }
                }
            }
            Tasks::Add(cfg) => {
                let conn = Connection::open(".tasklist/default.db")?;
                let _ = setup_db(&conn);

                add_task(&conn, cfg.message, cfg.due).expect("failed to add task");
            }
        },
        Cli::Lists(lists) => match lists {
            Lists::Get(_cfg) => {
                println!("Lists Get");
            }
            Lists::Add(_cfg) => {
                println!("Lists Add");
            }
        },
    }

    Ok(())
}

fn add_task(conn: &Connection, task: String, due: Option<NaiveDateTime>) -> Result<()> {
    let me = Task {
        id: None,
        name: task,
        due: due
    };
    conn.execute("INSERT INTO task (name, due) VALUES (?1, ?2)", params![me.name, me.due])?;

    Ok(())
}

fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT rowid, name, due FROM task")?;
    let rows = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            due: row.get(2)?
        })
    })?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}
