use std::collections::{HashMap, HashSet};

use ambassador::{delegatable_trait, Delegate};
use git2::{BranchType, Oid, Repository, Sort};

use crate::{error::Error, models::branch::BranchSvgProps};

use super::{
    branch::Branch,
    commit::{Commit, Head},
};

#[delegatable_trait]
pub trait Draw {
    fn draw(&self);
}

pub struct GitGraphDraw {}
pub struct GitExtDraw {}

impl Draw for GitGraphDraw {
    fn draw(&self) {
        todo!()
    }
}
impl Draw for GitExtDraw {
    fn draw(&self) {
        todo!()
    }
}

#[derive(Delegate)]
#[delegate(Draw)]
pub enum DrawType {
    Ext(GitExtDraw),
    Graph(GitGraphDraw),
}

pub struct RepoData {
    repository: Repository,
    pub commits: Vec<Commit>,
    pub indices: HashMap<Oid, usize>,
    pub all_branches: Vec<Branch>,
    pub branches: Vec<usize>,
    pub tags: Vec<usize>,
    pub head: Head,
    pub stashes: HashSet<Oid>,
}

impl RepoData {
    pub fn new(mut repo: Repository, draw_type: DrawType) -> Result<Self, Error> {
        let mut stashes = HashSet::new();
        repo.stash_foreach(|_, _, oid| {
            stashes.insert(*oid);
            true
        })?;

        let mut revwalk = repo.revwalk()?;

        revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL)?;

        revwalk.push_glob("*")?;

        // get head info
        let head = Head::new(&repo)?;

        // create commits vec
        let mut commits = Vec::new();

        // create indices map
        let mut indices = HashMap::new();

        // walk through revwalk
        // if oid is not a stash
        // find the commit, create the commit object, and push the index to the array
        let mut index = 0;
        for oid in revwalk {
            if let Ok(oid) = oid {
                if !stashes.contains(&oid) {
                    let commit = repo.find_commit(oid)?;

                    commits.push(Commit::new(&commit)?);
                    indices.insert(oid, index);
                    index += 1;
                }
            }
        }

        Self::assign_children(&mut commits, &indices);

        let mut all_branches = Self::assign_branches(&repo, &mut commits, &indices)?;

        draw_type.draw();
        todo!()
    }

    fn assign_children(commits: &mut [Commit], indices: &HashMap<Oid, usize>) {
        for index in 0..commits.len() {
            let (oid, parents) = {
                let commit = &commits[index];
                (commit.oid, commit.parents)
            };
            for parent in parents {
                if let Some(par_index) = parent.and_then(|oid| indices.get(&oid)) {
                    commits[*par_index].children.push(oid);
                }
            }
        }
    }

    fn assign_branches(
        repo: &Repository,
        commits: &mut [Commit],
        indices: &HashMap<Oid, usize>,
    ) -> Result<Vec<Branch>, Error> {
        let mut branch_index = 0;
        let mut branches = Self::create_branches(repo, commits, indices)?;
        todo!()
    }

    fn create_branches(
        repo: &Repository,
        commits: &[Commit],
        indices: &HashMap<Oid, usize>,
    ) -> Result<Vec<Branch>, Error> {
        // TODO (@day): we need settings at some point to choose whether we want local, remote, or both
        let filter = Some(BranchType::Local);

        let mut counter = 0;
        let existing_branches = repo.branches(filter)?.collect::<Result<Vec<_>, _>>()?;

        let mut valid_branches = existing_branches
            .iter()
            .filter_map(|(branch, branch_type)| {
                branch.get().name().and_then(|name| {
                    branch.get().target().map(|target| {
                        counter += 1;
                        let start_index = match branch_type {
                            BranchType::Local => 11,
                            BranchType::Remote => 13,
                        };
                        let name = &name[start_index..];
                        let end_index = indices.get(&target).cloned();
                        let colour = "#eb6d5d".to_string();

                        Ok(Branch::new(
                            target,
                            None,
                            name.to_string(),
                            0u8,
                            &BranchType::Remote == branch_type,
                            false,
                            false,
                            BranchSvgProps::new(0, colour),
                            end_index,
                        ))
                    })
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        valid_branches.sort_by_cached_key(|branch| (branch.persistence, !branch.is_merged));

        let mut tags = Vec::new();

        repo.tag_foreach(|oid, name| {
            tags.push((oid, name.to_vec()));
            true
        })?;

        for (oid, name) in tags {
            let name = std::str::from_utf8(&name[5..]).map_err(|err| err.to_string())?;

            let target = repo
                .find_tag(oid)
                .map(|tag| tag.target_id())
                .or_else(|_| repo.find_commit(oid).map(|_| oid));

            if let Ok(target_oid) = target {
                if let Some(target_index) = indices.get(&target_oid) {
                    counter += 1;
                    let colour = "#eb6d5d".to_string();
                    let tag_info = Branch::new(
                        target_oid,
                        None,
                        name.to_string(),
                        1u8,
                        false,
                        false,
                        true,
                        BranchSvgProps::new(1, colour),
                        Some(*target_index),
                    );
                    valid_branches.push(tag_info);
                }
            }
        }

        Ok(valid_branches)
    }
}
