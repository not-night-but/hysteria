#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;

use git2::{BranchType, Repository, Sort};

mod app;
mod database_handler;

#[derive(serde::Serialize, serde::Deserialize)]
struct Commit {
    subject: String,
    body: String,
    author: Author,
    date: String,
    sha: String,
    parents: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Author {
    name: String,
    email: String,
}

#[tauri::command]
fn get_commits(app_handle: tauri::AppHandle) -> Vec<Commit> {
    println!("Attempting to open repository...");
    let repo = match Repository::open("/home/dsm6069/dev/DealSimple/") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to init: {}", e),
    };
    println!("Opened repository.");
    println!("Attempting to get commits...");
    if let Ok(mut revwalk) = repo.revwalk() {
        println!("Got revwalk.");

        repo.branches(Some(BranchType::Remote))
            .unwrap()
            .for_each(|branch| {
                let tip_id = branch.unwrap().0.get().peel_to_commit().unwrap().id();
                revwalk.push(tip_id);
            });
        revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL);
        // revwalk.push_head().unwrap();
        let commits = revwalk.take(500).map(|oid| {
            if let Ok(oid) = oid {
                // println!("Found commit {}", oid.clone());
                let commit = repo.find_commit(oid).unwrap();
                let subject = commit.summary().unwrap();
                let body = commit.body().unwrap_or("");
                let name = commit.author().name().unwrap().to_owned();
                let email = commit.author().email().unwrap().to_owned();
                let parents: Vec<String> = commit.parent_ids().map(|x| x.to_string()).collect();

                let date = commit.time();
                let sha = commit.id().to_string();

                return Commit {
                    subject: subject.to_string(),
                    body: body.to_string(),
                    author: Author { name, email },
                    date: format!("{:?}", date).to_string(),
                    sha: sha.to_string(),
                    parents,
                };
            }
            panic!("Failed to get oid");
        });
        let commits: Vec<Commit> = commits.collect();

        let fileref = File::create("/home/dsm6069/dev/commits.json").unwrap();
        serde_json::to_writer_pretty(fileref, &commits).unwrap();

        return commits;
    }
    panic!("Failed to get revwalk");
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler!(get_commits))
        .run(context)
        .expect("error while running tauri application");
}
