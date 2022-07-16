use crate::error::Error;
use git2::{BranchType, Repository};

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
