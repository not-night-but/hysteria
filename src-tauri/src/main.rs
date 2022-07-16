#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::generate_handler;

use crate::handlers::{commits_handler::get_commits, repo_handler::get_repo_branches};

mod app;
mod error;
pub mod handlers;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(generate_handler![get_commits, get_repo_branches])
        .run(context)
        .expect("error while running tauri application");
}
