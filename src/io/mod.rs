use crate::core::Task;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection, Result};
use std::fs;

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

pub fn init() {
    let _ = fs::create_dir(".tasklist");
}

pub fn get_tasks() -> Result<Vec<Task>> {
    let conn = Connection::open(".tasklist/default.db")?;
    let _ = setup_db(&conn);

    let mut stmt = conn.prepare("SELECT rowid, name, due FROM task")?;
    let rows = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            due: row.get(2)?,
        })
    })?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}

pub fn add_task(task: String, due: Option<NaiveDateTime>) -> Result<()> {
    let conn = Connection::open(".tasklist/default.db")?;
    let _ = setup_db(&conn);

    let me = Task {
        id: None,
        name: task,
        due: due,
    };
    conn.execute(
        "INSERT INTO task (name, due) VALUES (?1, ?2)",
        params![me.name, me.due],
    )?;

    Ok(())
}
