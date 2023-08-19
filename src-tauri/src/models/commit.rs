use git2::{Oid, Repository};

use crate::error::Error;

use super::branch::{Point, Vertex};

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
    next_parent: usize,
    point: Option<Point>,
    connections: Vec<Point>,
}

// TODO (@day): maybe move this elsewhere?
pub struct Head {
    pub oid: Oid,
    pub name: String,
    pub is_branch: bool,
}

impl Commit {
    pub fn new(commit: &git2::Commit) -> Result<Self, Error> {
        let next_parent = 0usize;
        todo!()
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
