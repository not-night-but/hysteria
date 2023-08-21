use crate::{
    error::Error,
    models::{
        app::Repo,
        repo_data::{DrawType, GitExtDraw, RepoData},
    },
    AppState,
};
use git2::{BranchType, Repository};
use tauri::State;

// TODO (@day): move to models.rs
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct BranchData {
    name: String,
    tip_id: String,
    is_remote: bool,
}

#[tauri::command]
pub fn get_repo_branches(repo_path: String) -> Result<Vec<BranchData>, Error> {
    let repo = Repository::open(repo_path)?;
    let branches = repo
        .branches(None)
        .unwrap()
        .map(|branch| {
            let (br, br_type) = branch.unwrap();
            let is_remote = br_type == BranchType::Remote;
            let tip_id = br.get().peel_to_commit().unwrap().id();
            let name = br.name().unwrap().unwrap().to_owned();
            return BranchData {
                name,
                tip_id: tip_id.to_string(),
                is_remote,
            };
        })
        .collect::<Vec<BranchData>>();
    Ok(branches)
}

#[tauri::command]
pub fn get_user_repos(state: State<AppState>) -> Result<Vec<Repo>, Error> {
    if let Ok(conn) = state.conn.lock() {
        let repos = (*conn).get_user_repos()?;
        Ok(repos)
    } else {
        Err(Error::HysteriaError(
            "Error retrieving user repos".to_owned(),
        ))
    }
}

#[tauri::command]
pub fn add_repo(state: State<AppState>, repo: Repo) -> Result<(), Error> {
    if let Ok(conn) = state.conn.lock() {
        (*conn).add_repo(repo);
        Ok(())
    } else {
        Err(Error::HysteriaError("Failed to add repository".to_owned()))
    }
}

#[tauri::command]
pub fn get_repo_data(repo_path: String) -> Result<RepoData, Error> {
    let draw_type = DrawType::Ext(GitExtDraw::default());
    let repo = Repository::open(repo_path)?;
    let data = RepoData::new(repo, draw_type)?;

    Ok(data)
}
