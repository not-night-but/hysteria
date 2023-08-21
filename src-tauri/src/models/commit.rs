use git2::{Oid, Repository};
use serde::ser::SerializeStruct;

use crate::error::Error;

use super::branch::Point;

#[derive(Clone)]
pub struct Commit {
    pub oid: Oid,
    pub is_merge: bool,
    pub parents: [Option<Oid>; 2],
    pub children: Vec<Oid>,
    pub branches: Vec<usize>,
    pub tags: Vec<usize>,
    // index of the branch the commit is on?
    pub branch_trace: Option<usize>,

    // for drawing things
    pub next_parent: usize,
    pub point: Option<Point>,
    pub connections: Vec<Point>,
}

impl serde::Serialize for Commit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Commit", 10)?;
        state.serialize_field("oid", &self.oid.to_string())?;
        state.serialize_field("is_merge", &self.is_merge)?;
        state.serialize_field(
            "parents",
            &self.parents.map(|p| {
                if let Some(o) = p {
                    Some(o.to_string())
                } else {
                    None
                }
            }),
        )?;
        state.serialize_field(
            "children",
            &self
                .children
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
        )?;
        state.serialize_field("branches", &self.branches)?;
        state.serialize_field("tags", &self.tags)?;
        state.serialize_field("branch_trace", &self.branch_trace)?;
        state.serialize_field("next_parent", &self.next_parent)?;
        state.serialize_field("point", &self.point)?;
        state.serialize_field("connections", &self.connections)?;
        state.end()
    }
}

// TODO (@day): maybe move this elsewhere?
pub struct Head {
    pub oid: Oid,
    pub name: String,
    pub is_branch: bool,
}

impl serde::Serialize for Head {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Head", 3)?;
        state.serialize_field("oid", &self.oid.to_string())?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("is_branch", &self.is_branch)?;
        state.end()
    }
}

impl Commit {
    pub fn new(commit: &git2::Commit) -> Result<Self, Error> {
        Ok(Commit {
            oid: commit.id(),
            is_merge: commit.parent_count() > 1,
            parents: [commit.parent_id(0).ok(), commit.parent_id(1).ok()],
            children: Vec::new(),
            branches: Vec::new(),
            tags: Vec::new(),
            branch_trace: None,
            next_parent: 0,
            point: None,
            connections: Vec::new(),
        })
    }
    pub fn get_next_parent(&mut self) -> Option<Oid> {
        if self.next_parent >= 2usize {
            None
        } else if let Some(parent_oid) = self.parents[self.next_parent] {
            Some(parent_oid)
        } else {
            None
        }
    }
    pub fn parent_processed(&mut self) -> () {
        self.next_parent += 1;
    }

    pub fn register_point(&mut self, point: Point) {
        self.point = Some(point);
    }

    pub fn add_connection(&mut self, connection: Point) {
        self.connections.push(connection);
    }
}

impl Head {
    /// Create an object representing the current head of the repository
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    /// - There is no name for the HEAD
    /// - There is no oid for the HEAD
    pub fn new(repo: &Repository) -> Result<Self, Error> {
        let head = repo.head()?;
        if let Some(name) = head.name() {
            let name = if name == "HEAD" {
                name.to_string()
            } else {
                name[11..].to_string()
            };

            if let Some(oid) = head.target() {
                Ok(Head {
                    oid,
                    name,
                    is_branch: head.is_branch(),
                })
            } else {
                Err(Error::GitError("no ID for HEAD".to_string()))
            }
        } else {
            Err(Error::GitError("No name for HEAD".to_string()))
        }
    }
}
