// This file should be used for initializing the application resources on first run,
// validating resources on subsequent runs, and setting up the application state.

use rusqlite::params;

use crate::database_handler::DbConnection;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader},
    ops::Add,
    path::Path,
    sync::Mutex,
};

struct AppState {
    initialized: Mutex<bool>,
    conn: Mutex<Option<DbConnection>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            initialized: Mutex::new(false),
            conn: Mutex::new(None),
        }
    }
    pub fn check(&self, app_handle: &tauri::AppHandle) {
        let mut initialized = self.initialized.lock().unwrap();
        if !*initialized {
            let app_dir = app_handle
                .path_resolver()
                .app_dir()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .replace("\\\\?\\", "")
                .add(std::path::MAIN_SEPARATOR.to_string().as_str());

            fs::create_dir_all(app_dir.clone()).expect("Error creating directory");
            let user_memes_path = format!("{}hysteria.db", app_dir);

            let initialize_database = !Path::new(user_memes_path.as_str()).exists();

            // Create connections to database.
            let mut conn = self.conn.lock().unwrap();
            *conn = Some(DbConnection::new(user_memes_path.as_str()));

            // Initialize database if they were just created.
            if initialize_database {
                if let Some(conn) = &*conn {
                    let file_name = app_handle
                        .path_resolver()
                        .resource_dir()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                        .replace("\\\\?\\", "")
                        .add(
                            format!(
                                "{sep}assets{sep}initialize_db.sql",
                                sep = std::path::MAIN_SEPARATOR
                            )
                            .as_str(),
                        );

                    let file = OpenOptions::new().read(true).open(file_name).unwrap();
                    let mut reader = BufReader::new(file);
                    let mut buf = vec![];

                    // Use read_until to read until EOF.
                    reader
                        .read_until(u8::MIN, &mut buf)
                        .expect("Error reading initialize_db.sql");

                    // Convert vector of bytes to string.
                    let init_query = String::from_utf8(buf).unwrap();

                    for query in init_query.split(";") {
                        if query.trim().is_empty() {
                            continue;
                        }
                        conn.raw_query(query, params![]);
                    }
                }
            }
        }

        *initialized = true;
    }
}
