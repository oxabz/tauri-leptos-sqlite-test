// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rusqlite::Connection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn increment_counter(conn: tauri::State<Mutex<Connection>>) -> i32 {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT count FROM counter").unwrap();
    let mut rows = stmt.query([]).unwrap();

    let count: i32 = rows.next().unwrap().unwrap().get(0).unwrap();

    conn.execute(
        "UPDATE counter SET count = ?",
        [count + 1],
    ).unwrap();

    count + 1
}

#[tauri::command]
fn decrement_counter(conn: tauri::State<Mutex<Connection>>) -> i32 {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT count FROM counter").unwrap();
    let mut rows = stmt.query([]).unwrap();

    let count: i32 = rows.next().unwrap().unwrap().get(0).unwrap();

    conn.execute(
        "UPDATE counter SET count = ?",
        [count - 1],
    ).unwrap();

    count - 1
}

#[tauri::command]
fn get_counter(conn: tauri::State<Mutex<Connection>>) -> i32 {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare("SELECT count FROM counter").unwrap();
    let mut rows = stmt.query([]).unwrap();

    rows.next().unwrap().unwrap().get(0).unwrap()
}

fn db_init(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS counter (
            id INTEGER PRIMARY KEY,
            count INTEGER NOT NULL
        )",
        [],
    ).unwrap();

    conn.execute(
        "INSERT INTO counter (count) VALUES (?)",
        [0],
    ).unwrap();
}

fn main() {
    let conn = rusqlite::Connection::open("../../test.db").unwrap();
    db_init(&conn);
    let conn = Mutex::new(conn);

    tauri::Builder::default()
        .manage(conn)
        .invoke_handler(tauri::generate_handler![greet, increment_counter, decrement_counter, get_counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
