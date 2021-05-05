use crate::core::Task;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::io;

const DIR: &str = ".tasklist";
const DEFAULT_DB: &str = ".tasklist/default.db";

fn setup_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE task (
            name            TEXT NOT NULL,
            due             DATETIME NULL,
            completed       BOOLEAN DEFAULT(FALSE)
        )",
        [],
    )?;

    Ok(())
}

pub fn init() -> io::Result<()> {
    fs::create_dir(DIR)
}

pub fn get_tasks() -> Result<Vec<Task>> {
    let conn = Connection::open(DEFAULT_DB)?;
    let _ = setup_db(&conn);

    let mut stmt = conn.prepare("SELECT rowid, name, due, completed FROM task WHERE completed = FALSE")?;
    let rows = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            due: row.get(2)?,
            completed: row.get(3)?,
        })
    })?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}

pub fn add_task(task: String, due: Option<NaiveDateTime>) -> Result<()> {
    let conn = Connection::open(DEFAULT_DB)?;
    let _ = setup_db(&conn);

    let me = Task {
        id: None,
        name: task,
        due: due,
        completed: false,
    };
    conn.execute(
        "INSERT INTO task (name, due) VALUES (?1, ?2)",
        params![me.name, me.due],
    )?;

    Ok(())
}

pub fn complete_task(id: u64) -> Result<()> {
    let conn = Connection::open(DEFAULT_DB)?;
    conn.execute(
        "UPDATE task SET completed = TRUE WHERE ROWID = ?",
        params![id],
    )?;

    Ok(())
}
