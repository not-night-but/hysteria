use crate::error::Error;
use git2::{Repository, Sort, Time};
use std::{cell::RefCell, fs::File, rc::Rc};
use time::{format_description, OffsetDateTime};

// TODO (@day): move to models.rs
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Commit {
    subject: String,
    body: String,
    author: Author,
    date: String,
    sha: String,
    parents: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Author {
    name: String,
    email: String,
}

#[tauri::command]
pub fn get_commits(repo_path: String) -> Result<Vec<Commit>, Error> {
    let repo = Repository::open(repo_path)?;

    if let Ok(mut revwalk) = repo.revwalk() {
        repo.branches(None).unwrap().for_each(|branch| {
            let tip_id = branch.unwrap().0.get().peel_to_commit().unwrap().id();
            revwalk.push(tip_id);
        });
        revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL)?;

        let err: Rc<RefCell<Option<Error>>> = Rc::new(RefCell::new(None));
        let captured_err = err.clone();
        let commits = revwalk.take(500).map(|oid| {
            if let Ok(oid) = oid {
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
                    date: get_time(&date),
                    sha: sha.to_string(),
                    parents,
                };
            }
            *captured_err.borrow_mut() = Some(Error::from("Failed to get commit"));
            return Commit::default();
        });

        if let Some(err) = captured_err.take() {
            return Err(err);
        }

        let commits: Vec<Commit> = commits.collect();

        let fileref = File::create("/home/notnight/dev/commits.json").unwrap();
        serde_json::to_writer_pretty(fileref, &commits).unwrap();

        return Ok(commits);
    }
    Err(Error::from("Failed to get revwalk"))
}

fn get_time(time: &Time) -> String {
    let offset = OffsetDateTime::from_unix_timestamp(time.seconds()).unwrap();
    let format =
        format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]Z").unwrap();
    return offset.format(&format).unwrap();
}
