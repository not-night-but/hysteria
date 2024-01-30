#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use handlers::database_handler::DbConnection;
use tauri::{generate_handler, Manager};

use crate::handlers::{
    commits_handler::get_commits,
    repo_handler::{add_repo, get_repo_branches, get_repo_data, get_user_repos},
};

mod error;
pub mod handlers;
mod models;

pub struct AppState {
    conn: Mutex<DbConnection>,
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            if let Some(res_dir) = app.path_resolver().resource_dir().unwrap().to_str() {
                let db_path = format!("{}{}app.db", res_dir, std::path::MAIN_SEPARATOR_STR);
                app.manage(AppState {
                    conn: Mutex::new(DbConnection::new(&db_path)),
                });
            }
            return Ok(());
        })
        .invoke_handler(generate_handler![
            get_commits,
            get_repo_branches,
            get_user_repos,
            add_repo,
            get_repo_data
        ])
        .run(context)
        .expect("error while running tauri application");
}
