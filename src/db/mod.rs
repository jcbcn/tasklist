use rusqlite::ToSql;
use crate::core::Task;
use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};
use std::fs;

const DIR: &str = ".tasklist";
const DEFAULT_DB: &str = ".tasklist/default.db";

pub fn init() -> Result<()> {
    let _ = fs::create_dir(DIR);

    let conn = Connection::open(DEFAULT_DB)?;
    conn.execute(
        "CREATE TABLE task (
            name            TEXT NOT NULL,
            due             DATE NULL,
            completed       BOOLEAN DEFAULT(FALSE)
        )",
        [],
    )?;

    Ok(())
}

pub fn get_tasks(due: Option<NaiveDate>) -> Result<Vec<Task>> {
    let conn = Connection::open(DEFAULT_DB)?;

    let mut query = "SELECT rowid, name, due, completed FROM task WHERE completed = FALSE".to_owned();
    let mut params:Vec<&dyn ToSql> = vec![];

    if due.is_some() {
        query = format!("{} AND due = ?", query);
        params.push(&due)
    }

    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map(&*params, |row| {
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

pub fn add_task(task: String, due: Option<NaiveDate>) -> Result<()> {
    let conn = Connection::open(DEFAULT_DB)?;

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
