use crate::{CTFdClient, PWNCollegeClient};

use rusqlite::{Connection, Result, params};
use std::collections::{HashMap, HashSet};

type UserId = i64;
type ChallengeId = i64;

pub struct DB {
    db_conn: Connection,
    user_db: HashMap<String, UserId>,
    challenge_db: HashMap<String, ChallengeId>,
    solves_relation: HashMap<UserId, HashSet<ChallengeId>>,
    flags: HashMap<String, String>,   // module, flag,
    modules: HashMap<String, String>, // challenge, module
}

impl DB {
    fn init_db(&self) -> Result<(), String> {
        self.db_conn
            .execute_batch("PRAGMA foreign_keys = ON;")
            .expect("Failed to establish foreign keys for sqlite");

        self.db_conn
            .execute(
                r#"CREATE TABLE IF NOT EXISTS challenge_categories (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    module_id VARCHAR(25),
                    flag VARCHAR(80)
                );"#,
                (),
            )
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;

        self.db_conn
            .execute(
                r#"CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username TEXT NOT NULL UNIQUE
                );"#,
                (),
            )
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;
        self.db_conn
            .execute(
                r#"CREATE TABLE IF NOT EXISTS challenges (
                    id INTEGER PRIMARY KEY,
                    challenge_name TEXT NOT NULL UNIQUE,
                    module TEXT NOT NULL
                );"#,
                (),
            )
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;
        self.db_conn
            .execute(
                r#"CREATE TABLE IF NOT EXISTS solves (
                    user_id INTEGER NOT NULL,
                    challenge_id INTEGER NOT NULL,
                    
                    PRIMARY KEY (user_id, challenge_id),
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
                    FOREIGN KEY (challenge_id) REFERENCES challenges(id) ON DELETE CASCADE
                );"#,
                (),
            )
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;

        Ok(())
    }

    /// Fill local database data structures using existing data in sqlite db
    fn fill_local_db(&mut self) -> Result<(), String> {
        // users
        let mut gather_users = self
            .db_conn
            .prepare("SELECT id, username FROM users;")
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;
        let usernames: Vec<(UserId, String)> = gather_users
            .query_map([], |row| {
                Ok((row.get::<_, UserId>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to format queried data: {e:?}").to_string())?;

        for user_data in usernames {
            self.user_db.insert(user_data.1.to_string(), user_data.0);
        }

        // challenges
        let mut gather_challenges = self
            .db_conn
            .prepare("SELECT id, challenge_name, module FROM challenges;")
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;
        let challenges: Vec<(ChallengeId, String, String)> = gather_challenges
            .query_map([], |row| {
                Ok((
                    row.get::<_, ChallengeId>(0)?, // id
                    row.get::<_, String>(1)?,      // challenge_name
                    row.get::<_, String>(2)?,      // module
                ))
            })
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to format queried data: {e:?}").to_string())?;

        for challenge_data in challenges {
            // both make sure the module exists and insert the inner data
            self.challenge_db
                .insert(challenge_data.1.to_string(), challenge_data.0);
            self.modules
                .insert(challenge_data.1.to_string(), challenge_data.2.to_string());
        }

        // solves
        let mut gather_solves = self
            .db_conn
            .prepare("SELECT user_id, challenge_id FROM solves;")
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;
        let solves: Vec<(UserId, ChallengeId)> = gather_solves
            .query_map([], |row| {
                Ok((
                    row.get::<_, UserId>(0)?, // user_id
                    row.get::<_, ChallengeId>(1)?,
                ))
            })
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to format queried data: {e:?}").to_string())?;

        for solve_data in solves {
            self.solves_relation
                .entry(solve_data.0)
                .or_insert_with(|| HashSet::new())
                .insert(solve_data.1);
        }

        // flags
        let mut gather_flags = self
            .db_conn
            .prepare("SELECT module_id, flag FROM challenge_categories;")
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;
        let flags: Vec<(String, String)> = gather_flags
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?, // module name
                    row.get::<_, String>(1)?, // flag
                ))
            })
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to format queried data: {e:?}").to_string())?;

        for flag in flags {
            self.flags.insert(flag.0.to_string(), flag.1.to_string());
        }

        Ok(())
    }

    pub fn new(db_name: &str) -> Result<Self, String> {
        let mut db: DB = Self {
            db_conn: match Connection::open(db_name) {
                Ok(conn) => conn,
                Err(e) => return Err(format!("Failed to open sqlite db file: {e:?}").to_string()),
            },
            user_db: HashMap::new(),
            challenge_db: HashMap::new(),
            solves_relation: HashMap::new(),
            flags: HashMap::new(),
            modules: HashMap::new(),
        };

        db.init_db()?;
        db.fill_local_db()?;

        Ok(db)
    }

    pub async fn insert_challenge(
        &mut self,
        id: i64,
        challenge: &str,
        module: &str,
    ) -> Result<(), String> {
        self.db_conn
            .execute(
                "INSERT INTO challenges (id, challenge_name, module) VALUES (?1, ?2, ?3);",
                (id, &challenge, &module),
            )
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;

        self.challenge_db.insert(challenge.to_string(), id);
        self.modules
            .insert(challenge.to_string(), module.to_string());

        Ok(())
    }

    pub async fn flag_exists(&self, module: &str) -> bool {
        self.flags.contains_key(module)
    }

    pub async fn insert_flag(&mut self, module: &str, flag: &str) -> Result<(), String> {
        self.db_conn
            .execute(
                "INSERT INTO challenge_categories (module_id, flag) VALUES (?1, ?2);",
                (&module, &flag),
            )
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;

        self.flags.insert(module.to_string(), flag.to_string());
        Ok(())
    }

    pub async fn user_exists(&self, username: &str) -> bool {
        self.user_db.contains_key(username)
    }

    async fn insert_user(&mut self, username: &str) -> Result<i64, String> {
        // add user via sql
        self.db_conn
            .execute("INSERT INTO users (username) VALUES (?1);", (&username,))
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;

        // gather id from new object
        let user_id: i64 = self
            .db_conn
            .query_row(
                "SELECT id FROM users WHERE username = ?1",
                [&username],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;

        // add user to objects
        self.user_db.insert(username.to_string(), user_id);
        Ok(user_id)
    }

    async fn insert_solve(
        &mut self,
        challenge_id: ChallengeId,
        user_id: UserId,
    ) -> Result<(), String> {
        self.db_conn
            .execute(
                "INSERT INTO solves (user_id, challenge_id) VALUES (?1, ?2)",
                (user_id, challenge_id),
            )
            .map_err(|e| format!("Failed to run query on db: {e:?}").to_string())?;
        self.solves_relation
            .entry(user_id)
            .or_insert_with(|| HashSet::new())
            .insert(challenge_id);

        Ok(())
    }

    pub async fn get_and_insert_new_users(
        &mut self,
        ctfd_client: &CTFdClient,
        pwn_college_client: &PWNCollegeClient,
    ) -> Result<(), String> {
        let api_users = ctfd_client.get_users().await.map_err(|e| format!("Failed to insert user: {e:?}").to_string())?;
        let previous_users: Vec<String> = self.user_db.iter().map(|x| x.0.clone()).collect();

        // quickest compare
        let set_previous_users: HashSet<_> = previous_users.iter().collect();
        let new_users: Vec<_> = api_users
            .iter()
            .filter(|user| !set_previous_users.contains(user))
            .collect();
        if new_users.len() < 1 {
            return Ok(());
        }

        // gather solves for new user
        let modules: Vec<_> = self.flags.iter().map(|x| x.0.clone()).collect();
        for user in new_users {
            // I have to insert the user first and gather the autoincremented id
            let user_id = self.insert_user(&user).await
                .map_err(|e| format!("Failed to insert user: {e:?}").to_string())?;
            for module in &modules {
                let all_solved_challenges = pwn_college_client
                    .get_solves_by_user_for_module("intro-to-cybersecurity", &module, &user)
                    .await
                    .map_err(|e| format!("Failed to get solved challenges for user: {e:?}").to_string())?;

                // go ahead and insert solves
                for challenge in all_solved_challenges {
                    let pretty_module = pwn_college_client.pretty_print_module(&challenge.challenge_id).await
                        .map_err(|e| format!("Couldn't get pretty name of challenge: {e:?}").to_string())?;
                    let challenge_id = match self.challenge_db.get(&pretty_module) {
                        Some(id) => id,
                        None => {
                            dbg!(&self.challenge_db);
                            todo!("I need to add the non-pretty challenge name to the db entry... Right now it's breaking because I can't access it via that value");
                            return Err(format!("Failed to get challenge: {}", &pretty_module).to_string());
                        }
                    };
                    self.insert_solve(user_id, *challenge_id).await
                        .map_err(|e| format!("Failed to insert solve: {e:?}").to_string())?;
                }
            }
        }

        Ok(())
    }
}
