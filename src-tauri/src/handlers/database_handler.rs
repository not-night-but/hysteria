use rusqlite::{Connection, Params};

pub struct DbConnection {
    conn: Connection,
}

impl DbConnection {
    pub fn new(path: &str) -> Self {
        Self {
            conn: Connection::open(path).expect("Error opening database"),
        }
    }

    pub fn raw_query<P>(&self, query: &str, params: P)
    where
        P: Params,
    {
        self.conn.execute(query, params).unwrap();
    }

    pub fn insert<P>(&self, table: &str, columns: Vec<&str>, params: P)
    where
        P: Params,
    {
        let mut values = "(".to_owned();
        let mut cols = "(".to_owned();
        for i in 0..columns.len() {
            values = format!("{}${}, ", values, i + 1);
            cols = format!("{}{}, ", cols, columns[i]);
        }
        values = format!("{})", values.trim_end_matches(", "));
        cols = format!("{})", cols.trim_end_matches(", "));
        self.conn
            .execute(
                format!("INSERT INTO {} {} VALUES {}", table, cols, values).as_str(),
                params,
            )
            .expect("Error inserting into database");
    }
}
