use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::{params, Connection, Params};
use rusqlite_migration::Migrations;

use crate::{error::Error, models::app::Repo};

pub struct DbConnection {
    conn: Connection,
}

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).expect("Error getting migrations");
}

impl DbConnection {
    pub fn new(path: &str) -> Self {
        if let Ok(mut conn) = Connection::open(path) {
            if let Ok(()) = MIGRATIONS.validate() {
                println!(
                    "VERSION BEFORE MIGRATIONS: {:?}",
                    MIGRATIONS.current_version(&mut conn)
                );
                MIGRATIONS
                    .to_latest(&mut conn)
                    .expect("Error applying migrations");
                println!(
                    "VERSION AFTER MIGRATIONS: {:?}",
                    MIGRATIONS.current_version(&mut conn)
                );
            }
            Self { conn }
        } else {
            panic!("CANNOT ACCESS APP DATABASE");
        }
    }

    pub fn raw_query<P>(&self, query: &str, params: P)
    where
        P: Params,
    {
        self.conn.execute(query, params).unwrap();
    }

    pub fn add_repo(&self, repo: Repo) {
        let query =
            format!("INSERT INTO user_repo (local_path, url, image_path) VALUES ($1, $2, $3);");

        self.raw_query(
            query.as_str(),
            params![repo.local_path, repo.url, repo.image_path],
        );
    }

    pub fn get_user_repos(&self) -> Result<Vec<Repo>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, local_path, url, image_path, colour, abb FROM user_repo")?;

        let repo_iter = stmt.query_map([], |row| {
            Ok(Repo {
                id: row.get(0)?,
                local_path: row.get(1)?,
                url: row.get(2)?,
                image_path: row.get(3)?,
                colour: row.get(4)?,
                abb: row.get(5)?,
            })
        })?;

        // @DAY: this is dangerous
        let repos: Vec<Repo> = repo_iter.map(|repo| repo.unwrap()).collect();

        Ok(repos)
    }

    // pub fn insert<P>(&self, table: &str, columns: Vec<&str>, params: P)
    // where
    //     P: Params,
    // {
    //     let mut values = "".to_owned();
    //     let mut cols = "".to_owned();
    //     for i in 0..columns.len() {
    //         values = format!("{} ${}, ", values, i + 1);
    //         cols = format!("{}{}, ", cols, columns[i]);
    //     }
    //     self.conn
    //         .execute(
    //             format!(
    //                 "INSERT INTO {} ({}) VALUES ({})",
    //                 table,
    //                 cols.trim_end_matches(", "),
    //                 values.trim_end_matches(", ")
    //             )
    //             .as_str(),
    //             params,
    //         )
    //         .expect("Error inserting into database");
    // }
}
