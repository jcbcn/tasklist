use structopt::StructOpt;
use rusqlite::{params, Connection, Result};

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    operation: Option<Operation>,
    object: String
}

#[derive(StructOpt)]
enum Operation {
    Get,
    Add
}

#[derive(Debug)]
struct Task {
    id: i32,
    name: String
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let conn = Connection::open("default.db")?;

    let _ = setup_db(&conn);
    handle_subcommand(&conn, args);

    Ok(())
}

fn setup_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE task (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
        [],
    )?;

    Ok(())
}

fn handle_subcommand(conn: &Connection, cli: Cli) {
    if let Some(op) = cli.operation{
        match op {
            Operation::Get => {
                println!("Get {}", cli.object);
                let task_iter = get_tasks(conn);
                for task in task_iter {
                    println!("Found task {:?}", task);
                }
            },
            Operation::Add => {
                println!("Add {}", cli.object);
                add_task(&conn, "Sample task".to_string()).expect("failed to add task");
            }
        }
    }
}

fn add_task(conn: &Connection, task: String) -> Result<()>{
    let me = Task {
        id: 0,
        name: task
    };
    conn.execute(
        "INSERT INTO task (name) VALUES (?1)",
        params![me.name],
    )?;

    Ok(())
}

fn get_tasks<'a>(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT id, name FROM task")?;
    let rows = stmt.query_map([], |row| row.get(1))?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}