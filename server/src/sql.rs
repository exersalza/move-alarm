use std::collections::HashMap;

use sqlite;

use std::fs;
use sqlite::State;
use tracing_subscriber::fmt::format;
use crate::Config;

pub struct Sql {
    conn: sqlite::Connection,
}

impl Sql {
    pub fn new() -> Self {
        let path = "./user.db";

        let conn = match fs::metadata(path) {
            Ok(_) => sqlite::open(path).unwrap(),
            Err(_) => create_user(path).unwrap()
        };

        Self { conn }
    }

    /// Get all users in an Vector with HashMaps
    pub fn get_all_users(&self) -> Vec<HashMap<String, String>> {
        let query = "select * from user";
        let mut statement = self.conn.prepare(query).unwrap();
        let mut ret: Vec<HashMap<String, String>> = Vec::new();

        while let Ok(sqlite::State::Row) = statement.next() {
            let user = statement.read::<String, _>("user").unwrap();
            let path = statement.read::<String, _>("media_path").unwrap();

            let mut map: HashMap<String, String> = HashMap::new();

            map.insert("path".to_string(), path);
            map.insert("name".to_string(), user);

            ret.push(map);
        }

        ret
    }


    /// Update an user
    pub fn update_user(&self, data: Config) {
        let query = format!("INSERT INTO user (user, media_path)
            VALUES ('{}', '{}')
            ON CONFLICT (user)
            DO UPDATE SET media_path = excluded.media_path;", data.username, data.path);

        self.conn.execute(query).expect("I let my threads panic for pleasure");
    }

    /// Delete a user by its name
    pub fn delete_user(&self, user: String) {
        self.conn.execute(format!("DELETE FROM user WHERE user = '{}';", user))
                .expect("TODO: panic message");
    }

    /// Get the path for an user
    pub fn get_path(&self, user: String) -> String {
        let mut path: String = String::new();
        let mut st = self.conn.prepare(format!("SELECT media_path FROM user WHERE user = '{user}'")).unwrap();

        while let Ok(State::Row) = st.next() {
            path = st.read::<String, _>("media_path").unwrap();
        }

        path
    }
}

/// Creates the user db if its not existing
fn create_user(path: &str) -> std::io::Result<sqlite::Connection> {
    fs::File::create(path)?;

    let conn = sqlite::open(path).unwrap();
    println!("creating db");

    let query = format!("
create table user
(
    user       TEXT
        constraint user_pk
            primary key,
    media_path text
);");
    conn.execute(query).unwrap();

    Ok(conn)
}