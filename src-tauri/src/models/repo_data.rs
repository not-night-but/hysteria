use std::collections::{HashMap, HashSet};

use ambassador::{delegatable_trait, Delegate};
use git2::{BranchType, Oid, Repository, Sort};
use serde::ser::SerializeStruct;

use crate::{error::Error, models::branch::BranchSvgProps};

use super::{
    branch::{Branch, Point},
    commit::{Commit, Head},
};

const ORIGIN: &str = "origin/";

#[delegatable_trait]
pub trait Draw {
    fn draw(
        &self,
        commits: &mut Vec<Commit>,
        all_branches: &mut Vec<Branch>,
        indices: &HashMap<Oid, usize>,
        branches: &Vec<usize>,
    );
}

#[derive(Default)]
pub struct GitGraphDraw {}
#[derive(Default)]
pub struct GitExtDraw {}

impl Draw for GitGraphDraw {
    fn draw(
        &self,
        commits: &mut Vec<Commit>,
        all_branches: &mut Vec<Branch>,
        indices: &HashMap<Oid, usize>,
        branches: &Vec<usize>,
    ) {
        // "draw" the graph in the more spread out way that git-graph does
        todo!()
    }
}
impl Draw for GitExtDraw {
    fn draw(
        &self,
        commits: &mut Vec<Commit>,
        all_branches: &mut Vec<Branch>,
        // TODO (day): do we need these?
        indices: &HashMap<Oid, usize>,
        branches: &Vec<usize>,
    ) {
        let mut unavailable_points: HashMap<usize, u32> = HashMap::new();
        let mut branches_commits: HashMap<usize, Vec<Oid>> = HashMap::new();
        for commit in commits.iter() {
            if let Some(trace) = commit.branch_trace {
                if branches_commits.get(&trace).is_none() {
                    branches_commits.insert(trace, vec![]);
                }
                if let Some(commits) = branches_commits.get_mut(&trace) {
                    commits.push(commit.oid);
                }
            }
        }

        // "draw" the graph in hysteria's style, inspired by git extensions
        for branch_idx in branches {
            let branch = &mut all_branches[*branch_idx];
            if let Some(branch_commits) = branches_commits.get(branch_idx) {
                for oid in branch_commits {
                    // TODO (@day): this could be dangerous, but it shouldn't be possible to get an oid we aren't tracking
                    let index = indices.get(oid).unwrap();
                    let vertex_x = unavailable_points.get(index).unwrap_or(&0u32).clone();
                    let commit = &mut commits[index.clone()];
                    // add the dot for the commit
                    branch.svg_props.add_vertex(
                        vertex_x,
                        index.clone() as u32,
                        commit.oid.to_string(),
                        commit.is_merge,
                    );
                    commit.register_point(Point {
                        x: vertex_x,
                        y: index.clone() as u32,
                    });
                    let commit = commits[index.clone()].clone();
                    // first parent is on the same branch (hopefully), draw a line to it
                    if let Some(first_parent) = commit.parents[0] {
                        if let Some(par_idx) = indices.get(&first_parent) {
                            let mut last_x: Option<u32> = None;
                            let parent = &mut commits[*par_idx];

                            if parent.branch_trace == commit.branch_trace {
                                for curr_y in *index..=*par_idx {
                                    if unavailable_points.get(&curr_y).is_none() {
                                        unavailable_points.insert(curr_y, 0);
                                    }
                                    let curr_x = *unavailable_points.get(&curr_y).unwrap_or(&0u32);
                                    if let Some(last_x) = last_x {
                                        branch.svg_props.add_line(
                                            last_x,
                                            (curr_y - 1) as u32,
                                            curr_x,
                                            curr_y as u32,
                                        );
                                    }
                                    last_x = Some(curr_x);
                                    // a point has been placed at the current y, so we increment the x counter
                                    if curr_y != *par_idx {
                                        if let Some(x) = unavailable_points.get_mut(&curr_y) {
                                            *x += 1;
                                        }
                                    }
                                }
                            } else {
                                parent.add_connection(Point {
                                    x: vertex_x,
                                    y: index.clone() as u32,
                                });
                            }
                        }
                    }
                    // create connection to other branch if the other parent exists
                    for p in 1..commit.parents.len() {
                        if let Some(par_oid) = &commit.parents[p] {
                            if let Some(par_idx) = indices.get(par_oid) {
                                let parent = &mut commits[*par_idx];
                                parent.add_connection(Point {
                                    x: vertex_x,
                                    y: index.clone() as u32,
                                });
                            }
                        }
                    }
                }
            }
        }

        // walk through commits drawing lines for connections
        for branch_idx in branches {
            let branch = &mut all_branches[*branch_idx];
            if let Some(branch_commits) = branches_commits.get(branch_idx) {
                let mut last_x: Option<u32> = None;
                for oid in branch_commits {
                    let index = indices.get(oid).unwrap();
                    let commit = &commits[*index];
                    for connection in &commit.connections {
                        if let Some(point) = &commit.point {
                            if point.y - connection.y > 1 {
                                for curr_y in point.y..=connection.y {
                                    if unavailable_points.get(&(curr_y as usize)).is_none() {
                                        unavailable_points.insert(curr_y as usize, 0);
                                    }
                                    let curr_x = *unavailable_points
                                        .get(&(curr_y as usize))
                                        .unwrap_or(&0u32);

                                    if let Some(last_x) = last_x {
                                        branch.svg_props.add_line(
                                            last_x,
                                            curr_y - 1,
                                            curr_x,
                                            curr_y,
                                        );
                                    }
                                    last_x = Some(curr_x);
                                }
                            } else {
                                branch
                                    .svg_props
                                    .add_line_from_points(connection.clone(), point.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Delegate)]
#[delegate(Draw)]
pub enum DrawType {
    Ext(GitExtDraw),
    Graph(GitGraphDraw),
}

pub struct RepoData {
    pub commits: Vec<Commit>,
    pub indices: HashMap<Oid, usize>,
    pub all_branches: Vec<Branch>,
    pub branches: Vec<usize>,
    pub tags: Vec<usize>,
    pub head: Head,
    pub stashes: HashSet<Oid>,
}

impl serde::Serialize for RepoData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RepoData", 7)?;
        state.serialize_field("commits", &self.commits)?;
        let mut indices = HashMap::new();
        for oid in self.indices.keys() {
            if let Some(value) = self.indices.get(oid) {
                indices.insert(oid.to_string(), value);
            }
        }
        state.serialize_field("indices", &indices)?;
        state.serialize_field("all_branches", &self.all_branches)?;
        state.serialize_field("branches", &self.branches)?;
        state.serialize_field("tags", &self.tags)?;
        state.serialize_field("head", &self.head)?;
        let mut stashes = HashSet::new();
        for stash in self.stashes.iter() {
            stashes.insert(stash.to_string());
        }
        state.serialize_field("stashes", &stashes)?;
        state.end()
    }
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
        Self::assign_sources_targets(&commits, &indices, &mut all_branches);

        // TODO (@day): something here needs to change depending on if we're using gitext drawing or gitgraph drawing
        Self::assign_branch_columns(&commits, &indices, &mut all_branches, false, true);

        let mut filtered_commits: Vec<Commit> = commits
            .into_iter()
            .filter(|commit| commit.branch_trace.is_some())
            .collect();

        let filtered_indices: HashMap<Oid, usize> = filtered_commits
            .iter()
            .enumerate()
            .map(|(idx, commit)| (commit.oid, idx))
            .collect();

        let index_map: HashMap<usize, Option<&usize>> = indices
            .iter()
            .map(|(oid, index)| (*index, filtered_indices.get(oid)))
            .collect();

        for branch in all_branches.iter_mut() {
            if let Some(mut start_idx) = branch.range.0 {
                let mut idx0 = index_map[&start_idx];
                while idx0.is_none() {
                    start_idx += 1;
                    idx0 = index_map[&start_idx];
                }
                branch.range.0 = Some(*idx0.unwrap());
            }
            if let Some(mut end_idx) = branch.range.1 {
                let mut idx0 = index_map[&end_idx];
                while idx0.is_none() {
                    end_idx -= 1;
                    idx0 = index_map[&end_idx];
                }
                branch.range.1 = Some(*idx0.unwrap());
            }
        }
        let branches = all_branches
            .iter()
            .enumerate()
            .filter_map(|(idx, br)| {
                if !br.is_merged && !br.is_tag {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        let tags = all_branches
            .iter()
            .enumerate()
            .filter_map(|(idx, br)| {
                if !br.is_merged && br.is_tag {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        draw_type.draw(
            &mut filtered_commits,
            &mut all_branches,
            &filtered_indices,
            &branches,
        );

        Ok(RepoData {
            commits: filtered_commits,
            indices: filtered_indices,
            all_branches,
            branches,
            tags,
            head,
            stashes,
        })
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
        let mut index_map: Vec<_> = (0..branches.len())
            .map(|old_idx| {
                let (target, is_tag, is_merged) = {
                    let branch = &branches[old_idx];
                    (branch.target, branch.is_tag, branch.is_merged)
                };
                if let Some(&index) = &indices.get(&target) {
                    let commit = &mut commits[index];
                    if is_tag {
                        commit.tags.push(old_idx);
                    } else if !is_merged {
                        commit.branches.push(old_idx);
                    }
                    let oid = commit.oid;
                    let any_assigned =
                        Self::trace_branch(repo, commits, indices, &mut branches, oid, old_idx)
                            .unwrap_or(false);

                    if any_assigned || !is_merged {
                        branch_index += 1;
                        Some(branch_index - 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let mut commit_count = vec![0; branches.len()];
        for commit in commits.iter_mut() {
            if let Some(trace) = commit.branch_trace {
                commit_count[trace] += 1;
            }
        }

        let mut count_skipped = 0;
        for (idx, branch) in branches.iter().enumerate() {
            if let Some(mapped) = index_map[idx] {
                if commit_count[idx] == 0 && branch.is_merged && !branch.is_tag {
                    index_map[idx] = None;
                    count_skipped += 1;
                } else {
                    index_map[idx] = Some(mapped - count_skipped);
                }
            }
        }

        for commit in commits.iter_mut() {
            if let Some(trace) = commit.branch_trace {
                commit.branch_trace = index_map[trace];
                for br in commit.branches.iter_mut() {
                    *br = index_map[*br].unwrap();
                }
                for tag in commit.tags.iter_mut() {
                    *tag = index_map[*tag].unwrap();
                }
            }
        }

        let branches: Vec<_> = branches
            .into_iter()
            .enumerate()
            .filter_map(|(arr_index, branch)| {
                if index_map[arr_index].is_some() {
                    Some(branch)
                } else {
                    None
                }
            })
            .collect();

        Ok(branches)
    }

    fn create_branches(
        repo: &Repository,
        commits: &[Commit],
        indices: &HashMap<Oid, usize>,
    ) -> Result<Vec<Branch>, Error> {
        // TODO (@day): we need settings at some point to choose whether we want local, remote, or both
        let filter = None;

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

    fn trace_branch<'repo>(
        repo: &'repo Repository,
        commits: &mut [Commit],
        indices: &HashMap<Oid, usize>,
        branches: &mut [Branch],
        commit_oid: Oid,
        branch_index: usize,
    ) -> Result<bool, Error> {
        let mut curr_oid = commit_oid;
        let mut prev_index: Option<usize> = None;
        let mut start_index: Option<i32> = None;
        let mut any_assigned = false;
        while let Some(index) = indices.get(&curr_oid) {
            let commit = &mut commits[*index];
            if let Some(old_trace) = commit.branch_trace {
                let (old_name, old_colour, old_range) = {
                    let old_branch = &branches[old_trace];
                    (
                        &old_branch.name.clone(),
                        old_branch.svg_props.colour.clone(),
                        old_branch.range,
                    )
                };

                let new_name = &branches[branch_index].name;
                let old_end = old_range.0.unwrap_or(0);
                let new_end = branches[branch_index].range.0.unwrap_or(0);
                if new_name == old_name && old_end >= new_end {
                    let old_branch = &mut branches[old_trace];
                    if let Some(old_end) = old_range.1 {
                        if index > &old_end {
                            old_branch.range = (None, None);
                        } else {
                            old_branch.range = (Some(*index), old_branch.range.1);
                        }
                    } else {
                        old_branch.range = (Some(*index), old_branch.range.1);
                    }
                } else {
                    let branch = &mut branches[branch_index];
                    if branch.name.starts_with(ORIGIN) && branch.name[7..] == old_name[..] {
                        branch.svg_props.colour = old_colour;
                    }
                    match prev_index {
                        None => start_index = Some(*index as i32 - 1),
                        Some(prev_index) => {
                            if commits[prev_index].is_merge {
                                let mut temp_index = prev_index;
                                for sibling_oid in &commits[*index].children {
                                    if sibling_oid != &curr_oid {
                                        let sibling_index = indices[sibling_oid];
                                        if sibling_index > temp_index {
                                            temp_index = sibling_index;
                                        }
                                    }
                                }
                                start_index = Some(temp_index as i32);
                            } else {
                                start_index = Some(*index as i32 - 1);
                            }
                        }
                    }
                    break;
                }
            }

            commit.branch_trace = Some(branch_index);
            any_assigned = true;

            let commit = repo.find_commit(curr_oid)?;
            match commit.parent_count() {
                0 => {
                    start_index = Some(*index as i32);
                    break;
                }
                _ => {
                    prev_index = Some(*index);
                    curr_oid = commit.parent_id(0)?;
                }
            }
        }

        let branch = &mut branches[branch_index];
        if let Some(end) = branch.range.0 {
            if let Some(start_index) = start_index {
                if start_index < end as i32 {
                    branch.range = (None, None);
                } else {
                    branch.range = (branch.range.0, Some(start_index as usize));
                }
            } else {
                branch.range = (branch.range.0, None);
            }
        } else {
            branch.range = (branch.range.0, start_index.map(|si| si as usize));
        }
        Ok(any_assigned)
    }

    fn assign_sources_targets(
        commits: &[Commit],
        indices: &HashMap<Oid, usize>,
        branches: &mut [Branch],
    ) {
        for index in 0..branches.len() {
            let target_branch_idx = branches[index]
                .merge_target
                .and_then(|oid| indices.get(&oid))
                .and_then(|idx| commits.get(*idx))
                .and_then(|commit| commit.branch_trace);

            branches[index].target_branch = target_branch_idx;

            let group = target_branch_idx
                .and_then(|trace| branches.get(trace))
                .map(|br| br.svg_props.order_group);

            branches[index].svg_props.target_order_group = group;
        }

        for commit in commits {
            let mut max_par_order = None;
            let mut source_branch_id = None;
            for par_oid in commit.parents.iter() {
                let par_commit = par_oid
                    .and_then(|oid| indices.get(&oid))
                    .and_then(|idx| commits.get(*idx));
                if let Some(par_commit) = par_commit {
                    if par_commit.branch_trace != commit.branch_trace {
                        if let Some(trace) = par_commit.branch_trace {
                            source_branch_id = Some(trace);
                        }

                        let group = par_commit
                            .branch_trace
                            .and_then(|trace| branches.get(trace))
                            .map(|br| br.svg_props.order_group);
                        if let Some(gr) = max_par_order {
                            if let Some(p_group) = group {
                                if p_group > gr {
                                    max_par_order = group
                                }
                            }
                        } else {
                            max_par_order = group;
                        }
                    }
                }
            }
            let branch = commit
                .branch_trace
                .and_then(|trace| branches.get_mut(trace));
            if let Some(branch) = branch {
                if let Some(order) = max_par_order {
                    branch.svg_props.source_order_group = Some(order);
                }
                if let Some(source_id) = source_branch_id {
                    branch.source_branch = Some(source_id);
                }
            }
        }
    }

    fn assign_branch_columns(
        commits: &[Commit],
        indices: &HashMap<Oid, usize>,
        branches: &mut [Branch],
        shortest_first: bool,
        forward: bool,
    ) {
        let mut occupied: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![]; 4];

        let length_sort_factor = if shortest_first { 1 } else { -1 };
        let start_sort_factor = if forward { 1 } else { -1 };

        let mut branches_sort: Vec<_> = branches
            .iter()
            .enumerate()
            .filter(|(_idx, br)| br.range.0.is_some() || br.range.1.is_some())
            .map(|(idx, br)| {
                (
                    idx,
                    br.range.0.unwrap_or(0),
                    br.range.1.unwrap_or(branches.len() - 1),
                    br.svg_props.source_order_group.unwrap_or(4),
                    br.svg_props.target_order_group.unwrap_or(4),
                )
            })
            .collect();

        branches_sort.sort_by_cached_key(|tup| {
            (
                std::cmp::max(tup.3, tup.4),
                (tup.2 as i32 - tup.1 as i32) * length_sort_factor,
                tup.1 as i32 * start_sort_factor,
            )
        });

        for (branch_idx, start, end, _, _) in branches_sort {
            let branch = &branches[branch_idx];
            let group = branch.svg_props.order_group;
            let group_occ = &mut occupied[group];

            let align_right = branch
                .source_branch
                .map(|src| branches[src].svg_props.order_group > branch.svg_props.order_group)
                .unwrap_or(false)
                || branch
                    .target_branch
                    .map(|trg| branches[trg].svg_props.order_group > branch.svg_props.order_group)
                    .unwrap_or(false);

            let len = group_occ.len();
            let mut found = len;
            for i in 0..len {
                let index = if align_right { len - i - 1 } else { i };
                let column_occ = &group_occ[index];
                let mut occ = false;
                for (s, e) in column_occ {
                    if start <= *e && end >= *s {
                        occ = true;
                        break;
                    }
                }
                if !occ {
                    if let Some(merge_trace) = branch
                        .merge_target
                        .and_then(|t| indices.get(&t))
                        .and_then(|t_idx| commits[*t_idx].branch_trace)
                    {
                        let merge_branch = &branches[merge_trace];
                        if merge_branch.svg_props.order_group == branch.svg_props.order_group {
                            if let Some(merge_column) = merge_branch.svg_props.column {
                                if merge_column == index {
                                    occ = true;
                                }
                            }
                        }
                    }
                }
                if !occ {
                    found = index;
                    break;
                }
            }

            let branch = &mut branches[branch_idx];
            branch.svg_props.column = Some(found);
            if found == group_occ.len() {
                group_occ.push(vec![]);
            }
            group_occ[found].push((start, end));
        }

        let group_offset: Vec<usize> = occupied
            .iter()
            .scan(0, |acc, group| {
                *acc += group.len();
                Some(*acc)
            })
            .collect();

        for branch in branches {
            if let Some(column) = branch.svg_props.column {
                let offset = if branch.svg_props.order_group == 0 {
                    0
                } else {
                    group_offset[branch.svg_props.order_group - 1]
                };
                branch.svg_props.column = Some(column + offset);
            }
        }
    }
}
